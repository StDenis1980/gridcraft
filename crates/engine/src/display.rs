//! How cells look as text: number formats applied, for the grid, clipboard, CSV and agents.

use std::collections::HashMap;
use std::sync::Mutex;

use gridcraft_core::Region;
use gridcraft_core::{CellRef, RangeRef, Value};
use gridcraft_model::{Sheet, Workbook};
use gridcraft_numfmt::{Formatted, NumberFormat, format_value_full, format_value_in};

thread_local! {
    // Tinta Office fork: the names new workbooks and sheets get, without their number ("Book",
    // "Sheet"). The interface sets them from its language on its thread ("Книга", "Аркуш"); every
    // other thread (files, the API, tests) keeps Excel's English ones.
    static DEFAULT_NAMES: std::cell::RefCell<(String, String)> = std::cell::RefCell::new(("Book".into(), "Sheet".into()));
}

/// Names new workbooks `{book}N` and new sheets `{sheet}N` from now on, on this thread. Names that
/// are empty or not valid sheet names keep the English ones. Existing names never change.
pub fn set_default_names(book: &str, sheet: &str) {
    let book = if book.trim().is_empty() { "Book" } else { book };
    let empty = Workbook { sheets: vec![], ..Workbook::new() };
    let sheet = if empty.check_sheet_name(&format!("{sheet}1"), None).is_ok() { sheet } else { "Sheet" };
    let _ = DEFAULT_NAMES.try_with(|n| *n.borrow_mut() = (book.to_string(), sheet.to_string()));
}

/// The name a new workbook gets before its number ("Book", "Книга").
pub fn default_book_name() -> String {
    DEFAULT_NAMES.try_with(|n| n.borrow().0.clone()).unwrap_or_else(|_| "Book".into())
}

/// The name a new sheet gets before its number ("Sheet", "Аркуш").
pub fn default_sheet_name() -> String {
    DEFAULT_NAMES.try_with(|n| n.borrow().1.clone()).unwrap_or_else(|_| "Sheet".into())
}

/// A fresh sheet name `{sheet}N` not in use in `wb`.
pub fn next_sheet_name(wb: &Workbook) -> String {
    let base = default_sheet_name();
    (1..).map(|n| format!("{base}{n}")).find(|n| wb.sheet_index(n).is_none()).unwrap_or(base)
}

/// Shows values in `region` from now on, on this thread (cells, print and charts). Files,
/// formulas, CSV and the engine API stay en-US.
pub fn set_region(region: &'static Region) {
    gridcraft_numfmt::set_display_region(region);
}

/// The region values are shown in on this thread.
pub fn region() -> &'static Region {
    gridcraft_numfmt::display_region()
}

/// Runs `f` with values shown in `region` on this thread, then puts the region back (also when
/// `f` unwinds). Files and the engine API run in en-US whatever the interface shows.
pub fn in_region<R>(region: &'static Region, f: impl FnOnce() -> R) -> R {
    struct Restore(&'static Region);
    impl Drop for Restore {
        fn drop(&mut self) {
            set_region(self.0);
        }
    }
    let _restore = Restore(self::region());
    set_region(region);
    f()
}

/// Parsed number formats are cached by code.
pub(crate) fn parsed(code: &str) -> NumberFormat {
    static CACHE: std::sync::LazyLock<Mutex<HashMap<String, NumberFormat>>> = std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));
    // A code of more bytes than 4 × the character limit can't be within it: it shows as General
    // without being hashed or scanned again for every cell that shares it.
    if code.len() > 4 * gridcraft_numfmt::MAX_FORMAT_CODE_CHARS {
        return NumberFormat::parse("General");
    }
    let lock = || CACHE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(f) = lock().get(code) {
        return f.clone();
    }
    // Parse outside the lock, so a slow code never blocks the other threads' lookups.
    let f = NumberFormat::parse(code);
    let mut m = lock();
    if m.len() > 4096 {
        m.clear();
    }
    m.insert(code.to_string(), f.clone());
    f
}

/// The parsed format of `code`; one longer than the character limit is a General format that keeps
/// the code it was made from. The engine itself formats through `parsed`, which doesn't copy it.
pub fn number_format(code: &str) -> NumberFormat {
    if code.len() > 4 * gridcraft_numfmt::MAX_FORMAT_CODE_CHARS {
        return NumberFormat::parse(code);
    }
    parsed(code)
}

/// Formats a value with a format code, in the display [`region`].
pub fn format(v: &Value, code: &str, wb: &Workbook) -> Formatted {
    format_in(v, code, wb, region())
}

/// Formats a value with a format code in `region`.
pub fn format_in(v: &Value, code: &str, wb: &Workbook, region: &Region) -> Formatted {
    format_value_in(v, &parsed(code), wb.date_system, region)
}

/// Whether typing `t` back into a General cell would not give the text `t` again (it would parse
/// as a number, boolean, error, date or formula, or lose a leading apostrophe). Such text is shown
/// for editing with a leading `'`, as Excel does for text carrying a quote prefix, so that
/// committing the editor unchanged keeps the cell as text.
fn text_needs_quote_prefix(t: &str, sys: gridcraft_core::date::DateSystem) -> bool {
    if t.is_empty() {
        return false;
    }
    let mut chars = t.chars();
    let (first, second) = (chars.next(), chars.next());
    if matches!(first, Some('=' | '\'')) {
        return true;
    }
    if matches!(first, Some('+' | '-')) && second.is_some_and(|c| c.is_ascii_alphabetic() || c == '(') {
        return true;
    }
    // Text that the display region reads as a number or date (`1,5` in Spain) needs one too.
    if let Some(en) = gridcraft_core::parse::delocalize_input(t, region())
        && !matches!(gridcraft_core::parse::parse_input(&en, sys).value, Value::Text(_))
    {
        return true;
    }
    !matches!(gridcraft_core::parse::parse_input(t, sys).value, Value::Text(ref x) if x.as_ref() == t)
}

/// What the cell editor and the formula bar start from. Same as `Cell::input_text`, except that a
/// constant number in a date or time format is shown as a date (as the grid shows it), not as its
/// serial. When the formatted text would not read back as the same date (for example
/// `dddd, mmmm d`), an ISO `yyyy-mm-dd` (with `hh:mm:ss` for a time of day; just `hh:mm:ss`
/// for a time alone) is used instead, and if even that would not read back, the serial.
pub fn edit_text(wb: &Workbook, sheet: &Sheet, c: CellRef) -> String {
    let raw = sheet.input_text(c);
    let Some(cell) = sheet.cell(c) else { return raw };
    if let Value::Text(t) = &cell.value
        && cell.formula.is_none()
        && wb.styles.get(cell.style).num_fmt.as_str() != "@"
        && text_needs_quote_prefix(t, wb.date_system)
    {
        return format!("'{raw}");
    }
    let Value::Number(n) = &cell.value else { return raw };
    let code = wb.styles.get(cell.style).num_fmt.as_str();
    let region = region();
    if cell.formula.is_some() || !parsed(code).is_date() {
        // A constant number is typed with the region's decimal separator.
        return if cell.formula.is_none() { gridcraft_core::parse::localize_number(&raw, region) } else { raw };
    }
    let reads_back = |s: &str| {
        let en = gridcraft_core::parse::delocalize_input(s, region);
        matches!(gridcraft_core::parse::parse_input(en.as_deref().unwrap_or(s), wb.date_system).value, Value::Number(m) if (m - n).abs() < 1e-9)
    };
    let shown = format(&cell.value, code, wb).text;
    if reads_back(&shown) {
        return shown;
    }
    let Some(dt) = gridcraft_core::date::datetime_from_serial(wb.date_system, *n) else { return raw };
    let time = format!("{:02}:{:02}:{:02}", dt.hour, dt.minute, dt.second);
    let date = format!("{:04}-{:02}-{:02}", dt.year, dt.month, dt.day);
    let iso = if *n < 1.0 {
        time
    } else if n.fract() == 0.0 {
        date
    } else {
        format!("{date} {time}")
    };
    if reads_back(&iso) { iso } else { raw }
}

/// Whether `c` has a formula or Python code that must not be shown: its cell is formatted
/// Hidden on a protected sheet. A spilled cell follows its anchor.
pub fn formula_hidden(wb: &Workbook, sheet: &Sheet, c: CellRef) -> bool {
    if !sheet.is_protected() {
        return false;
    }
    let at = match sheet.cell(c) {
        Some(x) if x.formula.is_some() || x.py.is_some() => c,
        _ => match sheet.spill_ranges.iter().find(|(_, r)| r.contains(c)) {
            Some((anchor, _)) => *anchor,
            None => return false,
        },
    };
    wb.styles.get(sheet.style_id(at)).protection.hidden
}

/// The text a cell shows (General up to the 11-character cell width; the grid narrows General
/// further to fit the column).
pub fn cell_text(wb: &Workbook, sheet: &Sheet, c: CellRef) -> String {
    if sheet.show_formulas
        && let Some(f) = sheet.cell(c).and_then(|x| x.formula.as_ref())
        && !formula_hidden(wb, sheet, c)
    {
        return format!("={}", f.text);
    }
    shown_text(wb, sheet, c)
}

/// A cell's value with its number format applied, independent of column width: no `#####`
/// and no General narrowing (Save As CSV/Text writes this). Formulas give their result. A number
/// shown as General (also through a coloured or sectioned General, or a Text format) is written
/// as [`format_value_full`] writes it, never with an exponent: `&`'s 15 significant digits below
/// 1E+15 (`124412140710`, not `1.24412E+11`), the shortest round-trip digits from 1E+15 (every
/// integer below 1e16 exactly). Excel writes the cell display.
pub fn value_text(wb: &Workbook, sheet: &Sheet, c: CellRef) -> String {
    let style = wb.styles.get(sheet.style_id(c));
    with_fill_space(format_value_full(&sheet.value(c), &parsed(style.num_fmt.as_str()), wb.date_system))
}

/// A cell's value through its number format, General at the 11-character cell width.
fn shown_text(wb: &Workbook, sheet: &Sheet, c: CellRef) -> String {
    let style = wb.styles.get(sheet.style_id(c));
    value_text_as(wb, &sheet.value(c), style.num_fmt.as_str())
}

/// A value through a format code given apart from a cell, as a cell shows it (General at the
/// 11-character cell width). Print takes the code from the cell's effective look, which a
/// conditional format can override.
pub fn value_text_as(wb: &Workbook, v: &Value, code: &str) -> String {
    value_text_as_in(wb, v, code, region())
}

/// [`value_text_as`] in `region` (delimited files are always written en-US).
pub fn value_text_as_in(wb: &Workbook, v: &Value, code: &str, region: &Region) -> String {
    with_fill_space(format_in(v, code, wb, region))
}

/// The formatted text with a single space where a `*x` repeat fill would expand.
fn with_fill_space(f: Formatted) -> String {
    match f.fill {
        Some((_, pos)) => {
            let mut t = f.text;
            if pos <= t.len() && t.is_char_boundary(pos) {
                t.insert(pos, ' ');
            }
            t
        }
        None => f.text,
    }
}

/// Tab-separated text of a range, as the system clipboard gets it.
pub fn range_text(wb: &Workbook, sheet: usize, r: RangeRef) -> String {
    let Some(sh) = wb.sheet(sheet) else { return String::new() };
    let mut out = String::new();
    let r = clipboard_range(r);
    for row in r.start.row..=r.end.row {
        if sh.is_row_hidden(row) {
            continue;
        }
        let mut first = true;
        for col in r.start.col..=r.end.col {
            if !first {
                out.push('\t');
            }
            first = false;
            let t = cell_text(wb, sh, CellRef::new(row, col));
            // Like Excel, only text with a tab or line break is quoted (its quotes doubled);
            // `say "hi"` is copied as it is (#2225).
            if t.contains(['\t', '\n', '\r']) {
                out.push('"');
                out.push_str(&t.replace('"', "\"\""));
                out.push('"');
            } else {
                out.push_str(&t);
            }
        }
        out.push('\n');
    }
    out
}

/// The existing plain-text clipboard limit, shared with its HTML representation.
pub(crate) fn clipboard_range(r: RangeRef) -> RangeRef {
    if r.count() > 2_000_000 { RangeRef::new(r.start, r.start.offset_clamped(1999, 999)) } else { r }
}

/// Status-bar statistics for a selection: average, count (non-empty), numerical count, min,
/// max, sum.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize)]
pub struct Stats {
    pub count: u64,
    pub numeric: u64,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
    pub average: Option<f64>,
}

pub fn stats(sheet: &Sheet, ranges: &[RangeRef]) -> Stats {
    let mut s = Stats { min: f64::INFINITY, max: f64::NEG_INFINITY, ..Default::default() };
    let mut seen = std::collections::HashSet::new();
    for r in ranges {
        let r = match sheet.used_range().and_then(|u| r.intersection(&u)) {
            Some(x) => x,
            None => continue,
        };
        let mut visit = |c: CellRef, v: &Value| {
            if !seen.insert(c) || sheet.is_row_hidden(c.row) {
                return;
            }
            if v.is_empty() {
                return;
            }
            s.count += 1;
            if let Value::Number(n) = v {
                s.numeric += 1;
                s.sum += n;
                s.min = s.min.min(*n);
                s.max = s.max.max(*n);
            }
        };
        for (c, cell) in sheet.cells.iter_range(r) {
            visit(c, &cell.value);
        }
        for (c, v) in sheet.spill.range(r.start..=r.end) {
            if r.contains(*c) {
                visit(*c, v);
            }
        }
    }
    if s.numeric > 0 {
        s.average = Some(s.sum / s.numeric as f64);
    } else {
        s.min = 0.0;
        s.max = 0.0;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A format code far past Excel's 255 characters shows its values as General, for every cell
    /// that shares it, without being hashed into the cache each time.
    #[test]
    fn an_overlong_code_formats_as_general() {
        let wb = Workbook::new();
        let code = format!("0.00{}", "\"x\"".repeat(400_000));
        for _ in 0..1_000 {
            assert_eq!(format(&Value::Number(1.5), &code, &wb).text, "1.5");
        }
        assert_eq!(format(&Value::Number(1.5), "0.00", &wb).text, "1.50");
    }

    /// The public `number_format` keeps the code of an overlong format; it is General, but not the
    /// same format as a real `General`.
    #[test]
    fn number_format_keeps_an_overlong_code() {
        let code = format!("0.00{}", "\"x\"".repeat(2_000));
        let f = number_format(&code);
        assert!(f.is_general());
        assert_eq!(f.code(), code);
        assert_ne!(f, number_format("General"));
    }
}

#[cfg(test)]
mod tinta_default_names {
    use serde_json::json;

    /// Tinta Office fork: new workbooks and sheets take the interface's names (Excel's, in its
    /// language) on that thread only; names that exist never change, and other threads keep
    /// Excel's English ones.
    #[test]
    fn new_workbooks_and_sheets_take_the_interface_names() {
        std::thread::spawn(|| {
            let mut s = crate::Session::new();
            s.new_workbook();
            super::set_default_names("Книга", "Аркуш");
            s.new_workbook();
            s.new_workbook();
            let titles: Vec<String> = s.documents().iter().map(|d| d.title.clone()).collect();
            assert_eq!(titles, ["Book1", "Книга1", "Книга2"], "the earlier workbook keeps its name");
            let d = s.active().unwrap();
            assert_eq!(d.wb.sheets.iter().map(|sh| sh.name.as_str()).collect::<Vec<_>>(), ["Аркуш1"]);
            s.execute("home.insertSheet", json!({})).unwrap();
            let names: Vec<String> = s.active().unwrap().wb.sheets.iter().map(|sh| sh.name.clone()).collect();
            assert!(names.contains(&"Аркуш2".to_string()), "{names:?}");
            assert_eq!(s.documents()[0].wb.sheets[0].name, "Sheet1", "the first workbook's sheet is not renamed");
            // A name that can't be a sheet name keeps the English one.
            super::set_default_names("Книга", "Лист:");
            assert_eq!(super::default_sheet_name(), "Sheet");
        })
        .join()
        .unwrap();
        // Other threads (files, the API) keep Excel's English names.
        std::thread::spawn(|| {
            let mut s = crate::Session::new();
            s.new_workbook();
            assert_eq!(s.active().unwrap().title, "Book1");
            assert_eq!(s.active().unwrap().wb.sheets[0].name, "Sheet1");
        })
        .join()
        .unwrap();
    }
}

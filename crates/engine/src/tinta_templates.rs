//! Tinta Office templates for File › New: a self-employed worker's invoices, expenses, IVA and
//! IRPF (Spain), and a family budget in Spanish and Ukrainian. Original text, figures and colours,
//! built by replaying commands like the samples. The tax figures are a guide, not advice.

use gridcraft_model::Workbook;
use serde_json::{Value as Json, json};

use crate::{DocState, Session};

/// (id, title, interface languages it is offered in; empty = all).
pub const TEMPLATES: &[(&str, &str, &[&str])] = &[
    ("autonomo", "Autónomo: ingresos, IVA e IRPF", &[]),
    ("presupuesto-familiar", "Presupuesto familiar", &["en", "es", "ca", "gl", "eu"]),
    ("simeinyi-biudzhet", "Сімейний бюджет", &["uk"]),
];

/// The templates File › New offers for an interface language code.
pub fn for_language(code: &str) -> impl Iterator<Item = (&'static str, &'static str)> + '_ {
    TEMPLATES.iter().filter(move |(_, _, langs)| langs.is_empty() || langs.contains(&code)).map(|(id, title, _)| (*id, *title))
}

pub fn title(id: &str) -> Option<&'static str> {
    TEMPLATES.iter().find(|(n, ..)| *n == id).map(|(_, t, _)| *t)
}

/// Builds a template, or `None` for an unknown id (or if a step fails, which a test rules out).
pub fn build(id: &str) -> Option<Workbook> {
    let first_sheet = match id {
        "autonomo" => "Ingresos",
        "presupuesto-familiar" => "Presupuesto",
        "simeinyi-biudzhet" => "Бюджет",
        _ => return None,
    };
    let mut wb = Workbook::new();
    if let Some(sh) = wb.sheet_mut(0) {
        sh.name = first_sheet.into();
    }
    let mut s = Session::new();
    s.add_document(DocState::new(wb, None, title(id)?.to_string(), crate::Trust::Local));
    let ok = match id {
        "autonomo" => autonomo(&mut s),
        "presupuesto-familiar" => family(&mut s, &SPANISH_BUDGET),
        _ => family(&mut s, &UKRAINIAN_BUDGET),
    };
    if !ok {
        return None;
    }
    let mut wb = (*s.doc().ok()?.wb).clone();
    wb.set_active_sheet(0);
    for i in 0..wb.sheets.len() {
        if let Some(sh) = wb.sheet_mut(i) {
            sh.view_active = gridcraft_core::CellRef::default();
        }
    }
    Some(wb)
}

fn run(s: &mut Session, id: &str, p: Json) -> bool {
    match s.execute(id, p) {
        Ok(_) => true,
        Err(e) => {
            log::warn!("template: {id} failed: {e}");
            false
        }
    }
}

fn set(s: &mut Session, cell: &str, input: &str) -> bool {
    run(s, "cell.set", json!({"cell": cell, "input": input}))
}

const SPANISH_FOOTER: &str = "&CPágina &P de &N";
const EURO: &str = "#,##0.00 \"€\"";
/// Rows the invoice and expense formulas are filled down (about one a week); more are a fill-handle
/// drag away. Kept short because Excel and Calculo print rows whose formulas show nothing.
const ROWS: usize = 60;

/// Title, note and column headings shared by the template sheets.
fn heading(s: &mut Session, title: &str, note: &str, headers: Json, last_col: char) -> bool {
    let mut ok = run(s, "range.setValues", json!({"range": "B2", "values": [[title]]}));
    ok &= run(s, "home.cellStyle", json!({"range": "B2", "name": "Title"}));
    ok &= run(s, "range.setValues", json!({"range": "B3", "values": [[note]]}));
    ok &= run(s, "home.cellStyle", json!({"range": "B3", "name": "Explanatory Text"}));
    ok &= run(s, "range.setValues", json!({"range": "B5", "values": [headers]}));
    ok &= run(s, "home.cellStyle", json!({"range": format!("B5:{last_col}5"), "name": "Heading 3"}));
    ok &= run(s, "home.wrapText", json!({"range": format!("B5:{last_col}5"), "on": true}));
    ok &= run(s, "home.alignBottom", json!({"range": format!("B5:{last_col}5")}));
    ok &= run(s, "home.columnWidth", json!({"cols": "A:A", "width": 20}));
    ok
}

/// A4 landscape, one page wide, the heading row repeated and a page-number footer.
fn page_setup(s: &mut Session, footer: &str) -> bool {
    run(s, "home.autofitRowHeight", json!({"rows": "5:5"}))
        && run(
            s,
            "pageLayout.pageSetup",
            json!({"orientation": "landscape", "paper": "A4", "fitWidth": 1, "fitHeight": 0, "titleRows": "5:5", "footer": footer}),
        )
}

fn autonomo(s: &mut Session) -> bool {
    // Example invoices: date, number, client, base, IVA rate, IRPF withholding.
    let invoices = [
        ("1/20/2026", "2026-001", "Estudio de diseño (cliente de ejemplo)", 1200.0, 0.21, 0.15),
        ("2/14/2026", "2026-002", "Particular (sin retención)", 450.0, 0.21, 0.0),
        ("4/3/2026", "2026-003", "Academia (cliente de ejemplo)", 800.0, 0.21, 0.15),
        ("5/28/2026", "2026-004", "Clínica (cliente de ejemplo)", 2300.0, 0.21, 0.15),
        ("8/11/2026", "2026-005", "Asociación cultural", 615.5, 0.21, 0.15),
        ("10/2/2026", "2026-006", "Particular (IVA reducido)", 350.0, 0.10, 0.0),
    ];
    let expenses = [
        ("1/5/2026", "Seguridad Social", "Cuota de autónomos", 80.0, 0.0),
        ("1/10/2026", "Operadora", "Fibra y móvil", 45.45, 0.21),
        ("3/2/2026", "Tienda de informática", "Ordenador portátil", 999.0, 0.21),
        ("4/15/2026", "Gestoría", "Asesoría trimestral", 60.0, 0.21),
        ("7/1/2026", "Seguridad Social", "Cuota de autónomos", 80.0, 0.0),
        ("9/20/2026", "Librería", "Libros técnicos", 40.0, 0.04),
    ];
    let first = 6;

    // Ingresos: one row per invoice, B..K.
    let mut ok = heading(
        s,
        "Facturas emitidas 2026",
        "Datos de ejemplo: sustitúyalos por los suyos. Cuota IVA, Retención, Total y Trimestre se calculan solos hasta la fila 65.",
        json!(["Fecha", "Factura", "Cliente", "Base imponible", "Tipo IVA", "Cuota IVA", "Tipo IRPF", "Retención", "Total factura", "Trimestre"]),
        'K',
    );
    for (i, (date, number, client, base, iva, irpf)) in invoices.iter().enumerate() {
        let r = first + i;
        ok &= set(s, &format!("B{r}"), date);
        ok &= run(s, "range.setValues", json!({"range": format!("C{r}"), "values": [[number, client, base, iva]]}));
        ok &= run(s, "range.setValues", json!({"range": format!("H{r}"), "values": [[irpf]]}));
    }
    // Formulas run down every row, so a new invoice only needs its inputs typed.
    let quarter = |r: usize| format!("=IF(B{r}=\"\",\"\",ROUNDUP(MONTH(B{r})/3,0))");
    let cuota = |r: usize, rate: char| format!("=IF(E{r}=\"\",\"\",ROUND(E{r}*{rate}{r},2))");
    let rows: Vec<Json> = (first..first + ROWS).map(|r| json!([cuota(r, 'F')])).collect();
    ok &= run(s, "range.setValues", json!({"range": format!("G{first}"), "values": rows}));
    let rows: Vec<Json> = (first..first + ROWS).map(|r| json!([cuota(r, 'H'), format!("=IF(E{r}=\"\",\"\",E{r}+G{r}-I{r})"), quarter(r)])).collect();
    ok &= run(s, "range.setValues", json!({"range": format!("I{first}"), "values": rows}));
    let last = first + ROWS - 1;
    ok &= run(s, "home.numberFormat", json!({"range": format!("B{first}:B{last}"), "code": "dd/mm/yyyy"}));
    ok &= run(s, "home.numberFormat", json!({"range": format!("E{first}:E{last}"), "code": EURO}));
    ok &= run(s, "home.numberFormat", json!({"range": format!("G{first}:G{last}"), "code": EURO}));
    ok &= run(s, "home.numberFormat", json!({"range": format!("I{first}:J{last}"), "code": EURO}));
    ok &= run(s, "home.numberFormat", json!({"range": format!("F{first}:F{last}"), "code": "0%"}));
    ok &= run(s, "home.numberFormat", json!({"range": format!("H{first}:H{last}"), "code": "0%"}));
    ok &= run(s, "home.columnWidth", json!({"cols": "B:C", "width": 84}));
    ok &= run(s, "home.columnWidth", json!({"cols": "D:D", "width": 260}));
    ok &= run(s, "home.columnWidth", json!({"cols": "E:J", "width": 110}));
    ok &= run(s, "view.freezePanes", json!({"cell": format!("A{first}")}));
    ok &= page_setup(s, SPANISH_FOOTER);

    // Gastos: B..I.
    ok &= run(s, "home.insertSheet", json!({"name": "Gastos"}));
    ok &= run(s, "sheet.activate", json!({"sheet": "Gastos"}));
    ok &= heading(
        s,
        "Gastos deducibles 2026",
        "Anote solo gastos de la actividad con factura a su nombre. Las fórmulas llegan a la fila 65: para más, arrastre la última fila hacia abajo.",
        json!(["Fecha", "Proveedor", "Concepto", "Base imponible", "Tipo IVA", "IVA soportado", "Total", "Trimestre"]),
        'I',
    );
    for (i, (date, supplier, concept, base, iva)) in expenses.iter().enumerate() {
        let r = first + i;
        ok &= set(s, &format!("B{r}"), date);
        ok &= run(s, "range.setValues", json!({"range": format!("C{r}"), "values": [[supplier, concept, base, iva]]}));
    }
    let rows: Vec<Json> = (first..first + ROWS).map(|r| json!([cuota(r, 'F'), format!("=IF(E{r}=\"\",\"\",E{r}+G{r})"), quarter(r)])).collect();
    ok &= run(s, "range.setValues", json!({"range": format!("G{first}"), "values": rows}));
    ok &= run(s, "home.numberFormat", json!({"range": format!("B{first}:B{last}"), "code": "dd/mm/yyyy"}));
    ok &= run(s, "home.numberFormat", json!({"range": format!("E{first}:E{last}"), "code": EURO}));
    ok &= run(s, "home.numberFormat", json!({"range": format!("G{first}:H{last}"), "code": EURO}));
    ok &= run(s, "home.numberFormat", json!({"range": format!("F{first}:F{last}"), "code": "0%"}));
    ok &= run(s, "home.columnWidth", json!({"cols": "B:B", "width": 84}));
    ok &= run(s, "home.columnWidth", json!({"cols": "C:D", "width": 170}));
    ok &= run(s, "home.columnWidth", json!({"cols": "E:H", "width": 110}));
    ok &= run(s, "view.freezePanes", json!({"cell": format!("A{first}")}));
    ok &= page_setup(s, SPANISH_FOOTER);

    // Resumen: quarters on rows 6..9, total on 10.
    ok &= run(s, "home.insertSheet", json!({"name": "Resumen"}));
    ok &= run(s, "sheet.activate", json!({"sheet": "Resumen"}));
    ok &= heading(
        s,
        "Resumen trimestral: modelos 303 y 130",
        "Cifras orientativas en estimación directa simplificada; confírmelas con su asesor.",
        json!([
            "Trimestre",
            "Ingresos",
            "IVA repercutido",
            "IVA soportado",
            "Modelo 303",
            "Gastos",
            "Rendimiento acumulado",
            "Retenciones",
            "Modelo 130"
        ]),
        'J',
    );
    let inc = |col: char| format!("Ingresos!${col}${first}:${col}${last}");
    let exp = |col: char| format!("Gastos!${col}${first}:${col}${last}");
    for q in 1..=4 {
        let r = first + q - 1;
        ok &= set(s, &format!("B{r}"), &format!("{q}T"));
        let quarter = q.to_string();
        ok &= set(s, &format!("C{r}"), &format!("=SUMIFS({},{},{quarter})", inc('E'), inc('K')));
        ok &= set(s, &format!("D{r}"), &format!("=SUMIFS({},{},{quarter})", inc('G'), inc('K')));
        ok &= set(s, &format!("E{r}"), &format!("=SUMIFS({},{},{quarter})", exp('G'), exp('I')));
        ok &= set(s, &format!("F{r}"), &format!("=D{r}-E{r}"));
        ok &= set(s, &format!("G{r}"), &format!("=SUMIFS({},{},{quarter})", exp('E'), exp('I')));
        ok &= set(s, &format!("H{r}"), &format!("=SUM($C${first}:C{r})-SUM($G${first}:G{r})"));
        ok &= set(s, &format!("I{r}"), &format!("=SUMIFS({},{},{quarter})", inc('I'), inc('K')));
        // 20% of the year's net income so far, less withholdings and the earlier payments.
        ok &= set(s, &format!("J{r}"), &format!("=MAX(0,ROUND(0.2*H{r},2)-SUM($I${first}:I{r})-SUM($J${}:J{}))", first - 1, r - 1));
    }
    let total = first + 4;
    ok &= set(s, &format!("B{total}"), "Año");
    for c in ['C', 'D', 'E', 'F', 'G', 'I', 'J'] {
        ok &= set(s, &format!("{c}{total}"), &format!("=SUM({c}{first}:{c}{})", total - 1));
    }
    ok &= set(s, &format!("H{total}"), &format!("=H{}", total - 1));
    ok &= run(s, "home.numberFormat", json!({"range": format!("C{first}:J{total}"), "code": EURO}));
    ok &= run(s, "home.cellStyle", json!({"range": format!("B{total}:J{total}"), "name": "Total"}));
    ok &= run(s, "home.columnWidth", json!({"cols": "B:B", "width": 80}));
    ok &= run(s, "home.columnWidth", json!({"cols": "C:J", "width": 110}));
    ok &= page_setup(s, SPANISH_FOOTER);
    ok &= run(s, "selection.set", json!({"range": format!("B5:B{}", total - 1)}));
    ok &= run(s, "insert.chart", json!({"type": "column", "title": "IVA a ingresar por trimestre", "at": "B13", "width": 460, "height": 260}));
    let series =
        ("'Resumen'!$F$5".to_string(), format!("'Resumen'!$B${first}:$B${}", total - 1), format!("'Resumen'!$F${first}:$F${}", total - 1), "#2E6DA4");
    ok &= set_chart_series(s, "Resumen", &[series]);
    ok &= run(s, "sheet.activate", json!({"sheet": "Ingresos"}));
    ok &= run(s, "selection.set", json!({"cell": format!("B{first}")}));
    ok
}

/// Points the sheet's last chart at the given (name, categories, values, colour) series.
fn set_chart_series(s: &mut Session, sheet: &str, series: &[(String, String, String, &str)]) -> bool {
    let Ok(d) = s.doc_mut() else { return false };
    let wb = std::sync::Arc::make_mut(&mut d.wb);
    let Some(sh) = wb.sheet_index(sheet).and_then(|i| wb.sheet_mut(i)) else { return false };
    let Some(c) = sh.charts.last_mut() else { return false };
    c.series = series
        .iter()
        .map(|(name, categories, values, color)| gridcraft_model::Series {
            name: Some(name.clone()),
            categories: Some(categories.clone()),
            values: values.clone(),
            bubble_sizes: None,
            color: gridcraft_model::Color::from_hex(color),
            secondary: false,
            kind: None,
            quartiles: Default::default(),
            smooth: false,
            trendlines: Vec::new(),
        })
        .collect();
    true
}

struct Budget {
    title: &'static str,
    note: &'static str,
    /// Type, category, monthly heading, totals and status words.
    headers: [&'static str; 2],
    months: [&'static str; 12],
    total: &'static str,
    average: &'static str,
    income_kind: &'static str,
    expense_kind: &'static str,
    incomes: &'static str,
    expenses: &'static str,
    balance: &'static str,
    savings: &'static str,
    status: &'static str,
    over: &'static str,
    fine: &'static str,
    chart: &'static str,
    footer: &'static str,
    currency: &'static str,
    /// Example rows: (is income, category, monthly amount, follows the heating season).
    rows: &'static [(bool, &'static str, f64, bool)],
}

const SPANISH_BUDGET: Budget = Budget {
    title: "Presupuesto familiar 2026",
    note: "Importes de ejemplo: escriba los suyos en las columnas de los meses.",
    headers: ["Tipo", "Concepto"],
    months: ["Ene", "Feb", "Mar", "Abr", "May", "Jun", "Jul", "Ago", "Sep", "Oct", "Nov", "Dic"],
    total: "Total",
    average: "Media",
    income_kind: "Ingreso",
    expense_kind: "Gasto",
    incomes: "Ingresos",
    expenses: "Gastos",
    balance: "Ahorro",
    savings: "Tasa de ahorro",
    status: "Estado",
    over: "Hay meses en negativo",
    fine: "En positivo",
    chart: "Ingresos y gastos por mes",
    footer: SPANISH_FOOTER,
    currency: "#,##0 \"€\"",
    rows: &[
        (true, "Nómina", 2150.0, false),
        (true, "Otros ingresos", 120.0, false),
        (false, "Alquiler o hipoteca", 780.0, false),
        (false, "Luz, agua y gas", 135.0, true),
        (false, "Supermercado", 420.0, false),
        (false, "Transporte", 95.0, false),
        (false, "Colegio y actividades", 110.0, false),
        (false, "Salud", 40.0, false),
        (false, "Teléfono e internet", 48.0, false),
        (false, "Ocio", 90.0, false),
    ],
};

const UKRAINIAN_BUDGET: Budget = Budget {
    title: "Сімейний бюджет 2026",
    note: "Суми для прикладу: впишіть свої в колонки місяців.",
    headers: ["Тип", "Стаття"],
    months: ["Січ", "Лют", "Бер", "Кві", "Тра", "Чер", "Лип", "Сер", "Вер", "Жов", "Лис", "Гру"],
    total: "Разом",
    average: "Середнє",
    income_kind: "Дохід",
    expense_kind: "Витрата",
    incomes: "Доходи",
    expenses: "Витрати",
    balance: "Заощадження",
    savings: "Частка заощаджень",
    status: "Стан",
    over: "Є місяці в мінусі",
    fine: "Усе гаразд",
    chart: "Доходи й витрати за місяцями",
    footer: "&CСторінка &P з &N",
    currency: "#,##0 \"₴\"",
    rows: &[
        (true, "Зарплата", 42000.0, false),
        (true, "Підробіток", 3500.0, false),
        (false, "Оренда житла", 15000.0, false),
        (false, "Комунальні послуги", 3850.0, true),
        (false, "Продукти", 9800.0, false),
        (false, "Транспорт", 1200.0, false),
        (false, "Діти: гуртки", 2400.0, false),
        (false, "Здоров'я", 650.0, false),
        (false, "Зв'язок та інтернет", 450.0, false),
        (false, "Відпочинок", 1500.0, false),
    ],
};

/// Columns D..O are the months, P the total, Q the average.
const MONTH_COLS: [char; 12] = ['D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O'];

fn family(s: &mut Session, b: &Budget) -> bool {
    let mut headers = vec![b.headers[0], b.headers[1]];
    headers.extend(b.months);
    headers.extend([b.total, b.average]);
    let mut ok = heading(s, b.title, b.note, json!(headers), 'Q');
    let first = 6;
    // Bills follow the heating season, so the example months differ a little.
    const SEASON: [f64; 12] = [1.3, 1.25, 1.1, 0.9, 0.8, 0.75, 0.8, 0.85, 0.8, 0.95, 1.1, 1.3];
    for (i, (income, category, amount, seasonal)) in b.rows.iter().enumerate() {
        let r = first + i;
        let kind = if *income { b.income_kind } else { b.expense_kind };
        let mut row = vec![json!(kind), json!(category)];
        row.extend(SEASON.iter().map(|f| json!(if *seasonal { (amount * f).round() } else { *amount })));
        ok &= run(s, "range.setValues", json!({"range": format!("B{r}"), "values": [row]}));
        ok &= set(s, &format!("P{r}"), &format!("=SUM(D{r}:O{r})"));
        ok &= set(s, &format!("Q{r}"), &format!("=AVERAGE(D{r}:O{r})"));
    }
    let last = first + b.rows.len() - 1;
    let (inc, exp, bal) = (last + 2, last + 3, last + 4);
    for (r, label, kind) in [(inc, b.incomes, b.income_kind), (exp, b.expenses, b.expense_kind)] {
        ok &= set(s, &format!("C{r}"), label);
        for c in MONTH_COLS.iter().chain(&['P']) {
            ok &= set(s, &format!("{c}{r}"), &format!("=SUMIF($B${first}:$B${last},\"{kind}\",{c}${first}:{c}${last})"));
        }
    }
    ok &= set(s, &format!("C{bal}"), b.balance);
    for c in MONTH_COLS.iter().chain(&['P']) {
        ok &= set(s, &format!("{c}{bal}"), &format!("={c}{inc}-{c}{exp}"));
    }
    ok &= set(s, &format!("C{}", bal + 1), b.savings);
    ok &= set(s, &format!("P{}", bal + 1), &format!("=IF(P{inc}=0,0,P{bal}/P{inc})"));
    ok &= set(s, &format!("C{}", bal + 2), b.status);
    ok &= set(s, &format!("P{}", bal + 2), &format!("=IF(MIN(D{bal}:O{bal})<0,\"{}\",\"{}\")", b.over, b.fine));
    ok &= run(s, "home.numberFormat", json!({"range": format!("D{first}:Q{bal}"), "code": b.currency}));
    ok &= run(s, "home.numberFormat", json!({"range": format!("P{}", bal + 1), "code": "0.0%"}));
    ok &= run(s, "home.cellStyle", json!({"range": format!("C{inc}:P{exp}"), "name": "Heading 4"}));
    ok &= run(s, "home.cellStyle", json!({"range": format!("C{bal}:P{bal}"), "name": "Total"}));
    ok &= run(
        s,
        "home.conditionalFormat",
        json!({"range": format!("D{bal}:O{bal}"), "rule": {"type": "cellIs", "operator": "less", "value": 0, "preset": "lightRedFillDarkRedText"}}),
    );
    ok &= run(s, "home.columnWidth", json!({"cols": "B:B", "width": 70}));
    ok &= run(s, "home.columnWidth", json!({"cols": "C:C", "width": 170}));
    ok &= run(s, "home.columnWidth", json!({"cols": "D:O", "width": 84}));
    ok &= run(s, "home.columnWidth", json!({"cols": "P:Q", "width": 96}));
    ok &= run(s, "view.freezePanes", json!({"cell": format!("D{first}")}));
    ok &= page_setup(s, b.footer);
    ok &= run(s, "selection.set", json!({"range": format!("C{inc}:O{exp}")}));
    ok &= run(s, "insert.chart", json!({"type": "column", "title": b.chart, "at": format!("C{}", bal + 4), "width": 620, "height": 280}));
    let sheet = s.doc().ok().and_then(|d| d.wb.active().map(|sh| sh.name.clone())).unwrap_or_default();
    let months = format!("'{sheet}'!$D$5:$O$5");
    ok &= set_chart_series(
        s,
        &sheet,
        &[
            (format!("'{sheet}'!$C${inc}"), months.clone(), format!("'{sheet}'!$D${inc}:$O${inc}"), "#2E8B57"),
            (format!("'{sheet}'!$C${exp}"), months, format!("'{sheet}'!$D${exp}:$O${exp}"), "#C0504D"),
        ],
    );
    ok &= run(s, "selection.set", json!({"cell": format!("D{first}")}));
    ok
}

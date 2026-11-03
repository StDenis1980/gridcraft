//! Localized aliases accepted at user-entry boundaries; file syntax stays English.

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::czech::CZECH;
use crate::spanish::SPANISH;
use crate::{Expr, ParseError, parser};

/// Formula-entry syntax selected by the interface language. Languages without a function table
/// use the canonical English names.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FormulaLocale {
    #[default]
    En,
    /// Spanish function names, `;` between arguments, decimal comma, `\` between array columns.
    Es,
    /// English function names with the separators of a decimal-comma region (Ukrainian Excel):
    /// `;` between arguments, decimal comma, `\` between array columns.
    Semicolon,
    /// Czech function names (`SUMA`, `KDYŽ`), `;` between arguments, decimal comma, `\` between
    /// array columns.
    Cs,
}

/// A language's function names and the spelling of its two booleans.
struct Table {
    names: &'static [(&'static str, &'static str)],
    true_name: &'static str,
    false_name: &'static str,
}

const SPANISH_TABLE: Table = Table { names: SPANISH, true_name: "VERDADERO", false_name: "FALSO" };
const CZECH_TABLE: Table = Table { names: CZECH, true_name: "PRAVDA", false_name: "NEPRAVDA" };

/// English -> localized and localized -> English function-name maps, keyed upper-case.
type NameMaps = (HashMap<String, &'static str>, HashMap<String, &'static str>);

/// Upper-case English name -> localized name, and upper-case localized name -> English name.
/// Keys are upper-cased in full Unicode, so `año` finds `AÑO` and `když` finds `KDYŽ`.
fn build_maps(names: &'static [(&'static str, &'static str)]) -> NameMaps {
    let to_local = names.iter().map(|&(en, local)| (en.to_uppercase(), local)).collect();
    let to_en = names.iter().map(|&(en, local)| (local.to_uppercase(), en)).collect();
    (to_local, to_en)
}

/// Every (English, Spanish) function-name pair. Names Spanish Excel keeps in English are listed
/// with themselves.
pub fn spanish_function_names() -> &'static [(&'static str, &'static str)] {
    SPANISH
}

impl FormulaLocale {
    /// Every formula locale.
    pub const ALL: [FormulaLocale; 4] = [FormulaLocale::En, FormulaLocale::Es, FormulaLocale::Semicolon, FormulaLocale::Cs];

    fn table(self) -> Option<&'static Table> {
        match self {
            Self::En | Self::Semicolon => None,
            Self::Es => Some(&SPANISH_TABLE),
            Self::Cs => Some(&CZECH_TABLE),
        }
    }

    fn maps(self) -> Option<&'static NameMaps> {
        static ES: OnceLock<NameMaps> = OnceLock::new();
        static CS: OnceLock<NameMaps> = OnceLock::new();
        let cell = match self {
            Self::En | Self::Semicolon => return None,
            Self::Es => &ES,
            Self::Cs => &CS,
        };
        let names = self.table()?.names;
        Some(cell.get_or_init(|| build_maps(names)))
    }

    /// Whether this locale has function names of its own (and its own booleans).
    pub fn is_localized(self) -> bool {
        self.table().is_some()
    }

    /// Every (English, localized) function-name pair, names the language keeps in English listed
    /// with themselves; empty for the locales that use the English names.
    pub fn function_names(self) -> &'static [(&'static str, &'static str)] {
        self.table().map_or(&[], |t| t.names)
    }

    /// The spelling of TRUE and FALSE in formulas (`VERDADERO`, `FALSO` in Spanish).
    pub fn boolean_names(self) -> (&'static str, &'static str) {
        self.table().map_or(("TRUE", "FALSE"), |t| (t.true_name, t.false_name))
    }

    pub fn list_separator(self) -> char {
        if self == Self::En { ',' } else { ';' }
    }
    pub fn decimal_separator(self) -> char {
        if self == Self::En { '.' } else { ',' }
    }
    pub fn array_column_separator(self) -> char {
        if self == Self::En { ',' } else { '\\' }
    }
    pub fn function_name(self, canonical: &str) -> &str {
        self.maps().and_then(|m| m.0.get(&canonical.to_uppercase()).copied()).unwrap_or(canonical)
    }
    pub fn canonical_name(self, localized: &str) -> &str {
        self.maps().and_then(|m| m.1.get(&localized.to_uppercase()).copied()).unwrap_or(localized)
    }
}

/// Parses UI input using the selected function table and separators. Existing names and
/// LET/LAMBDA bindings take precedence over aliases. File and API syntax uses [`crate::parse`].
pub fn parse_input(src: &str, locale: FormulaLocale, known_name: impl Fn(&str) -> bool) -> Result<Expr, ParseError> {
    let mut expr = parser::parse_syntax(src, locale)?;
    if locale.is_localized() {
        normalize(&mut expr, locale, &known_name, &mut Vec::new(), 0, false)?;
    }
    Ok(expr)
}

/// Prints canonical expressions for editing. Name collisions retain the canonical spelling
/// so displaying then committing a formula never changes a defined-name reference.
pub fn print_input(expr: &Expr, locale: FormulaLocale, known_name: impl Fn(&str) -> bool) -> Result<String, ParseError> {
    // Check before cloning/printing. A flat operator chain such as A1+A2+… nests on its left
    // without nesting in the text: cloning, printing and `normalize` follow that left spine in
    // a loop, so it does not count (Excel accepts 4,096 operands, #916). Other nesting does.
    let mut pending = vec![(expr, 0usize)];
    while let Some((node, depth)) = pending.pop() {
        if depth > parser::MAX_DEPTH {
            return Err(ParseError { msg: "formula is nested too deeply".into(), pos: 0 });
        }
        match node {
            Expr::Unary(_, child) | Expr::Paren(child) => pending.push((child, depth + 1)),
            Expr::Binary(_, left, right) => {
                pending.push((left, depth));
                pending.push((right, depth + 1));
            }
            Expr::Call(_, args) => pending.extend(args.iter().map(|e| (e, depth + 1))),
            Expr::Invoke(callee, args) => {
                pending.push((callee, depth + 1));
                pending.extend(args.iter().map(|e| (e, depth + 1)));
            }
            Expr::Array(rows) => pending.extend(rows.iter().flatten().map(|e| (e, depth + 1))),
            _ => {}
        }
    }
    if locale == FormulaLocale::En {
        return Ok(crate::print(expr));
    }
    let canonical = if locale.is_localized() {
        let mut expr = expr.clone();
        normalize(&mut expr, locale, &known_name, &mut Vec::new(), 0, true)?;
        crate::print(&expr)
    } else {
        crate::print(expr)
    };
    let tokens = crate::lexer::tokenize(&canonical).map_err(|e| ParseError { msg: e.msg, pos: e.pos })?;
    let mut out = String::new();
    let mut array_depth = 0usize;
    for token in tokens {
        use crate::lexer::Tok;
        let text = canonical.get(token.start..token.end).unwrap_or("");
        match token.tok {
            Tok::LBrace => {
                array_depth += 1;
                out.push_str(text);
            }
            Tok::RBrace => {
                array_depth = array_depth.saturating_sub(1);
                out.push_str(text);
            }
            Tok::Number(_) => out.push_str(&text.replace('.', &locale.decimal_separator().to_string())),
            Tok::Comma => out.push(if array_depth > 0 { locale.array_column_separator() } else { locale.list_separator() }),
            _ => out.push_str(text),
        }
    }
    Ok(out)
}

/// The boolean a localized `TRUE`/`FALSE` spelling stands for (`VERDADERO` in Spanish).
pub(crate) fn boolean_alias(name: &str, locale: FormulaLocale) -> Option<bool> {
    let table = locale.table()?;
    if crate::same_name(name, table.true_name) {
        Some(true)
    } else if crate::same_name(name, table.false_name) {
        Some(false)
    } else {
        None
    }
}

/// The localized (`display`) or English spelling of a function name that no binding or defined
/// name shadows. Shown localized, a function whose localized name a binding shadows keeps its
/// English name; when that English name is the localized name of another function (`DECIMAL`,
/// which is FIXED in Spanish), no spelling would read back as the same function, so the
/// formula cannot be shown in that language.
fn translate_function(name: &str, es: FormulaLocale, display: bool, bound: &impl Fn(&str) -> bool) -> Result<String, ParseError> {
    if !display {
        return Ok(es.canonical_name(name).to_string());
    }
    let alias = es.function_name(name);
    if !bound(alias) {
        return Ok(alias.to_string());
    }
    if !crate::same_name(es.canonical_name(name), name) {
        return Err(ParseError { msg: format!("{name} cannot be shown in this language while {alias} names a local value"), pos: 0 });
    }
    Ok(name.to_string())
}

/// Whether argument `index` of `count` passed to the built-in `function` (English name) is a
/// function, which may be given by name: the LAMBDA of the helpers and the aggregation of
/// GROUPBY and PIVOTBY, as the evaluator reads them. Every other argument is a value, so a
/// name there stays a name (`SUMA(RAIZ;1)` keeps the name `RAIZ`).
fn takes_function(function: &str, index: usize, count: usize) -> bool {
    let position = match function.to_ascii_uppercase().as_str() {
        "MAP" => count.checked_sub(1),
        "BYROW" | "BYCOL" => Some(1),
        "REDUCE" | "SCAN" | "MAKEARRAY" | "GROUPBY" => Some(2),
        "PIVOTBY" => Some(3),
        _ => None,
    };
    position == Some(index)
}

/// A function given by name where a function is expected, as in `MAP({1;4};RAIZ)`, is
/// translated like a call. Booleans, bound names and names that are no function's are left alone.
fn function_argument(arg: &mut Expr, es: FormulaLocale, display: bool, bound: &impl Fn(&str) -> bool) -> Result<(), ParseError> {
    let Expr::Name(name) = arg else { return Ok(()) };
    if bound(name) || boolean_alias(name, es).is_some() {
        return Ok(());
    }
    let other = if display { es.function_name(name) } else { es.canonical_name(name) };
    if !crate::same_name(other, name) {
        *name = translate_function(name, es, display, bound)?;
    }
    Ok(())
}

fn normalize(
    expr: &mut Expr,
    locale: FormulaLocale,
    known_name: &impl Fn(&str) -> bool,
    bindings: &mut Vec<String>,
    depth: usize,
    display: bool,
) -> Result<(), ParseError> {
    if depth > parser::MAX_DEPTH {
        return Err(ParseError { msg: "formula is nested too deeply".into(), pos: 0 });
    }
    let bound = |name: &str, bindings: &[String]| bindings.iter().rev().any(|n| crate::same_name(n, name)) || known_name(name);
    match expr {
        Expr::Bool(value) if display => {
            let (yes, no) = locale.boolean_names();
            let alias = if *value { yes } else { no };
            if !bound(alias, bindings) {
                *expr = Expr::Name(alias.into());
            }
        }
        Expr::Name(name) if !display && !bound(name, bindings) => {
            if let Some(value) = boolean_alias(name, locale) {
                *expr = Expr::Bool(value);
            }
        }
        Expr::Call(name, args) if name == "LET" && args.len() >= 3 && !args.len().is_multiple_of(2) => {
            let saved = bindings.len();
            if let Some((body, pairs)) = args.split_last_mut() {
                for pair in pairs.chunks_mut(2) {
                    if let [declaration, value] = pair {
                        // The new name is not in scope in its own value expression.
                        normalize(value, locale, known_name, bindings, depth + 1, display)?;
                        if let Expr::Name(name) = declaration {
                            bindings.push(name.clone());
                        }
                    }
                }
                normalize(body, locale, known_name, bindings, depth + 1, display)?;
            }
            bindings.truncate(saved);
        }
        Expr::Call(name, args) if name == "LAMBDA" => {
            let saved = bindings.len();
            if let Some((body, params)) = args.split_last_mut() {
                for param in params {
                    if let Expr::Name(name) = param {
                        bindings.push(name.clone());
                    }
                }
                normalize(body, locale, known_name, bindings, depth + 1, display)?;
            }
            bindings.truncate(saved);
        }
        Expr::Call(name, args) => {
            let mut canonical = name.clone();
            if !bound(name, bindings) {
                *name = translate_function(name, locale, display, &|n| bound(n, bindings))?;
                if !display {
                    canonical.clone_from(name);
                }
            }
            let count = args.len();
            for (i, arg) in args.iter_mut().enumerate() {
                if takes_function(&canonical, i, count) {
                    function_argument(arg, locale, display, &|n| bound(n, bindings))?;
                }
                normalize(arg, locale, known_name, bindings, depth + 1, display)?;
            }
        }
        Expr::Invoke(callee, args) => {
            normalize(callee, locale, known_name, bindings, depth + 1, display)?;
            for arg in args {
                normalize(arg, locale, known_name, bindings, depth + 1, display)?;
            }
        }
        Expr::Unary(_, child) | Expr::Paren(child) => normalize(child, locale, known_name, bindings, depth + 1, display)?,
        Expr::Binary(..) => {
            // Follow the left spine of a flat chain (A1+A2+…) in a loop: it is not nesting.
            // Each call leaves `bindings` as it found them, so the order does not matter.
            let mut node = expr;
            while let Expr::Binary(_, left, right) = node {
                normalize(right, locale, known_name, bindings, depth + 1, display)?;
                node = left;
            }
            normalize(node, locale, known_name, bindings, depth, display)?;
        }
        Expr::Array(rows) if display => {
            for row in rows {
                for child in row {
                    normalize(child, locale, known_name, bindings, depth + 1, display)?;
                }
            }
        }
        // Array elements have already been parsed as literals. Sheet/structured references
        // contain user-defined labels and must not be translated.
        _ => {}
    }
    Ok(())
}

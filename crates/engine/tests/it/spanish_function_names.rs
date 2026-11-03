//! Localized formula entry (Spanish, Czech) knows every built-in function (Tinta Calculo).

use gridcraft_engine::cmd::formulas::function_list;
use gridcraft_engine::display::cell_text;
use gridcraft_engine::formula::{FormulaLocale, spanish_function_names};
use serde_json::json;

#[test]
fn every_listed_function_has_a_spanish_name() {
    let missing: Vec<String> = function_list()
        .iter()
        .filter_map(|f| f["name"].as_str())
        .filter(|name| !spanish_function_names().iter().any(|(en, _)| en == name))
        .map(str::to_string)
        .collect();
    assert!(missing.is_empty(), "functions without a Spanish name: {missing:?}");
    // Every row names a function the app has, so the table holds no stale names.
    let listed: Vec<String> = function_list().iter().filter_map(|f| f["name"].as_str().map(str::to_string)).collect();
    let extra: Vec<&str> = spanish_function_names().iter().map(|p| p.0).filter(|en| !listed.iter().any(|n| n == en)).collect();
    assert!(extra.is_empty(), "Spanish names for unknown functions: {extra:?}");
}

#[test]
fn spanish_formulas_compute_like_english_ones() {
    use gridcraft_engine::formula::{parse_input, print};
    let mut s = gridcraft_engine::Session::new();
    s.new_workbook();
    let cases = [
        ("SUMAR.SI.CONJUNTO({10\\20\\30};{1\\2\\1};1)", "40"),
        ("AÑO(FECHA(2026;11;3))", "2026"),
        ("REDONDEAR(PROMEDIO(1;2;4);2)", "2.33"),
        ("SI.ERROR(1/0;\"sin datos\")", "sin datos"),
        ("MAYUSC(IZQUIERDA(\"tinta\";3))", "TIN"),
        ("DIAS.LAB(FECHA(2026;11;2);FECHA(2026;11;6))", "5"),
    ];
    for (row, (spanish, expected)) in cases.iter().enumerate() {
        let expr = parse_input(spanish, FormulaLocale::Es, |_| false).unwrap();
        s.execute("cell.set", json!({"cell": format!("A{}", row + 1), "input": format!("={}", print(&expr))})).unwrap();
        let d = s.doc().unwrap();
        let shown = cell_text(&d.wb, d.wb.active().unwrap(), gridcraft_engine::core::CellRef::new(row as u32, 0));
        assert_eq!(shown, *expected, "{spanish}");
    }
}

#[test]
fn every_localized_table_covers_exactly_the_listed_functions() {
    let listed: Vec<String> = function_list().iter().filter_map(|f| f["name"].as_str().map(str::to_string)).collect();
    for locale in FormulaLocale::ALL.into_iter().filter(|l| l.is_localized()) {
        let names = locale.function_names();
        let missing: Vec<&String> = listed.iter().filter(|n| !names.iter().any(|(en, _)| en == n)).collect();
        let extra: Vec<&str> = names.iter().map(|p| p.0).filter(|en| !listed.iter().any(|n| n == en)).collect();
        assert!(missing.is_empty() && extra.is_empty(), "{locale:?}: missing {missing:?}, unknown {extra:?}");
    }
}

#[test]
fn czech_formulas_compute_like_english_ones() {
    use gridcraft_engine::formula::{parse_input, print};
    let mut s = gridcraft_engine::Session::new();
    s.new_workbook();
    let cases = [
        ("SUMIFS({10\\20\\30};{1\\2\\1};1)", "40"),
        ("ROK(DATUM(2026;11;3))", "2026"),
        ("ZAOKROUHLIT(PRŮMĚR(1;2;4);2)", "2.33"),
        ("IFERROR(1/0;\"bez dat\")", "bez dat"),
        ("VELKÁ(ZLEVA(\"tinta\";3))", "TIN"),
        ("KDYŽ(A(PRAVDA;NE(NEPRAVDA));LOG(1000);0)", "3"),
    ];
    for (row, (czech, expected)) in cases.iter().enumerate() {
        let expr = parse_input(czech, FormulaLocale::Cs, |_| false).unwrap();
        s.execute("cell.set", json!({"cell": format!("A{}", row + 1), "input": format!("={}", print(&expr))})).unwrap();
        let d = s.doc().unwrap();
        let shown = cell_text(&d.wb, d.wb.active().unwrap(), gridcraft_engine::core::CellRef::new(row as u32, 0));
        assert_eq!(shown, *expected, "{czech}");
    }
}

#[test]
fn german_formulas_compute_like_english_ones() {
    use gridcraft_engine::formula::{parse_input, print};
    let mut s = gridcraft_engine::Session::new();
    s.new_workbook();
    let cases = [
        ("SUMMEWENNS({10\\20\\30};{1\\2\\1};1)", "40"),
        ("JAHR(DATUM(2026;11;3))", "2026"),
        ("RUNDEN(MITTELWERT(1;2;4);2)", "2.33"),
        ("WENNFEHLER(1/0;\"keine Daten\")", "keine Daten"),
        ("GROSS(LINKS(\"tinta\";3))", "TIN"),
        ("WENN(UND(WAHR;NICHT(FALSCH));LÄNGE(\"abc\");0)", "3"),
    ];
    for (row, (german, expected)) in cases.iter().enumerate() {
        let expr = parse_input(german, FormulaLocale::De, |_| false).unwrap();
        s.execute("cell.set", json!({"cell": format!("A{}", row + 1), "input": format!("={}", print(&expr))})).unwrap();
        let d = s.doc().unwrap();
        let shown = cell_text(&d.wb, d.wb.active().unwrap(), gridcraft_engine::core::CellRef::new(row as u32, 0));
        assert_eq!(shown, *expected, "{german}");
    }
}

#[test]
fn french_formulas_compute_like_english_ones() {
    use gridcraft_engine::formula::{parse_input, print};
    let mut s = gridcraft_engine::Session::new();
    s.new_workbook();
    let cases = [
        ("SOMME.SI.ENS({10\\20\\30};{1\\2\\1};1)", "40"),
        ("ANNEE(DATE(2026;11;3))", "2026"),
        ("ARRONDI(MOYENNE(1;2;4);2)", "2.33"),
        ("SIERREUR(1/0;\"aucune donnée\")", "aucune donnée"),
        ("MAJUSCULE(GAUCHE(\"tinta\";3))", "TIN"),
        ("SI(ET(VRAI;NON(FAUX));NBCAR(\"abc\");0)", "3"),
    ];
    for (row, (local, expected)) in cases.iter().enumerate() {
        let expr = parse_input(local, FormulaLocale::Fr, |_| false).unwrap();
        s.execute("cell.set", json!({"cell": format!("A{}", row + 1), "input": format!("={}", print(&expr))})).unwrap();
        let d = s.doc().unwrap();
        let shown = cell_text(&d.wb, d.wb.active().unwrap(), gridcraft_engine::core::CellRef::new(row as u32, 0));
        assert_eq!(shown, *expected, "{local}");
    }
}

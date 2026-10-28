//! The Tinta templates in File › New build, calculate without errors, take new rows without
//! further setup and survive an .xlsx round trip.

use gridcraft_engine::Session;
use gridcraft_engine::core::{CellRef, Value};
use gridcraft_engine::tinta_templates::{TEMPLATES, for_language};
use serde_json::json;

fn open(id: &str) -> Session {
    let mut s = Session::new();
    s.execute("file.new", json!({"sample": id})).unwrap_or_else(|e| panic!("{id}: {e}"));
    s
}

fn value(s: &Session, sheet: &str, a1: &str) -> Value {
    let d = s.doc().unwrap();
    d.wb.sheet(d.wb.sheet_index(sheet).unwrap()).unwrap().value(CellRef::parse(a1).unwrap())
}

fn number(s: &Session, sheet: &str, a1: &str) -> f64 {
    match value(s, sheet, a1) {
        Value::Number(n) => n,
        other => panic!("{sheet}!{a1} is {other:?}"),
    }
}

fn close(actual: f64, expected: f64, what: &str) {
    assert!((actual - expected).abs() < 1e-9, "{what}: {actual} != {expected}");
}

fn no_errors(s: &Session, id: &str) {
    let d = s.doc().unwrap();
    for sh in &d.wb.sheets {
        for (c, _) in sh.cells.iter() {
            assert!(!matches!(sh.value(c), Value::Error(_)), "{id}: {}!{} is {:?}", sh.name, c.a1(), sh.value(c));
        }
    }
}

fn round_trip(s: &mut Session, name: &str) {
    let saved = s.execute("file.saveBytes", json!({"format": "xlsx"})).unwrap();
    s.execute("file.open", json!({"name": name, "base64": saved["base64"]})).unwrap();
    s.execute("formulas.calculateNow", json!({})).unwrap();
}

#[test]
fn every_template_builds_without_errors_and_has_a_chart() {
    for (id, title, _) in TEMPLATES {
        let s = open(id);
        no_errors(&s, id);
        let d = s.doc().unwrap();
        assert_eq!(d.title, *title);
        assert!(d.wb.sheets.iter().any(|sh| !sh.charts.is_empty()), "{id} has a chart");
    }
}

#[test]
fn templates_are_offered_in_their_languages() {
    let ids = |code| for_language(code).map(|(id, _)| id).collect::<Vec<_>>();
    assert_eq!(ids("es"), ["autonomo", "presupuesto-familiar"]);
    assert_eq!(ids("uk"), ["autonomo", "simeinyi-biudzhet"]);
    assert_eq!(ids("en"), ["autonomo", "presupuesto-familiar"]);
    assert!(Session::new().execute("file.new", json!({"sample": "no-such-template"})).is_err());
}

#[test]
fn self_employed_template_sums_quarters_and_takes_a_new_invoice() {
    let mut s = open("autonomo");
    // Example data: Q1 income 1,650 €, output IVA 346.50 €, input IVA 9.54 + 209.79 €.
    close(number(&s, "Resumen", "C6"), 1650.0, "1T ingresos");
    close(number(&s, "Resumen", "D6"), 346.5, "1T IVA repercutido");
    close(number(&s, "Resumen", "E6"), 9.54 + 209.79, "1T IVA soportado");
    close(number(&s, "Resumen", "F10"), number(&s, "Resumen", "D10") - number(&s, "Resumen", "E10"), "303 del año");
    let paid_before = number(&s, "Resumen", "J10");
    assert!(paid_before > 0.0, "the example year owes some Modelo 130");
    assert_eq!(value(&s, "Ingresos", "G20"), Value::text(""), "empty rows stay blank");

    round_trip(&mut s, "autonomo.xlsx");
    no_errors(&s, "autonomo after reopening");
    close(number(&s, "Resumen", "C6"), 1650.0, "1T ingresos after reopening");

    // A new invoice typed in the first empty row reaches the summary with no formula to copy.
    s.execute("sheet.activate", json!({"sheet": "Ingresos"})).unwrap();
    s.execute("range.setValues", json!({"range": "B12", "values": [["11/20/2026", "2026-007", "Nuevo cliente", 1000, 0.21]]})).unwrap();
    s.execute("cell.set", json!({"cell": "H12", "input": "0.15"})).unwrap();
    close(number(&s, "Ingresos", "G12"), 210.0, "cuota IVA");
    close(number(&s, "Ingresos", "J12"), 1060.0, "total factura");
    close(number(&s, "Ingresos", "K12"), 4.0, "trimestre");
    close(number(&s, "Resumen", "C9"), 1350.0, "4T ingresos");
    assert!(number(&s, "Resumen", "J10") > paid_before);
}

#[test]
fn family_budgets_add_up_in_both_languages() {
    for (id, sheet, status) in [("presupuesto-familiar", "Presupuesto", "En positivo"), ("simeinyi-biudzhet", "Бюджет", "Усе гаразд")]
    {
        let mut s = open(id);
        // Ten example rows on 6..15: incomes, expenses and balance on 17..19.
        let income = number(&s, sheet, "P17");
        let spent = number(&s, sheet, "P18");
        close(number(&s, sheet, "P19"), income - spent, &format!("{id} balance"));
        close(number(&s, sheet, "D17") * 12.0, income, &format!("{id} yearly income"));
        close(number(&s, sheet, "P20"), (income - spent) / income, &format!("{id} savings rate"));
        assert_eq!(value(&s, sheet, "P21"), Value::text(status));

        round_trip(&mut s, &format!("{id}.xlsx"));
        assert_eq!(s.doc().unwrap().wb.sheets[0].name, sheet);
        close(number(&s, sheet, "P19"), income - spent, &format!("{id} balance after reopening"));
        // An overspent month flips the status.
        s.execute("cell.set", json!({"cell": "F8", "input": "999999"})).unwrap();
        assert!(number(&s, sheet, "F19") < 0.0);
        assert_ne!(value(&s, sheet, "P21"), Value::text(status));
    }
}

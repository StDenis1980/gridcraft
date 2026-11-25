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
        // A single invoice is a printed page; every other template charts its figures.
        if *id != "factura" {
            assert!(d.wb.sheets.iter().any(|sh| !sh.charts.is_empty()), "{id} has a chart");
        }
    }
}

#[test]
fn templates_are_offered_in_their_languages() {
    let ids = |code| for_language(code).map(|(id, _)| id).collect::<Vec<_>>();
    let self_employed = ["autonomo", "factura", "libro-registro", "iva-trimestral"];
    assert_eq!(ids("es"), [&self_employed[..], &["presupuesto-familiar"]].concat());
    assert_eq!(ids("uk"), [&self_employed[..], &["simeinyi-biudzhet"]].concat());
    assert_eq!(ids("en"), [&self_employed[..], &["presupuesto-familiar"]].concat());
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

#[test]
fn invoice_totals_follow_its_lines_and_withholding() {
    let mut s = open("factura");
    // Example lines: 600 € and 8 × 45 € at 21 %, 15 % withholding.
    close(number(&s, "Factura", "G33"), 960.0, "base imponible");
    close(number(&s, "Factura", "G34"), 201.6, "IVA 21 %");
    close(number(&s, "Factura", "G38"), 960.0 + 201.6 - 144.0, "total");
    assert_eq!(value(&s, "Factura", "F24"), Value::text(""), "empty lines stay blank");

    // A reduced-rate line and a private client (no withholding).
    s.execute("range.setValues", json!({"range": "B24", "values": [["Taller", 2, 50, 0.1]]})).unwrap();
    s.execute("cell.set", json!({"cell": "E37", "input": "0"})).unwrap();
    close(number(&s, "Factura", "G35"), 10.0, "IVA 10 %");
    close(number(&s, "Factura", "G38"), 1060.0 + 211.6, "total for a private client");

    round_trip(&mut s, "factura.xlsx");
    no_errors(&s, "factura after reopening");
    close(number(&s, "Factura", "G38"), 1060.0 + 211.6, "total after reopening");
}

#[test]
fn books_sum_by_quarter_and_kind() {
    let mut s = open("libro-registro");
    // Example data: Q1 income 1,200 + 450 €; Q1 expenses 87.61 + 45.45 + 189 €.
    close(number(&s, "Resumen", "C6"), 1650.0, "1T ingresos");
    close(number(&s, "Resumen", "D6"), 87.61 + 45.45 + 189.0, "1T gastos");
    let before = number(&s, "Resumen", "E12");
    close(before, number(&s, "Resumen", "E10") - number(&s, "Amortizaciones", "G6"), "rendimiento neto");
    assert!(number(&s, "Amortizaciones", "G6") < 999.0 * 0.26, "a laptop bought in March is depreciated for part of the year");

    // IVA that can't be deducted becomes part of the expense.
    s.execute("sheet.activate", json!({"sheet": "Gastos"})).unwrap();
    let deductible = number(&s, "Gastos", "L8");
    s.execute("cell.set", json!({"cell": "K8", "input": "No"})).unwrap();
    assert!(number(&s, "Gastos", "L8") > deductible);
    close(number(&s, "Resumen", "C22"), number(&s, "Gastos", "L8"), "material de oficina by kind");

    round_trip(&mut s, "libro.xlsx");
    no_errors(&s, "libro after reopening");
    // A new invoice typed in the first empty row reaches its quarter with no formula to copy.
    s.execute("sheet.activate", json!({"sheet": "Ingresos"})).unwrap();
    let q4 = number(&s, "Resumen", "C9");
    s.execute("cell.set", json!({"cell": "B11", "input": "11/20/2026"})).unwrap();
    s.execute("range.setValues", json!({"range": "H11", "values": [[1000, 0.21]]})).unwrap();
    close(number(&s, "Ingresos", "M11"), 1210.0, "total with no withholding");
    close(number(&s, "Resumen", "C9"), q4 + 1000.0, "4T ingresos");
}

#[test]
fn quarterly_vat_follows_the_chosen_quarter() {
    let mut s = open("iva-trimestral");
    let sheet = "Modelo 303";
    // Q1: 1,650 € at 21 % and 300 € at 10 % issued; 45.45 + 189 € at 21 % received.
    close(number(&s, sheet, "D9"), 1650.0, "base general 1T");
    close(number(&s, sheet, "F8"), 30.0, "cuota reducida 1T");
    close(number(&s, sheet, "F10"), 346.5 + 30.0, "casilla 27");
    close(number(&s, sheet, "F15"), 9.54 + 39.69, "casilla 45");
    close(number(&s, sheet, "F17"), 376.5 - 49.23, "casilla 46");
    assert_eq!(value(&s, sheet, "B20"), Value::text("A ingresar"));

    // The second quarter has an investment good, so more is deducted than charged.
    s.execute("sheet.activate", json!({"sheet": sheet})).unwrap();
    s.execute("cell.set", json!({"cell": "C4", "input": "2"})).unwrap();
    close(number(&s, sheet, "F14"), 735.0, "casilla 31");
    assert!(number(&s, sheet, "F19") < 0.0);
    assert_ne!(value(&s, sheet, "B20"), Value::text("A ingresar"));
    close(number(&s, sheet, "E25"), number(&s, sheet, "F17"), "the quarter table agrees with the return");

    // Credit carried over from earlier quarters lowers what is paid, never below zero.
    s.execute("cell.set", json!({"cell": "C4", "input": "3"})).unwrap();
    let due = number(&s, sheet, "F19");
    assert!(due > 0.0);
    s.execute("cell.set", json!({"cell": "F18", "input": "100000"})).unwrap();
    close(number(&s, sheet, "F19"), 0.0, "fully compensated");

    round_trip(&mut s, "iva.xlsx");
    no_errors(&s, "iva after reopening");
    close(number(&s, sheet, "F17"), due, "casilla 46 after reopening");
}

//! Workbooks a Spanish self-employed worker and a Ukrainian family keep, written from scratch:
//! invoices and expenses with IVA and IRPF, the quarterly Modelo 303 and 130 figures, and a
//! family budget in hryvnias. Each is checked against sums worked out here, then saved as .xlsx,
//! opened again and recalculated after an edit.

use gridcraft_engine::Session;
use gridcraft_engine::core::{CellRef, Region, Value};
use gridcraft_engine::display::{cell_text, set_region};
use serde_json::json;

/// (date, invoice, client, base, IVA rate, IRPF withholding rate)
const INVOICES: [(&str, &str, &str, f64, f64, f64); 6] = [
    ("1/20/2026", "2026-001", "Diseño Gráfico Levante SL", 1200.0, 0.21, 0.15),
    ("2/14/2026", "2026-002", "Panadería Núñez", 450.0, 0.21, 0.0),
    ("4/3/2026", "2026-003", "Academia Ñandú", 800.0, 0.21, 0.15),
    ("5/28/2026", "2026-004", "Clínica Camp de Morvedre SL", 2300.0, 0.21, 0.15),
    ("8/11/2026", "2026-005", "Asociació Cultural l'Horta", 615.5, 0.21, 0.15),
    ("10/2/2026", "2026-006", "Familia Ruiz", 350.0, 0.10, 0.0),
];

/// (date, supplier, concept, base, IVA rate)
const EXPENSES: [(&str, &str, &str, f64, f64); 6] = [
    ("1/5/2026", "Seguridad Social", "Cuota de autónomos", 80.0, 0.0),
    ("1/10/2026", "Operadora", "Fibra y móvil", 45.45, 0.21),
    ("3/2/2026", "Tienda de informática", "Portátil", 999.0, 0.21),
    ("4/15/2026", "Gestoría", "Asesoría trimestral", 60.0, 0.21),
    ("7/1/2026", "Seguridad Social", "Cuota de autónomos", 80.0, 0.0),
    ("9/20/2026", "Librería", "Libros técnicos", 40.0, 0.04),
];

fn cents(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

fn quarter(date: &str) -> usize {
    let month: usize = date.split('/').next().and_then(|m| m.parse().ok()).unwrap();
    month.div_ceil(3)
}

fn set(s: &mut Session, cell: &str, input: impl Into<String>) {
    let input = input.into();
    s.execute("cell.set", json!({"cell": cell, "input": input})).unwrap_or_else(|e| panic!("{cell} = {input}: {e}"));
}

fn number(s: &Session, sheet: &str, a1: &str) -> f64 {
    let d = s.doc().unwrap();
    let sh = d.wb.sheet(d.wb.sheet_index(sheet).unwrap()).unwrap();
    match sh.value(CellRef::parse(a1).unwrap()) {
        Value::Number(n) => n,
        other => panic!("{sheet}!{a1} is {other:?}"),
    }
}

fn text(s: &Session, sheet: &str, a1: &str) -> String {
    let d = s.doc().unwrap();
    let sh = d.wb.sheet(d.wb.sheet_index(sheet).unwrap()).unwrap();
    match sh.value(CellRef::parse(a1).unwrap()) {
        Value::Text(t) => t.to_string(),
        other => panic!("{sheet}!{a1} is {other:?}"),
    }
}

fn shown(s: &Session, sheet: &str, a1: &str) -> String {
    let d = s.doc().unwrap();
    let sh = d.wb.sheet(d.wb.sheet_index(sheet).unwrap()).unwrap();
    cell_text(&d.wb, sh, CellRef::parse(a1).unwrap())
}

fn close(actual: f64, expected: f64, what: &str) {
    assert!((actual - expected).abs() < 1e-9, "{what}: {actual} != {expected}");
}

fn reopen(s: &mut Session, name: &str) {
    let saved = s.execute("file.saveBytes", json!({"format": "xlsx"})).unwrap();
    s.execute("file.open", json!({"name": name, "base64": saved["base64"]})).unwrap();
    s.execute("formulas.calculateNow", json!({})).unwrap();
}

/// Builds the self-employed workbook: Ingresos, Gastos and Trimestres.
fn autonomo() -> Session {
    let mut s = Session::new();
    s.new_workbook();
    s.execute("sheet.rename", json!({"sheet": 0, "name": "Ingresos"})).unwrap();
    s.execute(
        "range.setValues",
        json!({"range": "A1", "values": [["Fecha", "Factura", "Cliente", "Base imponible", "IVA %", "Cuota IVA", "IRPF %", "Retención", "Total factura", "Trimestre"]]}),
    )
    .unwrap();
    for (i, (date, number, client, base, iva, irpf)) in INVOICES.iter().enumerate() {
        let r = i + 2;
        set(&mut s, &format!("A{r}"), *date);
        s.execute(
            "range.setValues",
            json!({"range": format!("B{r}"), "values": [[number, client, base, iva, format!("=ROUND(D{r}*E{r},2)"), irpf]]}),
        )
        .unwrap();
        set(&mut s, &format!("H{r}"), format!("=ROUND(D{r}*G{r},2)"));
        set(&mut s, &format!("I{r}"), format!("=D{r}+F{r}-H{r}"));
        set(&mut s, &format!("J{r}"), format!("=ROUNDUP(MONTH(A{r})/3,0)"));
    }
    s.execute("home.numberFormat", json!({"range": "A2:A7", "code": "dd/mm/yyyy"})).unwrap();
    s.execute("home.numberFormat", json!({"range": "D2:D7", "code": "#,##0.00 \"€\""})).unwrap();
    s.execute("home.numberFormat", json!({"range": "F2:F7", "code": "#,##0.00 \"€\""})).unwrap();
    s.execute("home.numberFormat", json!({"range": "H2:I7", "code": "#,##0.00 \"€\""})).unwrap();
    s.execute("home.numberFormat", json!({"range": "E2:E7", "code": "0%"})).unwrap();
    s.execute("home.numberFormat", json!({"range": "G2:G7", "code": "0%"})).unwrap();

    s.execute("home.insertSheet", json!({"name": "Gastos"})).unwrap();
    s.execute("sheet.activate", json!({"sheet": "Gastos"})).unwrap();
    s.execute(
        "range.setValues",
        json!({"range": "A1", "values": [["Fecha", "Proveedor", "Concepto", "Base imponible", "IVA %", "IVA soportado", "Total", "Trimestre"]]}),
    )
    .unwrap();
    for (i, (date, supplier, concept, base, iva)) in EXPENSES.iter().enumerate() {
        let r = i + 2;
        set(&mut s, &format!("A{r}"), *date);
        s.execute("range.setValues", json!({"range": format!("B{r}"), "values": [[supplier, concept, base, iva]]})).unwrap();
        set(&mut s, &format!("F{r}"), format!("=ROUND(D{r}*E{r},2)"));
        set(&mut s, &format!("G{r}"), format!("=D{r}+F{r}"));
        set(&mut s, &format!("H{r}"), format!("=ROUNDUP(MONTH(A{r})/3,0)"));
    }

    s.execute("home.insertSheet", json!({"name": "Trimestres"})).unwrap();
    s.execute("sheet.activate", json!({"sheet": "Trimestres"})).unwrap();
    s.execute(
        "range.setValues",
        json!({"range": "A1", "values": [["Trimestre", "Ingresos", "IVA repercutido", "IVA soportado", "Modelo 303", "Gastos", "Rendimiento acumulado", "Retenciones", "Modelo 130"]]}),
    )
    .unwrap();
    for q in 1..=4 {
        let r = q + 1;
        set(&mut s, &format!("A{r}"), q.to_string());
        set(&mut s, &format!("B{r}"), format!("=SUMIFS(Ingresos!$D$2:$D$50,Ingresos!$J$2:$J$50,A{r})"));
        set(&mut s, &format!("C{r}"), format!("=SUMIFS(Ingresos!$F$2:$F$50,Ingresos!$J$2:$J$50,A{r})"));
        set(&mut s, &format!("D{r}"), format!("=SUMIFS(Gastos!$F$2:$F$50,Gastos!$H$2:$H$50,A{r})"));
        set(&mut s, &format!("E{r}"), format!("=C{r}-D{r}"));
        set(&mut s, &format!("F{r}"), format!("=SUMIFS(Gastos!$D$2:$D$50,Gastos!$H$2:$H$50,A{r})"));
        set(&mut s, &format!("G{r}"), format!("=SUM($B$2:B{r})-SUM($F$2:F{r})"));
        set(&mut s, &format!("H{r}"), format!("=SUMIFS(Ingresos!$H$2:$H$50,Ingresos!$J$2:$J$50,A{r})"));
        // Modelo 130: 20% of the year's net income so far, less withholdings and earlier payments.
        set(&mut s, &format!("I{r}"), format!("=MAX(0,ROUND(0.2*G{r},2)-SUM($H$2:H{r})-SUM($I$1:I{}))", r - 1));
    }
    set(&mut s, "A6", "Total");
    for c in ["B", "C", "D", "E", "F", "H", "I"] {
        set(&mut s, &format!("{c}6"), format!("=SUM({c}2:{c}5)"));
    }
    set(&mut s, "A8", "Estado");
    set(&mut s, "B8", "=IF(E6>0,\"A ingresar\",\"A compensar\")");
    s.execute("home.numberFormat", json!({"range": "B2:I6", "code": "#,##0.00 \"€\""})).unwrap();
    s
}

struct Quarter {
    income: f64,
    iva_out: f64,
    iva_in: f64,
    expenses: f64,
    withheld: f64,
}

fn quarters() -> [Quarter; 4] {
    let mut q: [Quarter; 4] = std::array::from_fn(|_| Quarter { income: 0.0, iva_out: 0.0, iva_in: 0.0, expenses: 0.0, withheld: 0.0 });
    for (date, _, _, base, iva, irpf) in INVOICES {
        let t = &mut q[quarter(date) - 1];
        t.income += base;
        t.iva_out += cents(base * iva);
        t.withheld += cents(base * irpf);
    }
    for (date, _, _, base, iva) in EXPENSES {
        let t = &mut q[quarter(date) - 1];
        t.expenses += base;
        t.iva_in += cents(base * iva);
    }
    q
}

fn check_autonomo(s: &Session) {
    let q = quarters();
    let (mut income, mut expenses, mut withheld, mut paid, mut result_303) = (0.0, 0.0, 0.0, 0.0, 0.0);
    for (i, t) in q.iter().enumerate() {
        let r = i + 2;
        close(number(s, "Trimestres", &format!("B{r}")), t.income, &format!("ingresos T{}", i + 1));
        close(number(s, "Trimestres", &format!("C{r}")), t.iva_out, &format!("IVA repercutido T{}", i + 1));
        close(number(s, "Trimestres", &format!("D{r}")), t.iva_in, &format!("IVA soportado T{}", i + 1));
        close(number(s, "Trimestres", &format!("E{r}")), t.iva_out - t.iva_in, &format!("303 T{}", i + 1));
        income += t.income;
        expenses += t.expenses;
        withheld += t.withheld;
        result_303 += t.iva_out - t.iva_in;
        let due = (cents(0.2 * (income - expenses)) - withheld - paid).max(0.0);
        close(number(s, "Trimestres", &format!("I{r}")), due, &format!("130 T{}", i + 1));
        paid += due;
    }
    close(number(s, "Trimestres", "B6"), income, "ingresos del año");
    close(number(s, "Trimestres", "E6"), result_303, "303 del año");
    close(number(s, "Trimestres", "I6"), paid, "130 del año");
    assert_eq!(text(s, "Trimestres", "B8"), if result_303 > 0.0 { "A ingresar" } else { "A compensar" });
    for (i, (_, _, client, base, iva, irpf)) in INVOICES.iter().enumerate() {
        let r = i + 2;
        assert_eq!(text(s, "Ingresos", &format!("C{r}")), *client);
        close(number(s, "Ingresos", &format!("I{r}")), base + cents(base * iva) - cents(base * irpf), &format!("factura {r}"));
    }
}

#[test]
fn self_employed_iva_and_irpf_survive_xlsx_and_recalculate() {
    let mut s = autonomo();
    check_autonomo(&s);
    // Spot-check the hand arithmetic itself: Q1 is 1,650 € of income and 257.54 € of input IVA.
    close(number(&s, "Trimestres", "B2"), 1650.0, "T1 ingresos");
    close(number(&s, "Trimestres", "D2"), 9.54 + 209.79, "T1 IVA soportado");

    reopen(&mut s, "autonomo-2026.xlsx");
    let d = s.doc().unwrap();
    let names: Vec<&str> = d.wb.sheets.iter().map(|sh| sh.name.as_str()).collect();
    assert_eq!(names, ["Ingresos", "Gastos", "Trimestres"]);
    let sh = d.wb.sheet(d.wb.sheet_index("Trimestres").unwrap()).unwrap();
    assert_eq!(sh.input_text(CellRef::parse("I3").unwrap()), "=MAX(0,ROUND(0.2*G3,2)-SUM($H$2:H3)-SUM($I$1:I2))");
    check_autonomo(&s);

    // Editing an invoice after reopening flows through to every quarter and the year.
    s.execute("sheet.activate", json!({"sheet": "Ingresos"})).unwrap();
    let before = number(&s, "Trimestres", "I6");
    set(&mut s, "D2", "2000");
    close(number(&s, "Ingresos", "F2"), 420.0, "cuota IVA editada");
    close(number(&s, "Trimestres", "B2"), 2450.0, "T1 tras editar");
    assert!(number(&s, "Trimestres", "I6") > before, "the 130 payments grow with the income");

    // The region shows Spanish formats; the stored values stay the same.
    set_region(Region::EsEs);
    assert_eq!(shown(&s, "Ingresos", "D2"), "2.000,00 €");
    assert_eq!(shown(&s, "Ingresos", "A2"), "20/01/2026");
    assert_eq!(shown(&s, "Ingresos", "E2"), "21%");
    set_region(Region::EnUs);
    assert_eq!(shown(&s, "Ingresos", "D2"), "2,000.00 €");
}

/// (type, category, January, February, March)
const FAMILY: [(&str, &str, f64, f64, f64); 9] = [
    ("Дохід", "Зарплата", 42000.0, 42000.0, 44500.0),
    ("Дохід", "Підробіток", 3500.0, 0.0, 5200.0),
    ("Витрата", "Оренда житла", 15000.0, 15000.0, 15000.0),
    ("Витрата", "Комунальні послуги", 3850.4, 4120.75, 2980.1),
    ("Витрата", "Продукти", 9800.0, 10450.5, 9930.0),
    ("Витрата", "Транспорт", 1200.0, 1350.0, 1180.0),
    ("Витрата", "Діти: гуртки", 2400.0, 2400.0, 3100.0),
    ("Витрата", "Здоров'я", 650.0, 2890.0, 0.0),
    ("Витрата", "Зв'язок та інтернет", 450.0, 450.0, 450.0),
];

fn family_budget() -> Session {
    let mut s = Session::new();
    s.new_workbook();
    s.execute("sheet.rename", json!({"sheet": 0, "name": "Бюджет"})).unwrap();
    s.execute("range.setValues", json!({"range": "A1", "values": [["Тип", "Категорія", "Січень", "Лютий", "Березень", "Разом", "Середнє"]]}))
        .unwrap();
    for (i, (kind, category, jan, feb, mar)) in FAMILY.iter().enumerate() {
        let r = i + 2;
        s.execute("range.setValues", json!({"range": format!("A{r}"), "values": [[kind, category, jan, feb, mar]]})).unwrap();
        set(&mut s, &format!("F{r}"), format!("=SUM(C{r}:E{r})"));
        set(&mut s, &format!("G{r}"), format!("=AVERAGE(C{r}:E{r})"));
    }
    let last = FAMILY.len() + 1;
    let rows = [("Доходи", "Дохід"), ("Витрати", "Витрата")];
    for (i, (label, kind)) in rows.iter().enumerate() {
        let r = last + 2 + i;
        set(&mut s, &format!("B{r}"), *label);
        for c in ["C", "D", "E", "F"] {
            set(&mut s, &format!("{c}{r}"), format!("=SUMIF($A$2:$A${last},\"{kind}\",{c}$2:{c}${last})"));
        }
    }
    let (inc, exp) = (last + 2, last + 3);
    let bal = last + 4;
    set(&mut s, &format!("B{bal}"), "Залишок");
    for c in ["C", "D", "E", "F"] {
        set(&mut s, &format!("{c}{bal}"), format!("={c}{inc}-{c}{exp}"));
    }
    set(&mut s, &format!("B{}", bal + 1), "Частка заощаджень");
    set(&mut s, &format!("F{}", bal + 1), format!("=IF(F{inc}=0,0,F{bal}/F{inc})"));
    set(&mut s, &format!("B{}", bal + 2), "Стан");
    set(&mut s, &format!("F{}", bal + 2), format!("=IF(MIN(C{bal}:E{bal})<0,\"Перевитрата\",\"Гаразд\")"));
    s.execute("home.numberFormat", json!({"range": format!("C2:G{bal}"), "code": "#,##0.00 \"₴\""})).unwrap();
    s.execute("home.numberFormat", json!({"range": format!("F{}", bal + 1), "code": "0.0%"})).unwrap();
    s
}

fn check_family(s: &Session) {
    let last = FAMILY.len() + 1;
    let (inc, exp, bal) = (last + 2, last + 3, last + 4);
    let month = |m: usize, kind: &str| -> f64 { FAMILY.iter().filter(|f| f.0 == kind).map(|f| [f.2, f.3, f.4][m]).sum() };
    let mut min_balance = f64::MAX;
    for (m, c) in ["C", "D", "E"].iter().enumerate() {
        close(number(s, "Бюджет", &format!("{c}{inc}")), month(m, "Дохід"), &format!("доходи {c}"));
        close(number(s, "Бюджет", &format!("{c}{exp}")), month(m, "Витрата"), &format!("витрати {c}"));
        let balance = month(m, "Дохід") - month(m, "Витрата");
        close(number(s, "Бюджет", &format!("{c}{bal}")), balance, &format!("залишок {c}"));
        min_balance = min_balance.min(balance);
    }
    let income: f64 = (0..3).map(|m| month(m, "Дохід")).sum();
    let spent: f64 = (0..3).map(|m| month(m, "Витрата")).sum();
    close(number(s, "Бюджет", &format!("F{bal}")), income - spent, "залишок за квартал");
    close(number(s, "Бюджет", &format!("F{}", bal + 1)), (income - spent) / income, "частка заощаджень");
    assert_eq!(text(s, "Бюджет", &format!("F{}", bal + 2)), if min_balance < 0.0 { "Перевитрата" } else { "Гаразд" });
    assert_eq!(text(s, "Бюджет", "B9"), "Здоров'я");
}

#[test]
fn ukrainian_family_budget_survives_xlsx_and_recalculates() {
    let mut s = family_budget();
    check_family(&s);
    reopen(&mut s, "сімейний-бюджет.xlsx");
    assert_eq!(s.doc().unwrap().wb.sheets[0].name, "Бюджет");
    check_family(&s);

    // A March overspend flips the status after reopening.
    let last = FAMILY.len() + 1;
    let bal = last + 4;
    set(&mut s, "E4", "60000");
    assert!(number(&s, "Бюджет", &format!("E{bal}")) < 0.0);
    assert_eq!(text(&s, "Бюджет", &format!("F{}", bal + 2)), "Перевитрата");

    set_region(Region::UkUa);
    assert_eq!(shown(&s, "Бюджет", "C2"), "42\u{a0}000,00 ₴");
    assert_eq!(shown(&s, "Бюджет", "C5"), "3\u{a0}850,40 ₴");
    set_region(Region::EnUs);
    assert_eq!(shown(&s, "Бюджет", "C5"), "3,850.40 ₴");
}

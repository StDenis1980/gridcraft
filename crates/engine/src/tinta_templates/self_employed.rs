//! The self-employed worker's separate templates: an invoice, the income and expense books, and the
//! quarterly IVA worksheet for the modelo 303. Spanish text, example data; a guide, not advice.

use serde_json::json;

use super::{EURO, ROWS, SPANISH_FOOTER, Session, heading, page_setup, run, set, set_chart_series};

/// The fill of the cells meant to be typed in on the worksheets that are not printed for others.
const INPUT: &str = "#DDEBF7";
const FIRST: usize = 6;
const LAST: usize = FIRST + ROWS - 1;
const YES_NO: &str = "\"Sí,No\"";

fn input(s: &mut Session, range: &str) -> bool {
    run(s, "home.fillColor", json!({"range": range, "color": INPUT}))
}

fn note(s: &mut Session, cell: &str, text: &str) -> bool {
    set(s, cell, text) && run(s, "home.cellStyle", json!({"range": cell, "name": "Explanatory Text"}))
}

fn num(s: &mut Session, range: &str, code: &str) -> bool {
    run(s, "home.numberFormat", json!({"range": range, "code": code}))
}

fn width(s: &mut Session, cols: &str, w: u32) -> bool {
    run(s, "home.columnWidth", json!({"cols": cols, "width": w}))
}

fn list(s: &mut Session, range: &str, items: &str) -> bool {
    run(s, "data.validation", json!({"range": range, "type": "list", "formula1": items}))
}

/// `=IF(<col><r>="","",<formula>)` down the register rows, so a row stays blank until it is used.
fn down(s: &mut Session, col: char, blank_unless: char, formula: impl Fn(usize) -> String) -> bool {
    let rows: Vec<_> = (FIRST..=LAST).map(|r| json!([format!("=IF({blank_unless}{r}=\"\",\"\",{})", formula(r))])).collect();
    run(s, "range.setValues", json!({"range": format!("{col}{FIRST}"), "values": rows}))
}

fn quarter_col(s: &mut Session, col: char, date: char) -> bool {
    down(s, col, date, |r| format!("ROUNDUP(MONTH({date}{r})/3,0)"))
}

fn dates(s: &mut Session, col: char) -> bool {
    num(s, &format!("{col}{FIRST}:{col}{LAST}"), "dd/mm/yyyy")
}

/// Freezes the headings and sets the page up. The register columns are not filled: a filled
/// block prints over every empty row.
fn finish_register(s: &mut Session) -> bool {
    run(s, "view.freezePanes", json!({"cell": format!("A{FIRST}")})) && page_setup(s, SPANISH_FOOTER)
}

fn new_sheet(s: &mut Session, name: &str) -> bool {
    run(s, "home.insertSheet", json!({"name": name})) && run(s, "sheet.activate", json!({"sheet": name}))
}

/// A full invoice (factura completa) with the details the invoicing rules ask for.
pub(super) fn invoice(s: &mut Session) -> bool {
    let mut ok = set(s, "B2", "Factura");
    ok &= run(s, "home.cellStyle", json!({"range": "B2", "name": "Title"}));
    ok &= note(s, "B3", "Datos de ejemplo: escriba los suyos encima. Importes, IVA, retención y total se calculan solos.");
    let parties = [
        (5, "Emisor", ["Ana García López (ejemplo)", "00000000T", "Calle Ejemplo 1, 3.º B", "28001 Madrid"]),
        (11, "Cliente", ["Cliente de Ejemplo, S.L.", "B00000000", "Avenida Modelo 25", "46001 Valencia"]),
    ];
    for (r, title, values) in parties {
        ok &= set(s, &format!("B{r}"), title);
        ok &= run(s, "home.cellStyle", json!({"range": format!("B{r}:G{r}"), "name": "Heading 4"}));
        for (i, (label, value)) in ["Nombre o razón social", "NIF", "Domicilio", "Código postal y población"].iter().zip(values).enumerate() {
            let row = r + 1 + i;
            ok &= set(s, &format!("B{row}"), label);
            ok &= set(s, &format!("C{row}"), value);
            ok &= run(s, "home.mergeCells", json!({"range": format!("C{row}:G{row}")}));
        }
    }
    ok &= set(s, "B17", "Número de factura");
    ok &= set(s, "C17", "2026-001");
    ok &= set(s, "B18", "Fecha de expedición");
    ok &= set(s, "C18", "1/20/2026");
    ok &= set(s, "B19", "Fecha de la operación (si es otra)");
    ok &= num(s, "C18:C19", "dd/mm/yyyy");
    ok &= run(s, "home.alignLeft", json!({"range": "C17:C19"}));

    // Lines 22..31: concept, quantity, unit price, IVA rate; amount and IVA are formulas.
    let (first, last) = (22, 31);
    ok &= run(
        s,
        "range.setValues",
        json!({"range": "B21", "values": [["Concepto", "Cantidad", "Precio unitario", "Tipo IVA", "Importe", "Cuota IVA"]]}),
    );
    ok &= run(s, "home.cellStyle", json!({"range": "B21:G21", "name": "Heading 3"}));
    ok &= run(s, "home.wrapText", json!({"range": "B21:G21", "on": true}));
    ok &= run(
        s,
        "range.setValues",
        json!({"range": format!("B{first}"), "values": [["Diseño de logotipo", 1, 600, 0.21], ["Horas de consultoría", 8, 45, 0.21]]}),
    );
    let rows: Vec<_> = (first..=last)
        .map(|r| json!([format!("=IF(C{r}*D{r}=0,\"\",ROUND(C{r}*D{r},2))"), format!("=IF(F{r}=\"\",\"\",ROUND(F{r}*E{r},2))")]))
        .collect();
    ok &= run(s, "range.setValues", json!({"range": format!("F{first}"), "values": rows}));
    ok &= num(s, &format!("D{first}:D{last}"), EURO);
    ok &= num(s, &format!("E{first}:E{last}"), "0%");
    ok &= num(s, &format!("F{first}:G{last}"), EURO);
    ok &= run(s, "home.borders", json!({"range": format!("B{first}:G{last}"), "preset": "insideHorizontal", "color": "#BFBFBF"}));

    // Totals 33..39. IVA is shown per rate; the total takes every line, whatever its rate.
    let lines = |c: char| format!("{c}{first}:{c}{last}");
    ok &= set(s, "F33", "Base imponible");
    ok &= set(s, "G33", &format!("=SUM({})", lines('F')));
    for (r, rate, label) in [(34, "0.21", "IVA 21 %"), (35, "0.1", "IVA 10 %"), (36, "0.04", "IVA 4 %")] {
        ok &= set(s, &format!("F{r}"), label);
        ok &= set(s, &format!("G{r}"), &format!("=SUMIF({},{rate},{})", lines('E'), lines('G')));
    }
    ok &= set(s, "F37", "Retención IRPF");
    ok &= set(s, "E37", "0.15");
    ok &= num(s, "E37", "0%");
    ok &= set(s, "G37", "=-ROUND(G33*E37,2)");
    ok &= set(s, "F38", "Total factura");
    ok &= set(s, "G38", &format!("=G33+SUM({})+G37", lines('G')));
    ok &= num(s, "G33:G38", EURO);
    ok &= run(s, "home.cellStyle", json!({"range": "F38:G38", "name": "Total"}));
    ok &= run(s, "home.alignRight", json!({"range": "F33:F38"}));

    ok &= set(s, "B40", "Forma de pago: transferencia a la cuenta ES00 0000 0000 0000 0000 0000");
    ok &= run(s, "home.mergeCells", json!({"range": "B40:G40"}));
    // One short line each, so they print inside the page.
    for (cell, text) in [
        ("B41", "Retención (solo actividades profesionales): 15 %, o 7 % el año de alta y los dos siguientes,"),
        ("B42", "si el cliente es empresa o profesional; 0 % si es un particular."),
        ("B43", "Una línea exenta de IVA lleva tipo 0 % y, aquí debajo, el artículo de la Ley 37/1992 que la exime."),
        ("B44", "Los programas de facturación deben cumplir el reglamento VERI*FACTU: consulte a su asesor."),
    ] {
        ok &= note(s, cell, text);
    }

    ok &= width(s, "A:A", 20);
    ok &= width(s, "B:B", 230);
    ok &= width(s, "C:C", 80);
    ok &= width(s, "D:E", 100);
    ok &= width(s, "F:G", 120);
    ok &= run(s, "pageLayout.pageSetup", json!({"orientation": "portrait", "paper": "A4", "fitWidth": 1, "fitHeight": 1, "footer": SPANISH_FOOTER}));
    ok &= run(s, "selection.set", json!({"cell": "C6"}));
    ok
}

const INCOME_KINDS: &str = "\"Prestación de servicios,Venta de bienes,Subvención,Otros ingresos\"";
/// Expense kinds, as in the income tax return for business activities.
const EXPENSE_KINDS: [&str; 10] = [
    "Compras",
    "Alquiler",
    "Reparaciones",
    "Servicios profesionales",
    "Suministros",
    "Cuota de autónomos",
    "Seguros",
    "Material de oficina",
    "Formación",
    "Otros gastos",
];

/// The income and expense books, the depreciation book and a yearly summary.
pub(super) fn books(s: &mut Session) -> bool {
    // Ingresos B..N.
    let mut ok = heading(
        s,
        "Libro registro de ventas e ingresos 2026",
        "Una fila por factura emitida. Datos de ejemplo: sustitúyalos. Cuota, retención, total y trimestre se calculan solos hasta la fila 65.",
        json!([
            "Fecha de expedición",
            "Fecha de la operación",
            "Serie y número",
            "NIF del cliente",
            "Cliente",
            "Tipo de ingreso",
            "Base imponible",
            "Tipo IVA",
            "Cuota IVA",
            "Tipo IRPF",
            "Retención",
            "Total",
            "Trimestre"
        ]),
        'N',
    );
    let invoices = [
        ("1/20/2026", "2026-001", "B00000000", "Estudio de diseño (ejemplo)", 1200.0, 0.21, 0.15),
        ("2/14/2026", "2026-002", "", "Particular", 450.0, 0.21, 0.0),
        ("4/3/2026", "2026-003", "B00000001", "Academia (ejemplo)", 800.0, 0.21, 0.15),
        ("7/9/2026", "2026-004", "B00000002", "Clínica (ejemplo)", 2300.0, 0.21, 0.15),
        ("10/2/2026", "2026-005", "", "Particular", 350.0, 0.10, 0.0),
    ];
    for (i, (date, number, nif, client, base, iva, irpf)) in invoices.iter().enumerate() {
        let r = FIRST + i;
        ok &= set(s, &format!("B{r}"), date);
        ok &= run(s, "range.setValues", json!({"range": format!("D{r}"), "values": [[number, nif, client, "Prestación de servicios", base, iva]]}));
        ok &= run(s, "range.setValues", json!({"range": format!("K{r}"), "values": [[irpf]]}));
    }
    ok &= down(s, 'J', 'H', |r| format!("ROUND(H{r}*I{r},2)"));
    ok &= down(s, 'L', 'H', |r| format!("ROUND(H{r}*K{r},2)"));
    ok &= down(s, 'M', 'H', |r| format!("H{r}+J{r}-L{r}"));
    ok &= quarter_col(s, 'N', 'B');
    ok &= dates(s, 'B') && dates(s, 'C');
    ok &= num(s, &format!("H{FIRST}:H{LAST}"), EURO);
    ok &= num(s, &format!("J{FIRST}:J{LAST}"), EURO);
    ok &= num(s, &format!("L{FIRST}:M{LAST}"), EURO);
    ok &= num(s, &format!("I{FIRST}:I{LAST}"), "0%");
    ok &= num(s, &format!("K{FIRST}:K{LAST}"), "0%");
    ok &= list(s, &format!("G{FIRST}:G{LAST}"), INCOME_KINDS);
    ok &= width(s, "B:C", 90);
    ok &= width(s, "D:E", 90);
    ok &= width(s, "F:G", 170);
    ok &= width(s, "H:N", 90);
    ok &= finish_register(s);

    // Gastos B..N.
    ok &= new_sheet(s, "Gastos");
    ok &= heading(
        s,
        "Libro registro de compras y gastos 2026",
        "Solo gastos de la actividad con factura a su nombre. Si el IVA no es deducible, póngalo en «No» y pasa a ser gasto.",
        json!([
            "Fecha",
            "Factura del proveedor",
            "NIF del proveedor",
            "Proveedor",
            "Concepto",
            "Tipo de gasto",
            "Base imponible",
            "Tipo IVA",
            "Cuota IVA",
            "IVA deducible",
            "Gasto deducible",
            "Total",
            "Trimestre"
        ]),
        'N',
    );
    let expenses = [
        ("1/5/2026", "", "", "Seguridad Social", "Cuota de enero", "Cuota de autónomos", 87.61, 0.0),
        ("1/10/2026", "F-1029", "A00000000", "Operadora (ejemplo)", "Fibra y móvil", "Suministros", 45.45, 0.21),
        ("3/2/2026", "T-552", "B00000003", "Tienda de informática", "Impresora", "Material de oficina", 189.0, 0.21),
        ("4/15/2026", "G-77", "B00000004", "Gestoría (ejemplo)", "Asesoría del trimestre", "Servicios profesionales", 60.0, 0.21),
        ("9/20/2026", "L-310", "B00000005", "Librería", "Libros técnicos", "Formación", 40.0, 0.04),
    ];
    for (i, (date, number, nif, supplier, concept, kind, base, iva)) in expenses.iter().enumerate() {
        let r = FIRST + i;
        ok &= set(s, &format!("B{r}"), date);
        ok &= run(s, "range.setValues", json!({"range": format!("C{r}"), "values": [[number, nif, supplier, concept, kind, base, iva]]}));
    }
    ok &= down(s, 'J', 'H', |r| format!("ROUND(H{r}*I{r},2)"));
    ok &= down(s, 'L', 'H', |r| format!("H{r}+IF(K{r}=\"No\",J{r},0)"));
    ok &= down(s, 'M', 'H', |r| format!("H{r}+J{r}"));
    ok &= quarter_col(s, 'N', 'B');
    ok &= dates(s, 'B');
    ok &= num(s, &format!("H{FIRST}:H{LAST}"), EURO);
    ok &= num(s, &format!("J{FIRST}:J{LAST}"), EURO);
    ok &= num(s, &format!("L{FIRST}:M{LAST}"), EURO);
    ok &= num(s, &format!("I{FIRST}:I{LAST}"), "0%");
    ok &= list(s, &format!("G{FIRST}:G{LAST}"), &format!("\"{}\"", EXPENSE_KINDS.join(",")));
    ok &= list(s, &format!("K{FIRST}:K{LAST}"), YES_NO);
    ok &= width(s, "B:D", 90);
    ok &= width(s, "E:G", 160);
    ok &= width(s, "H:N", 90);
    ok &= finish_register(s);

    // Amortizaciones B..G.
    ok &= new_sheet(s, "Amortizaciones");
    ok &= heading(
        s,
        "Libro registro de bienes de inversión 2026",
        "Coeficientes máximos de la tabla de estimación directa simplificada: equipos informáticos 26 %, mobiliario 10 %, útiles 30 %. Confírmelos.",
        json!(["Fecha de alta", "Bien", "Valor de adquisición", "Coeficiente anual", "Meses de uso en el año", "Amortización del año"]),
        'G',
    );
    ok &= set(s, &format!("B{FIRST}"), "3/2/2026");
    ok &= run(s, "range.setValues", json!({"range": format!("C{FIRST}"), "values": [["Ordenador portátil", 999, 0.26, 10]]}));
    ok &= down(s, 'G', 'D', |r| format!("ROUND(D{r}*E{r}*IF(F{r}=\"\",12,F{r})/12,2)"));
    ok &= dates(s, 'B');
    ok &= num(s, &format!("D{FIRST}:D{LAST}"), EURO);
    ok &= num(s, &format!("E{FIRST}:E{LAST}"), "0%");
    ok &= num(s, &format!("G{FIRST}:G{LAST}"), EURO);
    ok &= width(s, "B:B", 90);
    ok &= width(s, "C:C", 200);
    ok &= width(s, "D:G", 110);
    ok &= finish_register(s);

    // Resumen: quarters 6..9, year 10, then the expense kinds.
    ok &= new_sheet(s, "Resumen");
    ok &= heading(
        s,
        "Resumen del año: rendimiento de la actividad",
        "Rendimiento antes de gastos de difícil justificación y reducciones. Cifras orientativas: confírmelas con su asesor.",
        json!(["Trimestre", "Ingresos", "Gastos deducibles", "Diferencia"]),
        'E',
    );
    let col = |sheet: &str, c: char| format!("{sheet}!${c}${FIRST}:${c}${LAST}");
    for q in 1..=4 {
        let r = FIRST + q - 1;
        ok &= set(s, &format!("B{r}"), &format!("{q}T"));
        ok &= set(s, &format!("C{r}"), &format!("=SUMIFS({},{},{q})", col("Ingresos", 'H'), col("Ingresos", 'N')));
        ok &= set(s, &format!("D{r}"), &format!("=SUMIFS({},{},{q})", col("Gastos", 'L'), col("Gastos", 'N')));
        ok &= set(s, &format!("E{r}"), &format!("=C{r}-D{r}"));
    }
    let year = FIRST + 4;
    ok &= set(s, &format!("B{year}"), "Año");
    for c in ['C', 'D', 'E'] {
        ok &= set(s, &format!("{c}{year}"), &format!("=SUM({c}{FIRST}:{c}{})", year - 1));
    }
    ok &= run(s, "home.cellStyle", json!({"range": format!("B{year}:E{year}"), "name": "Total"}));
    let (dep, net) = (year + 1, year + 2);
    ok &= set(s, &format!("B{dep}"), "Amortizaciones del año");
    ok &= set(s, &format!("E{dep}"), &format!("=-SUM({})", col("Amortizaciones", 'G')));
    ok &= set(s, &format!("B{net}"), "Rendimiento neto");
    ok &= set(s, &format!("E{net}"), &format!("=E{year}+E{dep}"));
    ok &= run(s, "home.cellStyle", json!({"range": format!("B{net}:E{net}"), "name": "Total"}));
    let kinds = net + 2;
    ok &= run(s, "range.setValues", json!({"range": format!("B{kinds}"), "values": [["Tipo de gasto", "Gasto deducible"]]}));
    ok &= run(s, "home.cellStyle", json!({"range": format!("B{kinds}:C{kinds}"), "name": "Heading 4"}));
    for (i, kind) in EXPENSE_KINDS.iter().enumerate() {
        let r = kinds + 1 + i;
        ok &= set(s, &format!("B{r}"), kind);
        ok &= set(s, &format!("C{r}"), &format!("=SUMIF({},B{r},{})", col("Gastos", 'G'), col("Gastos", 'L')));
    }
    ok &= num(s, &format!("C{FIRST}:E{}", kinds + EXPENSE_KINDS.len()), EURO);
    ok &= width(s, "B:B", 170);
    ok &= width(s, "C:E", 120);
    ok &= page_setup(s, SPANISH_FOOTER);
    ok &= run(s, "selection.set", json!({"range": format!("B5:D{}", year - 1)}));
    ok &= run(s, "insert.chart", json!({"type": "column", "title": "Ingresos y gastos por trimestre", "at": "G5", "width": 460, "height": 260}));
    let q = |c: char| format!("'Resumen'!${c}${FIRST}:${c}${}", year - 1);
    ok &=
        set_chart_series(s, "Resumen", &[("'Resumen'!$C$5".into(), q('B'), q('C'), "#2E8B57"), ("'Resumen'!$D$5".into(), q('B'), q('D'), "#C0504D")]);
    ok &= run(s, "sheet.activate", json!({"sheet": "Ingresos"}));
    ok &= run(s, "selection.set", json!({"cell": format!("B{FIRST}")}));
    ok
}

/// Issued and received invoices, and the quarter's modelo 303 boxes for the general regime.
pub(super) fn quarterly_vat(s: &mut Session) -> bool {
    // Emitidas B..H.
    let mut ok = heading(
        s,
        "Facturas emitidas 2026",
        "Una fila por factura. Datos de ejemplo: sustitúyalos. La cuota y el trimestre se calculan solos hasta la fila 65.",
        json!(["Fecha", "Factura", "Cliente", "Base imponible", "Tipo IVA", "Cuota IVA", "Trimestre"]),
        'H',
    );
    let issued = [
        ("1/20/2026", "2026-001", "Estudio de diseño (ejemplo)", 1200.0, 0.21),
        ("2/14/2026", "2026-002", "Particular", 450.0, 0.21),
        ("3/9/2026", "2026-003", "Particular (IVA reducido)", 300.0, 0.10),
        ("4/3/2026", "2026-004", "Academia (ejemplo)", 800.0, 0.21),
        ("7/9/2026", "2026-005", "Clínica (ejemplo)", 2300.0, 0.21),
        ("10/2/2026", "2026-006", "Editorial (ejemplo)", 500.0, 0.04),
    ];
    for (i, (date, number, client, base, iva)) in issued.iter().enumerate() {
        let r = FIRST + i;
        ok &= set(s, &format!("B{r}"), date);
        ok &= run(s, "range.setValues", json!({"range": format!("C{r}"), "values": [[number, client, base, iva]]}));
    }
    ok &= down(s, 'G', 'E', |r| format!("ROUND(E{r}*F{r},2)"));
    ok &= quarter_col(s, 'H', 'B');
    ok &= dates(s, 'B');
    ok &= num(s, &format!("E{FIRST}:E{LAST}"), EURO);
    ok &= num(s, &format!("G{FIRST}:G{LAST}"), EURO);
    ok &= num(s, &format!("F{FIRST}:F{LAST}"), "0%");
    ok &= width(s, "B:C", 90);
    ok &= width(s, "D:D", 220);
    ok &= width(s, "E:H", 100);
    ok &= finish_register(s);

    // Recibidas B..I.
    ok &= new_sheet(s, "Recibidas");
    ok &= heading(
        s,
        "Facturas recibidas 2026",
        "Solo facturas de la actividad a su nombre. Bien de inversión: «Sí» si el bien dura más de un año y cuesta más de 3.005,06 €.",
        json!(["Fecha", "Factura", "Proveedor", "Base imponible", "Tipo IVA", "Cuota IVA", "Bien de inversión", "Trimestre"]),
        'I',
    );
    let received = [
        ("1/10/2026", "F-1029", "Operadora (ejemplo)", 45.45, 0.21, "No"),
        ("3/2/2026", "T-552", "Tienda de informática", 189.0, 0.21, "No"),
        ("4/15/2026", "G-77", "Gestoría (ejemplo)", 60.0, 0.21, "No"),
        ("5/18/2026", "M-12", "Maquinaria (ejemplo)", 3500.0, 0.21, "Sí"),
        ("9/20/2026", "L-310", "Librería", 40.0, 0.04, "No"),
    ];
    for (i, (date, number, supplier, base, iva, asset)) in received.iter().enumerate() {
        let r = FIRST + i;
        ok &= set(s, &format!("B{r}"), date);
        ok &= run(s, "range.setValues", json!({"range": format!("C{r}"), "values": [[number, supplier, base, iva]]}));
        ok &= set(s, &format!("H{r}"), asset);
    }
    ok &= down(s, 'G', 'E', |r| format!("ROUND(E{r}*F{r},2)"));
    ok &= quarter_col(s, 'I', 'B');
    ok &= dates(s, 'B');
    ok &= num(s, &format!("E{FIRST}:E{LAST}"), EURO);
    ok &= num(s, &format!("G{FIRST}:G{LAST}"), EURO);
    ok &= num(s, &format!("F{FIRST}:F{LAST}"), "0%");
    ok &= list(s, &format!("H{FIRST}:H{LAST}"), YES_NO);
    ok &= width(s, "B:C", 90);
    ok &= width(s, "D:D", 220);
    ok &= width(s, "E:I", 100);
    ok &= finish_register(s);

    // Modelo 303.
    ok &= new_sheet(s, "Modelo 303");
    ok &= heading(
        s,
        "Modelo 303: IVA del trimestre",
        "Régimen general. Copie las cifras en las casillas del modelo en la sede de la AEAT y compruébelas con su asesor.",
        json!(["Concepto", "Casillas", "Base imponible", "Tipo", "Cuota"]),
        'F',
    );
    // The quarter to work out, above the headings.
    ok &= set(s, "B4", "Trimestre (1 a 4)");
    ok &= set(s, "C4", "1");
    ok &= input(s, "C4");
    ok &= list(s, "C4", "\"1,2,3,4\"");
    ok &= run(s, "home.alignLeft", json!({"range": "C4"}));
    ok &= set(
        s,
        "D4",
        "=\"Plazo: \"&CHOOSE($C$4,\"del 1 al 20 de abril\",\"del 1 al 20 de julio\",\"del 1 al 20 de octubre\",\"del 1 al 30 de enero del año siguiente\")",
    );
    let em = |c: char| format!("Emitidas!${c}${FIRST}:${c}${LAST}");
    let re = |c: char| format!("Recibidas!${c}${FIRST}:${c}${LAST}");
    ok &= set(s, "B6", "IVA devengado");
    ok &= run(s, "home.cellStyle", json!({"range": "B6:F6", "name": "Heading 4"}));
    for (r, label, boxes, rate) in
        [(7, "Tipo superreducido", "01 · 02 · 03", "0.04"), (8, "Tipo reducido", "04 · 05 · 06", "0.1"), (9, "Tipo general", "07 · 08 · 09", "0.21")]
    {
        ok &= run(s, "range.setValues", json!({"range": format!("B{r}"), "values": [[label, boxes]]}));
        ok &= set(s, &format!("D{r}"), &format!("=SUMIFS({},{},E{r},{},$C$4)", em('E'), em('F'), em('H')));
        ok &= set(s, &format!("E{r}"), rate);
        ok &= set(s, &format!("F{r}"), &format!("=SUMIFS({},{},E{r},{},$C$4)", em('G'), em('F'), em('H')));
    }
    ok &= run(s, "range.setValues", json!({"range": "B10", "values": [["Total cuota devengada", "27"]]}));
    ok &= set(s, "F10", "=SUM(F7:F9)");
    ok &= set(s, "B12", "IVA deducible");
    ok &= run(s, "home.cellStyle", json!({"range": "B12:F12", "name": "Heading 4"}));
    for (r, label, boxes, criterion) in
        [(13, "Operaciones interiores corrientes", "28 · 29", "\"<>Sí\""), (14, "Bienes de inversión", "30 · 31", "\"Sí\"")]
    {
        ok &= run(s, "range.setValues", json!({"range": format!("B{r}"), "values": [[label, boxes]]}));
        ok &= set(s, &format!("D{r}"), &format!("=SUMIFS({},{},{criterion},{},$C$4)", re('E'), re('H'), re('I')));
        ok &= set(s, &format!("F{r}"), &format!("=SUMIFS({},{},{criterion},{},$C$4)", re('G'), re('H'), re('I')));
    }
    ok &= run(s, "range.setValues", json!({"range": "B15", "values": [["Total a deducir", "45"]]}));
    ok &= set(s, "F15", "=F13+F14");
    ok &= run(s, "range.setValues", json!({"range": "B17", "values": [["Resultado del régimen general", "46"]]}));
    ok &= set(s, "F17", "=F10-F15");
    ok &= set(s, "B18", "Cuotas a compensar de trimestres anteriores");
    ok &= set(s, "F18", "0");
    ok &= input(s, "F18");
    ok &= set(s, "B19", "Resultado");
    ok &= set(s, "F19", "=F17-MIN(F18,MAX(F17,0))");
    ok &= run(s, "home.cellStyle", json!({"range": "B19:F19", "name": "Total"}));
    ok &= set(
        s,
        "B20",
        "=IF(F19>0,\"A ingresar\",IF(F19<0,IF($C$4=4,\"A compensar o a devolver\",\"A compensar en los trimestres siguientes\"),\"Sin cuota\"))",
    );
    ok &= run(s, "home.bold", json!({"range": "B20", "on": true}));
    ok &= num(s, "D7:D15", EURO);
    ok &= num(s, "F7:F19", EURO);
    ok &= num(s, "E7:E9", "0%");
    ok &= run(s, "home.alignCenter", json!({"range": "C7:C17"}));
    ok &= note(s, "B21", "Las facturas con otros tipos (0 %, exentas, recargo de equivalencia) no entran en este cálculo.");

    // The year by quarter, for the chart.
    ok &= run(s, "range.setValues", json!({"range": "B23", "values": [["Trimestre", "IVA devengado", "IVA deducible", "Diferencia"]]}));
    ok &= run(s, "home.cellStyle", json!({"range": "B23:E23", "name": "Heading 4"}));
    ok &= run(s, "home.wrapText", json!({"range": "B23:E23", "on": true}));
    ok &= run(s, "home.autofitRowHeight", json!({"rows": "23:23"}));
    for q in 1..=4 {
        let r = 23 + q;
        ok &= set(s, &format!("B{r}"), &format!("{q}T"));
        ok &= set(s, &format!("C{r}"), &format!("=SUMIFS({},{},{q})", em('G'), em('H')));
        ok &= set(s, &format!("D{r}"), &format!("=SUMIFS({},{},{q})", re('G'), re('I')));
        ok &= set(s, &format!("E{r}"), &format!("=C{r}-D{r}"));
    }
    ok &= num(s, "C24:E27", EURO);
    ok &= width(s, "B:B", 260);
    ok &= width(s, "C:C", 90);
    ok &= width(s, "D:F", 110);
    ok &= page_setup(s, SPANISH_FOOTER);
    ok &= run(s, "selection.set", json!({"range": "B23:B27"}));
    ok &= run(s, "insert.chart", json!({"type": "column", "title": "IVA a ingresar por trimestre", "at": "H5", "width": 460, "height": 260}));
    ok &= set_chart_series(
        s,
        "Modelo 303",
        &[("'Modelo 303'!$E$23".into(), "'Modelo 303'!$B$24:$B$27".into(), "'Modelo 303'!$E$24:$E$27".into(), "#2E6DA4")],
    );
    ok &= run(s, "sheet.activate", json!({"sheet": "Emitidas"}));
    ok &= run(s, "selection.set", json!({"cell": format!("B{FIRST}")}));
    ok
}

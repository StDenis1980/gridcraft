//! The ribbon's number format list shows, beside each format, the selected cell as that format
//! would show it, on the right of the row (as Excel's list does), so a format is chosen by sight.

use egui::Shape;
use egui_kittest::kittest::Queryable;
use gridcraft_engine::Session;
use gridcraft_ui_egui::SheetApp;
use serde_json::json;

type Harness = egui_kittest::Harness<'static, SheetApp>;

fn harness() -> Harness {
    let mut s = Session::new();
    s.new_workbook();
    s.execute("cell.set", json!({"cell": "A1", "input": "1234"})).unwrap();
    s.execute("selection.set", json!({"range": "A1"})).unwrap();
    let mut h = crate::common::builder().with_size(egui::vec2(1400.0, 900.0)).build_ui_state(
        |ui, app: &mut SheetApp| {
            app.logic(ui.ctx());
            app.ui(ui);
        },
        crate::common::app(s),
    );
    h.run_steps(4);
    h
}

/// Painted texts equal to `text`: their left x.
fn painted(h: &Harness, text: &str) -> Vec<f32> {
    h.output()
        .shapes
        .iter()
        .filter_map(|s| match &s.shape {
            Shape::Text(t) if t.galley.job.text == text => Some(t.pos.x),
            _ => None,
        })
        .collect()
}

#[test]
fn the_number_format_list_shows_each_formats_sample() {
    let mut h = harness();
    h.get_all_by_label("General").next().expect("the number format box").click();
    h.run_steps(4);
    let name = painted(&h, "Currency");
    let sample = painted(&h, "$1,234.00");
    assert!(!name.is_empty(), "the Currency row is shown");
    assert!(!sample.is_empty(), "the Currency row shows the selected cell as currency on its own");
    assert!(sample[0] > name[0] + 40.0, "the sample sits on the row's right: name at {name:?}, sample at {sample:?}");
    assert!(!painted(&h, "1234.00").is_empty() || !painted(&h, "1234,00").is_empty(), "the Number row's sample too");
}

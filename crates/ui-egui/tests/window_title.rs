//! The OS window title names the workbook and, outside macOS, this build's product name.

use gridcraft_engine::Session;
use gridcraft_ui_egui::{Services, SheetApp, window_title};

#[test]
fn a_new_blank_workbook_titles_the_window_book1_and_the_product() {
    let mut session = Session::new();
    session.new_workbook();
    let app = SheetApp::new(session, Services::default());
    let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_ui_state(
        |ui, app: &mut SheetApp| {
            let ctx = ui.ctx().clone();
            app.logic(&ctx);
            app.ui(ui);
        },
        app,
    );
    h.run_steps(3);
    let title = h.state().grid.last_title.clone();
    assert_eq!(title.as_deref(), Some(window_title("Book1", false).as_str()));
    if !cfg!(target_os = "macos") {
        assert_eq!(title.as_deref(), Some(format!("Book1 - {}", gridcraft_engine::brand::NAME).as_str()));
    }
}

//! Opening a password-protected workbook asks for its password (#11).

use egui::accesskit::Role;
use egui_kittest::kittest::Queryable as _;
use gridcraft_engine::{Session, core::CellRef, core::Value};
use gridcraft_ui_egui::{Inbox, Services, SheetApp};
use serde_json::json;

const PW: &str = "synthetic-pw-8842";

fn harness(services: Services) -> egui_kittest::Harness<'static, SheetApp> {
    crate::support::services_harness(services, 4)
}

/// A workbook with 42 in A1, saved as `.xlsx` and encrypted with `PW`.
fn encrypted_bytes() -> Vec<u8> {
    let mut s = Session::new();
    s.new_workbook();
    s.run("cell.set", json!({"cell": "A1", "input": "42"})).unwrap();
    let xlsx = gridcraft_engine::io::save_bytes(&s.active().unwrap().wb, "book.xlsx").unwrap();
    gridcraft_xlsx::testing::encrypt_workbook(&xlsx, PW)
}

/// `encrypted_bytes()` written to `Secret.xlsx` in a fresh temp dir named after `tag`.
fn encrypted_file(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("gridcraft-encrypted-ui-{tag}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("Secret.xlsx");
    std::fs::write(&path, encrypted_bytes()).unwrap();
    path
}

fn dialog(h: &egui_kittest::Harness<'static, SheetApp>) -> Option<String> {
    h.state().dialog.as_ref().map(|d| d.name().to_string())
}

fn a1(h: &egui_kittest::Harness<'static, SheetApp>) -> Value {
    h.state().session.active().unwrap().wb.active().unwrap().value(CellRef::parse("A1").unwrap())
}

#[test]
fn opening_an_encrypted_file_asks_for_its_password() {
    let path = encrypted_file("desktop");
    let p = path.to_string_lossy().to_string();
    let mut h = harness(Services::default());
    h.state_mut().open_path(&p);
    h.run_steps(3);
    assert_eq!(dialog(&h).as_deref(), Some("openPassword"));
    assert!(h.state().message.is_none(), "no error box: the dialog asks instead");
    assert!(h.query_by_role(Role::PasswordInput).is_some(), "the password is masked");

    // A wrong password says so, clears the field, and keeps the dialog and its file.
    h.get_by_role(Role::PasswordInput).click();
    h.run_steps(2);
    h.input_mut().events.push(egui::Event::Text("wrong-guess".into()));
    h.run_steps(2);
    h.get_by_label("   OK   ").click();
    h.run_steps(3);
    let msg = h.state().message.as_ref().map(|(_, m)| m.clone()).unwrap_or_default();
    assert!(msg.contains(gridcraft_engine::io::WRONG_PASSWORD), "{msg}");
    assert!(!msg.contains("wrong-guess"), "the password is never quoted: {msg}");
    assert_eq!(dialog(&h).as_deref(), Some("openPassword"));
    assert_eq!(h.state().dialog.as_ref().and_then(|d| d.values.get("password")), Some(&json!("")));
    assert_eq!(h.state().dialog.as_ref().and_then(|d| d.values.get("path")), Some(&json!(p)));
    h.state_mut().message = None;

    // The right one opens the workbook: its own file (Save encrypts it again), in Recent, with the notice.
    h.state_mut().dialog.as_mut().unwrap().values.insert("password".into(), json!(PW));
    h.get_by_label("   OK   ").click();
    h.run_steps(3);
    assert_eq!(dialog(&h), None);
    assert_eq!(a1(&h), Value::Number(42.0));
    assert_eq!(h.state().session.active().unwrap().path.as_deref(), Some(p.as_str()));
    assert!(h.state().session.active().unwrap().password.is_some());
    assert_eq!(h.state().ui.recent.first(), Some(&p));
    assert!(h.state().notice.as_ref().is_some_and(|n| n.text.contains("password-protected")));
    std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
}

#[test]
fn cancel_leaves_nothing_open() {
    let path = encrypted_file("cancel");
    let mut h = harness(Services::default());
    let before = h.state().session.documents().len();
    h.state_mut().open_path(&path.to_string_lossy());
    h.run_steps(3);
    assert_eq!(dialog(&h).as_deref(), Some("openPassword"));
    h.get_by_label(" Cancel ").click();
    h.run_steps(3);
    assert_eq!(dialog(&h), None);
    assert_eq!(h.state().session.documents().len(), before);
    assert!(h.state().message.is_none());
    std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
}

#[test]
fn a_file_picked_on_the_web_asks_for_its_password_too() {
    let inbox: Inbox = Default::default();
    let mut h = harness(Services { inbox: Some(inbox.clone()), ..Default::default() });
    inbox.lock().unwrap().push(("Secret.xlsx".into(), encrypted_bytes()));
    h.run_steps(3);
    assert_eq!(dialog(&h).as_deref(), Some("openPassword"));
    assert!(h.state().dialog.as_ref().is_some_and(|d| d.values.contains_key("base64")));
    h.state_mut().dialog.as_mut().unwrap().values.insert("password".into(), json!(PW));
    h.get_by_label("   OK   ").click();
    h.run_steps(3);
    assert_eq!(dialog(&h), None);
    assert_eq!(a1(&h), Value::Number(42.0));
    assert_eq!(h.state().session.active().unwrap().path, None);
}

#[test]
fn an_untouched_password_field_is_incorrect_not_required() {
    let path = encrypted_file("untouched");
    let mut h = harness(Services::default());
    h.state_mut().open_path(&path.to_string_lossy());
    h.run_steps(3);
    h.get_by_label("   OK   ").click();
    h.run_steps(3);
    let msg = h.state().message.as_ref().map(|(_, m)| m.clone()).unwrap_or_default();
    assert!(msg.contains(gridcraft_engine::io::WRONG_PASSWORD), "{msg}");
    assert_eq!(dialog(&h).as_deref(), Some("openPassword"));
    std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
}

#[test]
fn the_backstage_closes_once_the_password_opens_the_workbook() {
    let path = encrypted_file("backstage");
    let mut h = harness(Services::default());
    h.state_mut().backstage = Some(gridcraft_ui_egui::backstage::Backstage::new(gridcraft_ui_egui::backstage::Page::Open));
    h.state_mut().open_path(&path.to_string_lossy());
    h.run_steps(3);
    h.state_mut().dialog.as_mut().unwrap().values.insert("password".into(), json!(PW));
    h.get_by_label("   OK   ").click();
    h.run_steps(3);
    assert_eq!(a1(&h), Value::Number(42.0));
    assert!(h.state().backstage.is_none(), "the opened workbook shows, not the Backstage");
    std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
}

#[test]
fn app_open_over_the_control_channel_never_opens_a_dialog() {
    use gridcraft_ui_egui::control::{Outcome, handle};
    let path = encrypted_file("control");
    let p = path.to_string_lossy().to_string();
    let mut h = harness(Services::default());
    let ctx = h.ctx.clone();
    let reply = |o: Outcome| match o {
        Outcome::Done(v) => v,
        Outcome::Screenshot { .. } => json!(null),
    };
    let r = reply(handle(h.state_mut(), &ctx, "app.open", &json!({"path": p})));
    assert_eq!(r["ok"], json!(false), "{r}");
    assert!(r["error"].as_str().unwrap_or_default().contains(gridcraft_engine::io::PASSWORD_REQUIRED), "{r}");
    assert_eq!(dialog(&h), None, "programmatic calls never open dialogs");
    let r = reply(handle(h.state_mut(), &ctx, "app.open", &json!({"path": p, "password": PW})));
    assert_eq!(r["ok"], json!(true), "{r}");
    assert_eq!(r["result"]["title"], json!("Secret.xlsx"));
    assert_eq!(a1(&h), Value::Number(42.0));
    assert_eq!(h.state().ui.recent.first(), Some(&p));
    std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
}

#[test]
fn file_info_encrypts_the_workbook_with_a_confirmed_password() {
    let mut h = harness(Services::default());
    h.get_by_label("File").click();
    h.run_steps(3);
    h.get_by_role_and_label(Role::Button, "Info").click();
    h.run_steps(3);
    assert!(h.query_by_label("Require a password to open this workbook.").is_some());
    h.get_by_role_and_label(Role::Button, "Encrypt with Password").click();
    h.run_steps(3);
    assert_eq!(dialog(&h).as_deref(), Some("encryptPassword"));
    assert_eq!(h.query_all_by_role(Role::PasswordInput).count(), 2, "asked twice, masked");

    // Two different entries are refused and leave the workbook as it was.
    let set = |h: &mut egui_kittest::Harness<'static, SheetApp>, a: &str, b: &str| {
        let d = h.state_mut().dialog.as_mut().unwrap();
        d.values.insert("password".into(), json!(a));
        d.values.insert("confirmPassword".into(), json!(b));
    };
    set(&mut h, PW, "typo");
    h.get_by_label("   OK   ").click();
    h.run_steps(3);
    assert!(h.state().message.as_ref().is_some_and(|(_, m)| m.contains("match")));
    assert!(h.state().session.active().unwrap().password.is_none());
    h.state_mut().message = None;
    if dialog(&h).is_none() {
        h.state_mut().run("file.encrypt", json!({})).unwrap();
        h.run_steps(3);
    }
    set(&mut h, PW, PW);
    h.get_by_label("   OK   ").click();
    h.run_steps(3);
    assert_eq!(dialog(&h), None);
    let doc = h.state().session.active().unwrap();
    assert!(doc.password.is_some() && doc.is_dirty(), "set, and a change to save");
    assert!(h.query_by_label("A password is required to open this workbook.").is_some());
    let journal = serde_json::to_string(&h.state().session.journal).unwrap();
    assert!(!journal.contains(PW), "the password never reaches the journal");
}

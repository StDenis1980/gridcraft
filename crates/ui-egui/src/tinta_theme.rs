//! Tinta Office: the Tinta look, the colours and control shapes of Tinta Correo carried over to
//! Tinta Calculo. The window is a soft grey with a wash of the app's colour; the ribbon, menus,
//! dialogs and fields are lighter cards; buttons have no frame, only a tint of the accent that
//! deepens under the pointer; corners are rounder. The accent is the olive green of the Tinta
//! Calculo icon.
//!
//! Only the chrome changes, never the layout or the sheet: the sheet stays white with upstream's
//! gridlines, selection and reference colours. Off (File ▸ Options ▸ General ▸ Tinta Office look,
//! `view.tintaLook`), the interface is upstream's classic palette. The setting is read through
//! [`on`], which the app sets each frame from its preferences; [`skin`] is the pure mapping, so
//! tests never touch the global switch.

use std::sync::atomic::{AtomicBool, Ordering};

use egui::{Color32, CornerRadius, Stroke, Visuals};

use crate::theme::Tokens;

/// Off until the app reads its setting.
static ON: AtomicBool = AtomicBool::new(false);

/// Whether the Tinta look is on. Never in this crate's unit tests: they share one process, and
/// upstream's compare colours with the classic palette whatever other tests switch on.
pub fn on() -> bool {
    !cfg!(test) && allowed() && ON.load(Ordering::Relaxed)
}

/// Set by the desktop app ([`allow`]): only it shows the Tinta look. Upstream's integration tests
/// and examples compare colours with the classic palette, so they keep it, unless `TINTA_LOOK` is
/// set in the environment (for screenshots of the Tinta look).
static ALLOWED: AtomicBool = AtomicBool::new(false);

/// Let this program show the Tinta look (the desktop app calls it once, at start).
pub fn allow() {
    ALLOWED.store(true, Ordering::Relaxed);
}

fn allowed() -> bool {
    static ENV: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    ALLOWED.load(Ordering::Relaxed) || *ENV.get_or_init(|| std::env::var_os("TINTA_LOOK").is_some())
}

/// Turn the Tinta look on or off; true when that changed it, so the caller re-applies the theme.
/// Never in unit tests, where re-applying the theme mid-test would change their frames.
pub fn set(on: bool) -> bool {
    !cfg!(test) && ON.swap(on, Ordering::Relaxed) != on
}

/// The palette in use: [`skin`] of `classic` while the Tinta look is on, else `classic` itself.
pub fn tokens(classic: Tokens) -> Tokens {
    if on() { skin(classic) } else { classic }
}

/// The Tinta palette, light or dark: Tinta Correo's neutrals with the Tinta Calculo green.
struct Palette {
    accent: Color32,
    window: Color32,
    band: Color32,
    card: Color32,
    text: Color32,
    text_weak: Color32,
    border: Color32,
    on_accent: Color32,
}

const LIGHT: Palette = Palette {
    accent: Color32::from_rgb(0x46, 0x6E, 0x28),
    window: Color32::from_rgb(0xEE, 0xF1, 0xEA),
    band: Color32::from_rgb(0xDA, 0xE6, 0xCE),
    card: Color32::WHITE,
    text: Color32::from_rgb(0x1F, 0x24, 0x30),
    text_weak: Color32::from_rgb(0x58, 0x5F, 0x6B),
    border: Color32::from_rgb(0xE0, 0xE5, 0xDB),
    on_accent: Color32::WHITE,
};

const DARK: Palette = Palette {
    accent: Color32::from_rgb(0x8D, 0xBA, 0x63),
    window: Color32::from_rgb(0x18, 0x1B, 0x16),
    band: Color32::from_rgb(0x24, 0x30, 0x1C),
    card: Color32::from_rgb(0x25, 0x28, 0x24),
    text: Color32::from_rgb(0xEC, 0xE8, 0xDF),
    text_weak: Color32::from_rgb(0xB4, 0xB8, 0xAE),
    border: Color32::from_rgb(0x3B, 0x40, 0x38),
    on_accent: Color32::from_rgb(0x12, 0x16, 0x10),
};

/// `a` moved `share` (0 to 1) of the way to `b`.
pub fn mix(a: Color32, b: Color32, share: f32) -> Color32 {
    let s = share.clamp(0.0, 1.0);
    let ch = |x: u8, y: u8| (f32::from(x) + (f32::from(y) - f32::from(x)) * s).round().clamp(0.0, 255.0) as u8;
    Color32::from_rgb(ch(a.r(), b.r()), ch(a.g(), b.g()), ch(a.b(), b.b()))
}

/// The Tinta look of `classic`: the chrome takes the Tinta palette; the sheet's own colours, the
/// category icons, code syntax, notices and status colours stay as upstream chose them.
pub fn skin(classic: Tokens) -> Tokens {
    let p = if classic.dark { DARK } else { LIGHT };
    let dark = classic.dark;
    let ribbon = if dark { p.card } else { mix(p.window, p.card, 0.55) };
    Tokens {
        window: mix(p.window, p.band, 0.45),
        ribbon,
        ribbon_border: p.border,
        text: p.text,
        text_dim: p.text_weak,
        text_disabled: mix(p.text_weak, p.window, 0.5),
        accent: p.accent,
        accent_dark: if dark { mix(p.accent, Color32::BLACK, 0.2) } else { mix(p.accent, Color32::BLACK, 0.18) },
        accent_soft: mix(p.card, p.accent, if dark { 0.26 } else { 0.14 }),
        hover: mix(ribbon, p.accent, if dark { 0.16 } else { 0.10 }),
        pressed: mix(ribbon, p.accent, if dark { 0.26 } else { 0.16 }),
        separator: p.border,
        desk: if dark { mix(p.window, Color32::BLACK, 0.3) } else { mix(p.window, p.text_weak, 0.10) },
        header_bg: if dark { mix(p.card, p.window, 0.4) } else { mix(p.card, p.window, 0.6) },
        header_text: p.text_weak,
        header_line: mix(p.border, p.text_weak, 0.25),
        header_sel_bg: mix(p.card, p.accent, if dark { 0.28 } else { 0.18 }),
        header_sel_text: if dark { mix(p.accent, Color32::WHITE, 0.35) } else { mix(p.accent, Color32::BLACK, 0.2) },
        header_all_bg: mix(p.card, p.accent, if dark { 0.36 } else { 0.30 }),
        input_bg: if dark { mix(p.card, p.window, 0.5) } else { p.card },
        input_border: mix(p.text_weak, p.card, if dark { 0.3 } else { 0.25 }),
        tab_bar: mix(p.window, p.band, 0.45),
        tab_active: ribbon,
        status_bar: mix(p.window, p.band, 0.45),
        menu_bg: p.card,
        on_accent: p.on_accent,
        ..classic
    }
}

/// Tinta Correo's control shapes on egui's visuals for `t` ([`crate::theme::apply`] fills in the
/// colours first): rounder corners, buttons without a frame (a soft accent tint instead), fields
/// with a frame that turns to the accent under the pointer, softer shadows.
pub fn visuals(v: &mut Visuals, t: &Tokens) {
    if !on() {
        return;
    }
    let card = t.menu_bg;
    v.window_corner_radius = CornerRadius::same(12);
    v.menu_corner_radius = CornerRadius::same(8);
    v.window_stroke = Stroke::new(1.0, t.ribbon_border);
    v.faint_bg_color = mix(card, t.window, 0.5);
    let w = &mut v.widgets;
    w.inactive.weak_bg_fill = mix(card, t.accent, 0.06);
    w.hovered.bg_stroke = Stroke::new(1.0, t.accent);
    for st in [&mut w.noninteractive, &mut w.inactive, &mut w.hovered, &mut w.active, &mut w.open] {
        st.corner_radius = CornerRadius::same(6);
    }
    v.popup_shadow = egui::Shadow { offset: [0, 4], blur: 14, spread: 0, color: Color32::from_black_alpha(if t.dark { 100 } else { 32 }) };
    v.window_shadow = egui::Shadow { offset: [0, 6], blur: 24, spread: 0, color: Color32::from_black_alpha(if t.dark { 110 } else { 40 }) };
}

/// The setting's label in Options.
pub(crate) const LABEL: &str = "Tinta Office look";

/// [`LABEL`] in each interface language, by its code.
const LABELS: [(&str, &str); 22] = [
    ("es", "Aspecto de Tinta Office"),
    ("ca", "Aspecte de Tinta Office"),
    ("gl", "Aspecto de Tinta Office"),
    ("eu", "Tinta Office itxura"),
    ("el", "Εμφάνιση Tinta Office"),
    ("ja", "Tinta Office の外観"),
    ("zh", "Tinta Office 外观"),
    ("ko", "Tinta Office 모양"),
    ("ru", "Оформление Tinta Office"),
    ("pt", "Aparência do Tinta Office"),
    ("de", "Tinta-Office-Design"),
    ("cs", "Vzhled Tinta Office"),
    ("fr", "Apparence Tinta Office"),
    ("sr", "Izgled Tinta Office"),
    ("tr", "Tinta Office görünümü"),
    ("pl", "Wygląd Tinta Office"),
    ("it", "Aspetto Tinta Office"),
    ("uk", "Вигляд Tinta Office"),
    ("vi", "Giao diện Tinta Office"),
    ("nl", "Tinta Office-uiterlijk"),
    ("ro", "Aspect Tinta Office"),
    ("lt", "Tinta Office išvaizda"),
];

/// [`LABEL`] in the language with `code`, English when there is none.
pub(crate) fn label(code: &str) -> &'static str {
    LABELS.iter().find(|(c, _)| c.eq_ignore_ascii_case(code)).map_or(LABEL, |(_, l)| l)
}

/// `view.tintaLook`: `{on?: bool}` turns the Tinta look on or off, toggling without it.
pub(crate) fn command(ui: &mut crate::UiState, p: &serde_json::Value) -> Result<serde_json::Value, String> {
    match p.get("on") {
        None => ui.tinta_look = !ui.tinta_look,
        Some(serde_json::Value::Bool(b)) => ui.tinta_look = *b,
        Some(other) => return Err(format!("view.tintaLook takes on: true or false, not {other}")),
    }
    Ok(serde_json::json!({ "on": ui.tinta_look }))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Upstream's WCAG pairs run on the Tinta palette too (`theme::contrast_tests`); these are the
    /// Tinta look's own promises.
    #[test]
    fn the_sheet_keeps_upstreams_colours() {
        for classic in [Tokens::light(), Tokens::dark()] {
            let t = skin(classic);
            assert_eq!(
                (t.grid_bg, t.gridline, t.sel_border, t.sel_fill, t.sheet_ref_colors, t.cell_text, t.paper),
                (classic.grid_bg, classic.gridline, classic.sel_border, classic.sel_fill, classic.sheet_ref_colors, classic.cell_text, classic.paper)
            );
        }
    }

    #[test]
    fn the_setting_is_on_by_default_and_toggles() {
        let mut ui = crate::UiState::default();
        assert!(ui.tinta_look);
        assert_eq!(command(&mut ui, &serde_json::json!({})).unwrap()["on"], false);
        assert_eq!(command(&mut ui, &serde_json::json!({"on": true})).unwrap()["on"], true);
        assert!(command(&mut ui, &serde_json::json!({"on": "yes"})).is_err());
        let saved: crate::UiState = serde_json::from_str(r#"{"tintaLook": false}"#).unwrap();
        assert!(!saved.tinta_look);
    }

    #[test]
    fn every_interface_language_has_the_label() {
        for lang in crate::i18n::Language::for_picker() {
            if lang.code() != "en" {
                assert_ne!(label(lang.code()), LABEL, "{} has no {LABEL:?}", lang.code());
            }
        }
    }
}

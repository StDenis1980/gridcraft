# Tinta Calculo — Tinta Office roadmap

> **Status:** 2026-10-27 · approved by the owner (Denys) · **Revision history:** 2026-10-27 created; 2026-10-27 Spanish names for every function; 2026-10-27 embedded fonts for Cyrillic in PDF export; 2026-10-27 es-ES and uk-UA regional formats; 2026-10-27 Ukrainian formula entry; 2026-10-27 self-employed and family budget test workbooks; 2026-10-28 templates in New, regional chart labels.

Tinta Calculo is the Tinta Office build of GridCraft and replaces Microsoft Excel. Upstream keeps the
feature-parity roadmap ([ROADMAP.md](../ROADMAP.md), [roadmap.md](roadmap.md),
[gaps.md](gaps.md)); this page lists only what the fork does and which upstream gaps matter most
for users in Spain and Ukraine. It is a fork-only file, so upstream syncs never conflict on it.

## Where it stands (2026-10-27)

Ready for real work ≈ 50% (essentials user ≈ 63%, upstream estimate); 62 commits behind upstream.
504 of 523 Excel functions work. Spanish formula entry (`FormulaLocale::Es`) knows the Spanish name of every built-in function (514, `crates/formula/src/spanish.rs`), and with the
interface in Spanish, Catalan, Galician, Basque or Ukrainian the grid shows and accepts the region's
formats (`crates/core/src/region.rs`): decimal comma, `.` or space thousands, dd/mm/yyyy (dd.mm.yyyy
in Ukrainian), `1.234,50 €` and `грн`. Files, formulas, CSV and `TEXT()` stay en-US; chart axis and data
labels follow the region too. Ukrainian, Catalan, Galician and Basque formula entry
keeps English function names with `;` between arguments and the decimal comma
(`FormulaLocale::Semicolon`).
PDF export embeds a subset of a system TrueType font (Arial, Liberation, DejaVu…) for text the 14 standard
fonts cannot encode, so Ukrainian text prints and stays searchable. CJK still uses a non-embedded font,
and Arabic or Devanagari would need text shaping first.

## Priorities

| # | Task | Where | Stage |
|---|---|---|---|
| 1 | Spanish names for every function (BUSCARV, SUMAR.SI.CONJUNTO…) **done 2026-10-27**; dd/mm/yyyy, decimal comma, € and грн in display and entry **done 2026-10-27**; uk-UA formula entry (English names, `;` separators, decimal comma) **done 2026-10-27** | upstream (extend the existing table) | 1 |
| 2 | Embedded fonts in PDF export, so Cyrillic prints **done 2026-10-27** (system fonts, `crates/engine/src/pdf_fonts.rs`) | upstream | 1 |
| 3 | Test workbooks written from scratch: self-employed income and expenses, IVA and IRPF, a family budget; an .xlsx save and recalc test **done 2026-10-27** (`crates/engine/tests/it/tinta_workbooks.rs`) | fork (tests) | 1 |
| 4 | Those workbooks as templates in New (our text and colours), also course material: templates **done 2026-10-28** (`crates/engine/src/tinta_templates.rs`: Autónomo with IVA/IRPF and Modelos 303/130, family budget in Spanish and Ukrainian); course material still to do | fork | 2 |
| 5 | Old .xls and password-protected workbooks; .xlsx fidelity on real files without Excel's repair prompt | upstream | 2 |
| 6 | Charts: kinds and options survive a save (≈ 30% today) | upstream | 2 |
| 7 | Large workbooks (100 MB, 1M rows), PivotTables | upstream | 3 |
| 8 | Bank statement import, CSV and Norma 43 (an idea for self-employed users) | fork, if the owner wants it | 3 |

## Suite-wide work (all five Tinta Office apps)

Stages: **1** = November 2026 (Preview 3), **2** = December 2026 – January 2027 (public beta of
the most ready apps), **3** = 2027 (1.0). Dates are indicative.

| Work | Stage | What |
|---|---|---|
| Upstream sync | ongoing | Weekly, Mondays. Catch up before Preview 3. |
| Our changes upstream | 1 | Offer storytold the configurable brand, Presenta's translations, the codec feature flag and the ca/gl/eu catalogs (craft-i18n#53), so the fork stays small. |
| GitHub Actions | 1 | GitHub disabled Actions in pdfcraft, wordcraft and deckcraft; ask GitHub Support. Until then, run the gates locally. |
| Releases | 1 | A preview every month for all five apps, release notes in es/uk/en, one build script. |
| Linux | 1 | AppImage + .deb with Tinta names for all five (built in the cloud); Flatpak in stage 2. |
| Windows installers | 1 | Desktop shortcuts for Texto and Calculo; Presenta registered for .pptx. |
| Code signing | 2 | Windows (Certum Open Source or Azure Trusted Signing; scripts exist), macOS (Apple Developer + notarization). Owner decision and payment. |
| Automated tests | 1–2 | Coverage audit per app; tests for every fork change (brand, languages, installers); a smoke test of each release installer on Windows; investigate the flaky tests upstream. |
| User guide | 2 | A short "first steps" guide per app in Spanish and Ukrainian on tintaoffice.com (Correo has one). |
| Fork documentation | 1 | This file: what the fork changes, how to build a release, how to sync. |
| Translations | 1–2 | Ukrainian reviewed by the owner, Spanish by a native tester, ca/gl/eu through communities. Marked as drafts until reviewed. |
| Feedback | 1 | Help › Report a problem → the fork's issues or hola@tintaoffice.com; issue templates in Spanish and Ukrainian. |
| Error reports | 1 | Automatic, only with consent (decided 2026-10-30): the first start asks "Send error reports automatically?"; File › Options › General changes it, shows the installation id and deletes the reports from the server. Panics and a window busy for 15 s write a scrubbed report (no document text, file names, paths, e-mail addresses, long numbers) to `error-reports/pending` in the app's data folder; while allowed it goes to report.tintaoffice.com (tintaoffice-web `report-worker/`: R2 in the EU, 90 days, one GitHub issue per distinct crash labelled `auto-report`). Calculo: `crates/engine/src/tinta_reports.rs` (`file.errorReports`), `crates/ui-egui/src/tinta_reports_ui.rs` (strings translated there), `apps/gridcraft/src/tinta_report.rs`, address in `.cargo/config.toml` (`TINTA_REPORT_URL`). |

Rules: upstream first (fixes useful to everyone go to storytold as PRs); every change ships with
automated tests and updated docs in the same PR; files people exchange (.docx, .xlsx, .pptx, PDF)
come first; Spain and Ukraine first (UI language, spelling, number/date formats, national
certificates).

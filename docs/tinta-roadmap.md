# Tinta Calculo — Tinta Office roadmap

> **Status:** 2026-11-12 · approved by the owner (Denys) · **Revision history:** 2026-10-27 created; 2026-10-27 Spanish names for every function; 2026-10-27 embedded fonts for Cyrillic in PDF export; 2026-10-27 es-ES and uk-UA regional formats; 2026-10-27 Ukrainian formula entry; 2026-10-27 self-employed and family budget test workbooks; 2026-10-28 templates in New, regional chart labels; 2026-10-30 lesson and video for the Autónomo template; save with password; 2026-11-03 Flatpak bundle manifest under the Tinta id; 2026-11-03 plan revision 2 (owner-approved): Preview 3 and a closed beta first, stage gates, next steps per stage; 2026-11-03 owner decisions recorded: Preview 3 on 12 November, Certum signing from January, beta sign-up form on tintaoffice.com/beta; 2026-11-12 agent permissions for Vilna OS: risk per command, `--allow-commands`, repeated roots, name snapshots.

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
| 4 | Those workbooks as templates in New (our text and colours), also course material: templates **done 2026-10-28** (`crates/engine/src/tinta_templates.rs`: Autónomo with IVA/IRPF and Modelos 303/130, family budget in Spanish and Ukrainian); course material **done 2026-10-30** (a 45-minute lesson in Spanish and Ukrainian and a narrated video) | fork | 2 |
| 5 | Old .xls and password-protected workbooks; .xlsx fidelity on real files without Excel's repair prompt. Password-protected .xlsx **done 2026-10-30**: opens, saves back with its password, File › Info › Encrypt with Password; Excel-encrypted files no longer fail as damaged | upstream | 2 |
| 6 | Charts: kinds and options survive a save (≈ 30% today) | upstream | 2 |
| 7 | Large workbooks (100 MB, 1M rows), PivotTables | upstream | 3 |
| 8 | Bank statement import, CSV and Norma 43 (an idea for self-employed users) | fork, if the owner wants it | 3 |

## Plan revision 2 (2026-11-03, approved by the owner)

Since 2026-10-27 the suite shipped more than stage 1 planned, but no new build has reached people:
the last releases are from 2026-10-26/27. So November is about delivery, not new features.

Suite priorities for November (stage 1), in order:

1. **Preview 3 for all five apps on 12 November**, after the 9 November upstream sync: Windows MSI
   and portable, Linux .deb, AppImage and .flatpak, the extra dictionary pack as its own file,
   release notes in es/uk/en, the site's Download section; every installer smoke-tested on the
   owner's PC first.
2. **Closed beta in Spain** from Preview 3: a sign-up form on tintaoffice.com/beta and a guide
   for testers at /beta/testers/ (what to check, how to report), 20–50 autónomos and 1–2 gestorías, Ukrainians in Spain
   as testers; weekly triage of error reports and issues.
3. **Tinta Presenta catches up** (it has barely moved since 2026-10-27): #74, a .pptx test set, #70/#77.
4. **Tinta Correo finishes M5**: calendar and contacts from .pst, export, the Outlook migration guide.
5. **Translation review**: Ukrainian by the owner, Spanish by the first testers, ca/gl/eu through
   Softcatalà and Proxecto Trasno.
6. **Tests and stability**: a coverage audit of fork changes per app, an installer smoke-test script,
   fuzzing file opening before each release.
7. **A smaller fork**: port Texto's proofing to upstream's engine, see the open upstream PRs through
   (Calculo ×4, Texto #4952), ask GitHub Support to re-enable Actions.

Gates: stage 2 starts when Preview 3 is out and its installers passed the smoke test; the **public
beta (February 2027)** starts when the Windows installers are code-signed and the es/uk
translations are reviewed; 1.0 when no critical reports remain. Public beta: Tinta PDF, Texto and
Calculo; Presenta if its test set passes by January; Correo stays a preview without promotion.
Ukraine not before Q2 2027 (KEP), the rest of Europe in H2 2027; no paid ads or support sales
before the public beta.

This app's next steps:

| Stage | Next steps |
|---|---|
| 1 — November 2026 | Password-protected .xlsx, kept chart XML and the cs/de/fr/it/pl function names reach testers in Preview 3; reconcile our de/fr/it names with upstream's open PRs (#3020, #2655, #1894) at the next sync |
| 2 — December 2026 – January 2027 | Legacy .xls; the remaining chart options (axis bounds, error bars) once upstream merges #1290 and #2623 |
| 3 — 2027 | Large workbooks, PivotTables; Norma 43 bank statement import if the owner wants it |

Code signing for Windows, decided 2026-11-03: Certum Open Source, registered at the start of January
2027 once the owner has his new ID card (December); Preview 3 and the closed beta stay unsigned.
Certum's identity check takes days to two weeks, so signed installers are expected late January,
before the public beta in February.

Beta sign-up, decided 2026-11-03: a form on tintaoffice.com
([tintaoffice.com/beta](https://tintaoffice.com/beta/), with a testers' guide at `/beta/testers/`).

Preview 3 date, decided 2026-11-03: 12 November 2026.

Owner decisions still open: which apps enter the public beta, time
for the Ukrainian review, the video channel and playlists.

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
| Flatpak | 2 | Bundle manifest under the Tinta id (`packaging/linux/flatpak/io.github.stdenis1980.tintacalculo.bundle.yml`, generated from upstream's by `packaging/linux/tinta-flatpak.sh generate`; `tinta-flatpak.sh bundle` turns the Linux tarball into a `.flatpak`). Requested by Vilna OS (tintaoffice-web#14): bundles in releases by April 2027; Flathub only with Denys's approval. |
| Windows installers | 1 | Desktop shortcuts for Texto and Calculo; Presenta registered for .pptx. |
| Code signing | 2 | Windows: Certum Open Source (decided 2026-11-03), registration early January 2027, signed installers before the February public beta; scripts exist. macOS (Apple Developer + notarization) later. |
| Automated tests | 1–2 | Coverage audit per app; tests for every fork change (brand, languages, installers); a smoke test of each release installer on Windows; investigate the flaky tests upstream. |
| User guide | 2 | A short "first steps" guide per app in Spanish and Ukrainian on tintaoffice.com (Correo has one). |
| Fork documentation | 1 | This file: what the fork changes, how to build a release, how to sync. |
| Translations | 1–2 | Ukrainian reviewed by the owner, Spanish by a native tester, ca/gl/eu through communities. Marked as drafts until reviewed. |
| Feedback | 1 | Help › Report a problem → the fork's issues or hola@tintaoffice.com; issue templates in Spanish and Ukrainian. |
| Error reports | 1 | Automatic, only with consent (decided 2026-10-30): the first start asks "Send error reports automatically?"; File › Options › General changes it, shows the installation id and deletes the reports from the server. Panics and a window busy for 15 s write a scrubbed report (no document text, file names, paths, e-mail addresses, long numbers) to `error-reports/pending` in the app's data folder; while allowed it goes to report.tintaoffice.com (tintaoffice-web `report-worker/`: R2 in the EU, 90 days, one GitHub issue per distinct crash labelled `auto-report`). Calculo: `crates/engine/src/tinta_reports.rs` (`file.errorReports`), `crates/ui-egui/src/tinta_reports_ui.rs` (strings translated there), `apps/gridcraft/src/tinta_report.rs`, address in `.cargo/config.toml` (`TINTA_REPORT_URL`). A distribution can answer the first-start question for all five apps (2026-11-03, for Vilna OS, tintaoffice-web#16): `TINTA_OFFICE_REPORTS=true|false` (seen inside Flatpak too), or a line `enabled=true|false` in `$XDG_CONFIG_DIRS/tinta-office/reports.conf` (default `/etc/xdg`); read only while the person has not answered, saved as their answer, and changeable in the settings. |
| Agent permissions (Vilna OS Herkos) | 2 | For tintaoffice-web#26 and #33 (2026-11-12): every command and MCP tool says what it may do beyond the open document (`risk`: reads/changes the document, reads/writes disk, network, print, sign, other program, runs commands, UI only); `--allow-commands` confines a session; the automation root flags may be repeated; command ids and tool names are pinned by a snapshot test, so a rename is a deliberate, noted change. Calculo: `crates/engine/src/tinta_risk.rs`, snapshots `docs/tinta-commands.tsv` and `docs/tinta-mcp-tools.tsv`; repeated `--automation-read-root`/`--automation-write-root` (offer upstream). Next: check the MCP server inside the Flatpak sandbox with no home access and no network. |

Rules: upstream first (fixes useful to everyone go to storytold as PRs); every change ships with
automated tests and updated docs in the same PR; files people exchange (.docx, .xlsx, .pptx, PDF)
come first; Spain and Ukraine first (UI language, spelling, number/date formats, national
certificates).

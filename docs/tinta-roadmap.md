# Tinta Calculo — Tinta Office roadmap

> **Status:** 2026-10-27 · approved by the owner (Denys) · **Revision history:** 2026-10-27 created; 2026-10-27 Spanish names for every function.

Tinta Calculo is the Tinta Office build of GridCraft and replaces Microsoft Excel. Upstream keeps the
feature-parity roadmap ([ROADMAP.md](../ROADMAP.md), [roadmap.md](roadmap.md),
[gaps.md](gaps.md)); this page lists only what the fork does and which upstream gaps matter most
for users in Spain and Ukraine. It is a fork-only file, so upstream syncs never conflict on it.

## Where it stands (2026-10-27)

Ready for real work ≈ 50% (essentials user ≈ 63%, upstream estimate); 62 commits behind upstream.
504 of 523 Excel functions work. Spanish formula entry (`FormulaLocale::Es`) knows the Spanish name of every built-in function (514, `crates/formula/src/spanish.rs`), but number
and date formats are still en-US.
PDF export uses the 14 standard fonts without embedding, so **Ukrainian text does not reach the PDF**.

## Priorities

| # | Task | Where | Stage |
|---|---|---|---|
| 1 | Spanish names for every function (BUSCARV, SUMAR.SI.CONJUNTO…) **done 2026-10-27**; still to do: dd/mm/yyyy and € formats, and uk-UA (English names, decimal comma, hryvnia) | upstream (extend the existing table) | 1 |
| 2 | Embedded fonts in PDF export, so Cyrillic prints | upstream | 1 |
| 3 | Test workbooks written from scratch: self-employed income and expenses, IVA and IRPF, a family budget; an .xlsx save and recalc test | fork (tests) | 1 |
| 4 | Those workbooks as templates in New (our text and colours), also course material | fork | 2 |
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

Rules: upstream first (fixes useful to everyone go to storytold as PRs); every change ships with
automated tests and updated docs in the same PR; files people exchange (.docx, .xlsx, .pptx, PDF)
come first; Spain and Ukraine first (UI language, spelling, number/date formats, national
certificates).

# Formula language

View → Interface language is the existing, persisted language preference. Choose **Español**
for Spanish formula entry and display. On first run the preference follows the system language.
Choose **Čeština** for Czech formula entry (Tinta Office fork): Czech names as Czech Excel
shows them (`SUMA`, `KDYŽ`, `SVYHLEDAT`, `PRAVDA`/`NEPRAVDA`; Czech `LOG` is `LOG10` and `LOGZ`
is `LOG`), with the same separators as Spanish; functions Czech Excel keeps in English keep
their English names (`SUMIF`, `XLOOKUP`), and an English name typed with Czech separators is read
too. The tables live in `crates/formula/src/{spanish,czech}.rs`; the Czech one is AI-assisted
and not yet checked against Czech Excel. Choose **Deutsch** for German names (`SUMME`, `WENN`, `SVERWEIS`, Tinta Office fork; same separators,
AI-assisted table in `crates/formula/src/german.rs`); the interface also writes numbers
and dates the de-DE way.
Choose **Français** for French names (`SOMME`, `SI`, `RECHERCHEV`, Tinta Office fork; same separators,
AI-assisted table in `crates/formula/src/french.rs`); the interface also writes numbers
and dates the fr-FR way.
Choose **Italiano** for Italian names (`SOMMA`, `SE`, `CERCA.VERT`, Tinta Office fork; same separators,
AI-assisted table in `crates/formula/src/italian.rs`); the interface also writes numbers
and dates the it-IT way.
Choose **Polski** for Polish names (`SUMA`, `JEŻELI`, `WYSZUKAJ.PIONOWO`, Tinta Office fork; same separators,
AI-assisted table in `crates/formula/src/polish.rs`); the interface also writes numbers
and dates the pl-PL way.
Catalan, Galician, Basque, Ukrainian, Romanian and
Lithuanian use English names with `;` and the decimal comma; English and the other interface
languages use the English formula table and separators.

| Spanish | Canonical English |
| --- | --- |
| `SUMA` | `SUM` |
| `PROMEDIO` | `AVERAGE` |
| `SI` | `IF` |
| `CONTAR` | `COUNT` |
| `SUMAR.SI` | `SUMIF` |
| `CONTAR.SI` | `COUNTIF` |
| `BUSCARV` | `VLOOKUP` |
| `HOY` | `TODAY` |
| `VERDADERO`, `FALSO` | `TRUE`, `FALSE` (literals or function calls) |

Names are case-insensitive. Spanish formulas use decimal commas (`1,5`), semicolon arguments
and reference unions, and backslashes between array columns (`{1\2;3\4}`). For example,
`=SI(VERDADERO;SUMA(1,5;2,25);0)` evaluates to 3.75. English mode retains comma arguments,
decimal points and comma array columns; it does not silently translate Spanish aliases.

The cell editor, formula bar, Show Formulas, autocomplete, the Formula ribbon, Insert Function, and argument hints
use the selected formula language. Common Spanish function help is translated; functions
without a Spanish entry retain their English names/help. General interface labels and errors
fall back to English where Spanish translations are not provided. An edit captures its language
when it starts so switching the preference cannot reinterpret unfinished input.

Workbook/sheet names, table names, LET bindings and LAMBDA parameters take precedence over
aliases. Text, sheet names and table column names are unchanged. When a localized spelling
would collide with a name, displayed formulas retain the canonical spelling to preserve meaning.

Stored formulas, XLSX import/export, engine commands, control/API calls, CLI evaluation and
clipboard exchange retain strict English syntax. This also applies to advanced dialogs such as
Name Manager and conditional-format rules. Localization happens at the cell/formula-bar editor
boundary, including filling the selected range with Ctrl+Enter; it does not change the workbook
format or teach the calculation engine ambiguous aliases.

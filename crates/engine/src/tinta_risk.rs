//! What each command may do beyond the open document, for agents' permission layers
//! (tintaoffice-web#26, Vilna OS's Herkos): `list_commands`, `engine.commands` and
//! `gridcraft-cli commands --json` give every command a `risk` object.
//!
//! - `reads_document` / `changes_document`: the open workbook (`changes_document` is an undoable
//!   change; every command but a window-only one reads it).
//! - `reads_disk` / `writes_disk`: files outside the workbook in memory (open, pictures, CSV and
//!   data imports; save, export, the user dictionary).
//!   The automation roots still hold every such path.
//! - `network`: anything that leaves the device.
//! - `print`: the printer. `sign`: a certificate signature (none in Calculo).
//! - `other_program`: hands something to another program (a link to the browser).
//! - `runs_commands`: runs other commands (scripts, macros), each still held to `--allow-commands`.
//! - `ui_only`: only the window, not the workbook.
//!
//! The tables below name the commands with those effects. They are reviewed by hand: a new
//! command changes `docs/tinta-commands.tsv`, whose test (`tinta_risk::tests`) fails until the
//! snapshot is updated on purpose, which is the moment to add it here. A table entry that names
//! no command fails the test too, so a rename can't leave a stale entry behind.

use serde_json::{Value, json};

use crate::CommandSpec;

/// Commands that read files from disk (besides the workbook already open).
const READS_DISK: &[&str] = &["file.open", "insert.picture", "data.fromTextCsv", "data.getData", "pageLayout.background"];

/// Commands that write files.
const WRITES_DISK: &[&str] =
    &["file.save", "file.saveAs", "file.exportCsv", "file.exportHtml", "file.exportPdf", "file.print", "review.addToDictionary"];

/// Commands that may reach the network.
const NETWORK: &[&str] = &["file.errorReports"];

/// Commands that print (`file.print` hands the PDF to the system's printing).
const PRINT: &[&str] = &["file.print"];

/// Commands that sign with a certificate.
const SIGN: &[&str] = &[];

/// Commands that hand something to another program on the computer (a link to the browser or
/// mail program).
const OTHER_PROGRAM: &[&str] = &["link.open"];

/// Commands that run other commands: scripts, macros and the buttons that start them. Each step
/// is still held to `--allow-commands`.
const RUNS_COMMANDS: &[&str] = &["automate.runScript", "macro.run", "macro.debug", "control.press"];

/// Groups (`window.`) and ids whose commands only change the window, not the workbook.
const UI_ONLY: &[&str] = &[
    "window.",
    "view.zoom",
    "view.zoom100",
    "view.zoomToSelection",
    "view.formulaBar",
    "view.ruler",
    "view.newWindow",
    "view.arrangeAll",
    "view.hideWindow",
    "view.unhideWindow",
    "view.switchWindows",
];

fn named(table: &[&str], id: &str) -> bool {
    table.iter().any(|t| if t.ends_with('.') { id.starts_with(t) } else { *t == id })
}

/// The `risk` object of `spec`.
pub fn risk(spec: &CommandSpec) -> Value {
    let id = spec.id;
    let ui_only = named(UI_ONLY, id);
    json!({
        "reads_document": !ui_only,
        "changes_document": spec.undoable,
        "reads_disk": named(READS_DISK, id),
        "writes_disk": named(WRITES_DISK, id),
        "network": named(NETWORK, id),
        "print": named(PRINT, id),
        "sign": named(SIGN, id),
        "other_program": named(OTHER_PROGRAM, id),
        "runs_commands": named(RUNS_COMMANDS, id),
        "ui_only": ui_only,
    })
}

/// One line per command, sorted by id: the id, then the risk flags that are set
/// (`docs/tinta-commands.tsv`).
pub fn snapshot(specs: &[CommandSpec]) -> String {
    let mut lines: Vec<String> = specs
        .iter()
        .map(|spec| {
            let r = risk(spec);
            let set: Vec<&str> =
                r.as_object().map(|o| o.iter().filter(|(_, v)| v.as_bool() == Some(true)).map(|(k, _)| k.as_str()).collect()).unwrap_or_default();
            format!("{}\t{}", spec.id, set.join(","))
        })
        .collect();
    lines.sort();
    lines.dedup();
    let mut out = String::from(
        "# Tinta Calculo commands and what they may do (tinta_risk.rs). Generated: TINTA_UPDATE_SNAPSHOT=1 cargo test -p gridcraft-engine --lib tinta_risk\n",
    );
    for line in lines {
        out.push_str(&line);
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SNAPSHOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../docs/tinta-commands.tsv");

    fn specs() -> &'static [CommandSpec] {
        crate::command_specs()
    }

    /// Command ids are what agents' permission manifests name (tintaoffice-web#33): a command
    /// added, renamed or removed, or a risk flag changed, fails here until the snapshot is
    /// regenerated on purpose (and the change noted in the release notes).
    #[test]
    fn command_ids_and_risks_match_the_snapshot() {
        let now = snapshot(specs());
        if std::env::var_os("TINTA_UPDATE_SNAPSHOT").is_some() {
            std::fs::write(SNAPSHOT, &now).unwrap();
            return;
        }
        let saved = std::fs::read_to_string(SNAPSHOT).unwrap_or_default();
        if saved != now {
            let old: std::collections::BTreeSet<&str> = saved.lines().collect();
            let new: std::collections::BTreeSet<&str> = now.lines().collect();
            let gone: Vec<&&str> = old.difference(&new).collect();
            let added: Vec<&&str> = new.difference(&old).collect();
            panic!(
                "the commands changed: gone {gone:?}, new {added:?}. Review tinta_risk.rs for the new ones, then run \
                 TINTA_UPDATE_SNAPSHOT=1 cargo test -p gridcraft-engine --lib tinta_risk"
            );
        }
    }

    /// Every table entry names a command that exists.
    #[test]
    fn every_table_entry_names_a_command() {
        let ids: Vec<&str> = specs().iter().map(|s| s.id).collect();
        for table in [READS_DISK, WRITES_DISK, NETWORK, PRINT, SIGN, OTHER_PROGRAM, RUNS_COMMANDS, UI_ONLY] {
            for entry in table {
                assert!(ids.iter().any(|id| named(&[entry], id)), "{entry} names no command");
            }
        }
    }

    #[test]
    fn saving_printing_and_sending_are_flagged() {
        let all = specs();
        let flags = |id: &str| risk(all.iter().find(|s| s.id == id).unwrap());
        assert_eq!(flags("file.save")["writes_disk"], true);
        assert_eq!(flags("file.print")["print"], true);
        assert_eq!(flags("data.getData")["reads_disk"], true);
        assert_eq!(flags("file.errorReports")["network"], true);
        assert_eq!(flags("home.bold")["changes_document"], true);
        assert_eq!(flags("home.bold")["writes_disk"], false);
        assert_eq!(flags("macro.run")["runs_commands"], true);
        assert_eq!(flags("view.zoom")["ui_only"], true);
    }
}

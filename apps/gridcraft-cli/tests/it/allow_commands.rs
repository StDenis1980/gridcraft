//! `gridcraft-cli mcp` confinement flags at startup, through the real binary: a flag without its
//! value, an unknown command, an oversized or streamed `@FILE`, a repeated flag and `--connect`
//! all stop the server before it serves; a good list starts it held to that list.

use std::process::{Command, Output};

/// Run the CLI with `args`. `output()` gives the child no stdin, so a server that does start
/// ends at once instead of waiting for a client.
fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_gridcraft-cli")).args(args).output().unwrap()
}

/// The run must fail with `needle` on stderr, without ever having served.
fn refused(args: &[&str], needle: &str) {
    let out = cli(args);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success(), "{args:?} should not start: {stderr}");
    assert!(stderr.contains(needle), "{args:?}: expected `{needle}` in: {stderr}");
    assert!(!stderr.contains("MCP server on stdio"), "{args:?} served: {stderr}");
}

/// A flag without its value, or given twice, never starts the server.
#[test]
fn a_flag_without_its_value_never_starts_the_server() {
    refused(&["mcp", "--allow-commands"], "needs a value");
    refused(&["mcp", "--allow-commands="], "empty");
    refused(&["mcp", "--allow-commands", ""], "empty");
    refused(&["mcp", "--allow-commands", "--automation-read-root", "."], "needs a value");
    refused(&["mcp", "--allow-commands", "home.*", "--allow-commands", "cell.set"], "given twice");
}

/// `--connect` before or after the flag doesn't get past the checks.
#[test]
fn connect_is_not_a_way_around_the_checks() {
    refused(&["mcp", "--connect", "7979", "--allow-commands"], "needs a value");
    refused(&["mcp", "--allow-commands", "--connect", "7979"], "needs a value");
    refused(&["mcp", "--connect", "7979", "--allow-commands", "home.*"], "running app is not confined");
}

/// Unknown ids and malformed patterns stop the server at startup.
#[test]
fn unknown_commands_and_patterns_are_refused() {
    refused(&["mcp", "--allow-commands", "cell.sett"], "unknown command");
    refused(&["mcp", "--allow-commands", "nope.*"], "matches no command");
    refused(&["mcp", "--allow-commands", "*"], "--allow-commands");
    refused(&["mcp", "--allow-commands", "home*"], "--allow-commands");
}

/// An oversized `@FILE`, a folder, `/dev/zero` and a FIFO are refused without blocking.
#[test]
fn oversized_and_streamed_list_files_are_refused() {
    let dir = std::env::temp_dir().join(format!("gridcraft-it-allowlist-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let big = dir.join("big.txt");
    // Valid ids up front, so only the size can be the reason.
    let mut text = String::from("home.bold\n");
    text.push_str(&"#".repeat(256 * 1024));
    std::fs::write(&big, text).unwrap();
    refused(&["mcp", "--allow-commands", &format!("@{}", big.display())], "too long");
    refused(&["mcp", "--allow-commands", &format!("@{}", dir.display())], "not a regular file");
    #[cfg(unix)]
    {
        refused(&["mcp", "--allow-commands", "@/dev/zero"], "not a regular file");
        let fifo = dir.join("list");
        assert!(Command::new("mkfifo").arg(&fifo).status().is_ok_and(|s| s.success()), "mkfifo");
        refused(&["mcp", "--allow-commands", &format!("@{}", fifo.display())], "not a regular file");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// A good `@FILE` list starts the server held to it.
#[test]
fn a_good_list_starts_the_server_held_to_it() {
    let dir = std::env::temp_dir().join(format!("gridcraft-it-allowlist-ok-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("list.txt"), "# what the agent may do\nhome.bold\ncell.set, cell.get\n").unwrap();
    let out = cli(&["mcp", "--allow-commands", &format!("@{}", dir.join("list.txt").display())]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(out.status.success(), "{stderr}");
    assert!(stderr.contains("commands: cell.get, cell.set, home.bold"), "{stderr}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// The roots may be repeated, one flag per folder (tintaoffice-web#33): the server starts held to
/// all of them; a missing folder anywhere in the list stops it.
#[test]
fn repeated_root_flags_start_the_server_with_every_root() {
    let dir = std::env::temp_dir().join(format!("gridcraft-it-roots-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    for d in ["a", "b"] {
        std::fs::create_dir_all(dir.join(d)).unwrap();
    }
    let (a, b) = (dir.join("a").display().to_string(), dir.join("b").display().to_string());
    let out = cli(&["mcp", "--automation-read-root", &a, "--automation-read-root", &b, "--automation-write-root", &b]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(out.status.success(), "{stderr}");
    assert!(stderr.contains(&format!("read roots {a}, {b}, write roots {b}")), "{stderr}");
    refused(&["mcp", "--automation-read-root", &a, "--automation-read-root", &dir.join("missing").display().to_string()], "read root");
    let _ = std::fs::remove_dir_all(&dir);
}

//! What this build calls itself and where it sends people. The release is GridCraft by the ArtCraft
//! team; a redistributed build of the code (MIT OR Apache-2.0) must not use the ArtCraft marks
//! (`docs/brand/LICENSE-brand.txt`), so it sets its own name and addresses at compile time:
//!
//! ```sh
//! GRIDCRAFT_BRAND_NAME="Acme Sheets" GRIDCRAFT_BRAND_APP_ID=com.example.acmesheets \
//!   GRIDCRAFT_BRAND_WEBSITE=https://example.com GRIDCRAFT_BRAND_GITHUB=https://github.com/acme/sheets \
//!   cargo build --release -p gridcraft
//! ```
//!
//! or once, for every build of a fork, in `.cargo/config.toml`:
//!
//! ```toml
//! [env]
//! GRIDCRAFT_BRAND_NAME = "Acme Sheets"
//! ```
//!
//! Unset, every value is the release's and nothing changes. Setting a name makes the build
//! [`REBRANDED`]: the ArtCraft logo, Discord and website links are left out, and the name replaces
//! "GridCraft" in the interface text ([`rebrand`]). Identifiers that files and other programs rely
//! on (command ids, the `gridcraft` MCP server name, `GRIDCRAFT_*` variables, `gridcraft-cli`) stay as they are, so
//! documents and scripts work across builds.
//!
//! L0: every layer that writes the name (workbook metadata, PDF export, the shell) reads it from
//! here.

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use std::borrow::Cow;
use std::sync::OnceLock;

/// The release's name, as written in interface text and documents.
pub const UPSTREAM_NAME: &str = "GridCraft";

/// The release's application id (Wayland app id, Linux desktop entry, macOS bundle id).
pub const UPSTREAM_APP_ID: &str = "ai.storyteller.gridcraft";

/// `option_env!` with a default, usable in a `const`. A blank value counts as unset.
macro_rules! env_or {
    ($var:literal, $default:expr) => {
        match option_env!($var) {
            Some(v) if !v.is_empty() => v,
            _ => $default,
        }
    };
}

/// The product name shown to people and written into documents (`GRIDCRAFT_BRAND_NAME`).
pub const NAME: &str = env_or!("GRIDCRAFT_BRAND_NAME", UPSTREAM_NAME);

/// The lowercase, space-free name for folders and sockets on Linux and BSD (XDG), such as
/// `~/.config/gridcraft` (`GRIDCRAFT_BRAND_SLUG`). Folders elsewhere use [`NAME`]. A rebranded build
/// keeps its settings, recovery files and running instance apart from the release's.
pub const SLUG: &str = env_or!("GRIDCRAFT_BRAND_SLUG", "gridcraft");

/// Reverse-DNS application id (`GRIDCRAFT_BRAND_APP_ID`).
pub const APP_ID: &str = env_or!("GRIDCRAFT_BRAND_APP_ID", UPSTREAM_APP_ID);

/// Who publishes this build, when that is someone other than the GridCraft contributors, for the
/// About box and the Windows file properties (`GRIDCRAFT_BRAND_PUBLISHER`).
pub const PUBLISHER: Option<&str> = match option_env!("GRIDCRAFT_BRAND_PUBLISHER") {
    Some(v) if !v.is_empty() => Some(v),
    _ => None,
};

/// The product's web page (`GRIDCRAFT_BRAND_WEBSITE`).
pub const APP_PAGE: &str = env_or!("GRIDCRAFT_BRAND_WEBSITE", "https://getartcraft.com/apps/gridcraft");

/// The source repository; its GitHub releases are what the update check reads
/// (`GRIDCRAFT_BRAND_GITHUB`).
pub const GITHUB: &str = env_or!("GRIDCRAFT_BRAND_GITHUB", "https://github.com/storytold/gridcraft");

/// Where people get help (`GRIDCRAFT_BRAND_SUPPORT`). The release sends them to the ArtCraft
/// Discord; a rebranded build without its own support page sends them to its web page.
pub const SUPPORT: &str = env_or!("GRIDCRAFT_BRAND_SUPPORT", if REBRANDED { APP_PAGE } else { "https://discord.gg/artcraft" });

/// Whether this build carries a name of its own. The ArtCraft marks and community links are only
/// for the release (`docs/brand/LICENSE-brand.txt`).
pub const REBRANDED: bool = match option_env!("GRIDCRAFT_BRAND_NAME") {
    Some(v) => !v.is_empty(),
    None => false,
};

/// Where this build's releases are listed: [`GITHUB`]`/releases`.
pub fn releases_page() -> &'static str {
    static PAGE: OnceLock<String> = OnceLock::new();
    PAGE.get_or_init(|| format!("{}/releases", GITHUB.trim_end_matches('/')))
}

/// This build's GitHub `owner/repo`, the repository its update check reads releases from, or
/// `None` when [`GITHUB`] is not a `https://github.com/owner/repo` address.
pub fn github_repo() -> Option<&'static str> {
    let repo = GITHUB.strip_prefix("https://github.com/")?.trim_end_matches('/');
    let mut parts = repo.split('/');
    let (Some(owner), Some(name), None) = (parts.next(), parts.next(), parts.next()) else { return None };
    let plain = |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'));
    (plain(owner) && plain(name)).then_some(repo)
}

/// `text` with the release's name replaced by this build's. Borrowed, and unchanged, in the
/// release or when the text doesn't name it.
pub fn rebrand(text: &str) -> Cow<'_, str> {
    replace_name(text, NAME)
}

fn replace_name<'a>(text: &'a str, name: &str) -> Cow<'a, str> {
    if name == UPSTREAM_NAME || !text.contains(UPSTREAM_NAME) { Cow::Borrowed(text) } else { Cow::Owned(text.replace(UPSTREAM_NAME, name)) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_release_keeps_its_name_and_text() {
        assert!(matches!(replace_name("About GridCraft", UPSTREAM_NAME), Cow::Borrowed("About GridCraft")));
    }

    #[test]
    fn a_rebranded_build_replaces_every_mention_and_only_those() {
        assert_eq!(replace_name("About GridCraft", "Acme Sheets"), "About Acme Sheets");
        assert_eq!(replace_name("GridCraft {v}: GridCraft", "Acme"), "Acme {v}: Acme");
        assert!(matches!(replace_name("Open a file", "Acme"), Cow::Borrowed("Open a file")));
        // Other names that contain "Craft" are not this product's.
        assert_eq!(replace_name("ArtCraft and PhotoCraft", "Acme"), "ArtCraft and PhotoCraft");
    }

    #[test]
    fn the_constants_agree() {
        assert_eq!(REBRANDED, option_env!("GRIDCRAFT_BRAND_NAME").is_some_and(|v| !v.is_empty()));
        assert!(!NAME.is_empty() && !APP_ID.is_empty());
        assert!(APP_PAGE.starts_with("https://") && GITHUB.starts_with("https://") && SUPPORT.starts_with("https://"));
        assert_eq!(releases_page(), format!("{GITHUB}/releases"));
        if !REBRANDED {
            assert_eq!(github_repo(), Some("storytold/gridcraft"));
        }
    }
}

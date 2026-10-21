//! Project links (shown in Help, About, the CLI and MCP).
//!
//! This build is a fork of `storytold/deckcraft`. It is not affiliated with or endorsed by
//! ArtCraft, and it links only to its own repository.

pub const WEBSITE: &str = "https://github.com/685ks/deckcraft";
/// The Arthavan Projects launcher, which installs and updates the Craft apps.
pub const APPS: &str = "https://github.com/685ks/craft-launcher";
pub const APP_PAGE: &str = "https://github.com/685ks/deckcraft";
pub const GITHUB: &str = "https://github.com/685ks/deckcraft";
pub const ISSUES: &str = "https://github.com/685ks/deckcraft/issues";
pub const RELEASES: &str = "https://github.com/685ks/deckcraft/releases";
/// The project this one is forked from (credited in About; never contacted).
pub const UPSTREAM: &str = "https://github.com/storytold/deckcraft";
/// One line for About, the README and `--version` style output.
pub const FORK_NOTE: &str = "A fork of storytold/deckcraft. Not affiliated with or endorsed by ArtCraft.";

/// Set to `1` by the Arthavan Projects launcher on the apps it starts.
pub const LAUNCHER_ENV: &str = "ARTHAVAN_LAUNCHER";

/// What About says about updates. DeckCraft never checks for updates over the network: started
/// by the launcher, the launcher installs them; otherwise the releases page has them.
pub const UPDATES_BY_LAUNCHER: &str = "Updates are managed by Arthavan Projects";
pub const UPDATES_MANUAL: &str = "New versions are published on GitHub Releases";

/// True when the Arthavan Projects launcher started this process.
pub fn launched_by_launcher() -> bool {
    launcher_flag(std::env::var(LAUNCHER_ENV).ok().as_deref())
}

/// [`launched_by_launcher`] for a given value of [`LAUNCHER_ENV`].
pub fn launcher_flag(value: Option<&str>) -> bool {
    value.is_some_and(|v| v.trim() == "1")
}

/// The update line for About and the File tab.
pub fn update_note(by_launcher: bool) -> &'static str {
    if by_launcher { UPDATES_BY_LAUNCHER } else { UPDATES_MANUAL }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_launcher_flag_is_exactly_one() {
        assert!(launcher_flag(Some("1")));
        assert!(!launcher_flag(Some("0")));
        assert!(!launcher_flag(Some("")));
        assert!(!launcher_flag(None));
        assert_eq!(update_note(true), "Updates are managed by Arthavan Projects");
        assert_eq!(update_note(false), UPDATES_MANUAL);
    }

    #[test]
    fn links_stay_on_the_fork() {
        for link in [WEBSITE, APPS, APP_PAGE, GITHUB, ISSUES, RELEASES] {
            assert!(link.starts_with("https://github.com/685ks/"), "{link}");
        }
    }
}

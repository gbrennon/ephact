use std::{fmt, str::FromStr};

/// Selects the keyboard bindings used to navigate the terminal user interface.
///
/// Each mode has a distinct movement vocabulary; choose the mode that matches
/// the user's preferred interaction style.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NavigationMode {
    /// The default mode, with arrow keys and h/j/k/l movement aliases.
    ///
    /// PageUp and PageDown move between pages.
    #[default]
    Standard,
    /// Vim-style navigation: h/j/k/l move, gg jumps to the top, G to the
    /// bottom, and Ctrl-d/Ctrl-u page down/up.
    ///
    /// Arrow keys are not mapped in this mode.
    Vim,
    /// Emacs-style navigation: Ctrl-f/Ctrl-b move right/left,
    /// Ctrl-n/Ctrl-p move next/previous, Ctrl-a/Ctrl-e move top/bottom, and
    /// Ctrl-v/Meta-v page down/up.
    ///
    /// Arrow keys are not mapped in this mode.
    Emacs,
}

impl NavigationMode {
    pub const ALL: [Self; 3] = [Self::Standard, Self::Vim, Self::Emacs];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Vim => "vim",
            Self::Emacs => "emacs",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Standard => "Standard",
            Self::Vim => "Vim",
            Self::Emacs => "Emacs",
        }
    }
}

impl fmt::Display for NavigationMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for NavigationMode {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "standard" => Ok(Self::Standard),
            "vim" => Ok(Self::Vim),
            "emacs" => Ok(Self::Emacs),
            _ => Err(format!("unknown navigation mode: {value}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_standard() {
        assert_eq!(NavigationMode::default(), NavigationMode::Standard);
    }

    #[test]
    fn parses_and_formats_supported_modes() {
        for mode in NavigationMode::ALL {
            assert_eq!(mode.as_str().parse::<NavigationMode>(), Ok(mode));
            assert_eq!(mode.to_string(), mode.as_str());
            assert!(!mode.label().is_empty());
        }
    }
}

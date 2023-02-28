use crate::defaults::DEFAULT_FG_SYMBOL;

#[derive(Debug, Clone)]
pub struct ProgressBarStyle {
    fg_symbol: char,
    bg_symbol: Option<char>,
}

impl Default for ProgressBarStyle {
    fn default() -> Self {
        Self {
            fg_symbol: DEFAULT_FG_SYMBOL,
            bg_symbol: None,
        }
    }
}

impl ProgressBarStyle {
    pub fn get_fg_symbol(&self) -> char {
        self.fg_symbol
    }

    pub fn get_bg_symbol(&self) -> Option<char> {
        self.bg_symbol
    }
}

/// Used to style a progress bar.
///
/// # Example Usage
/// ```
/// use progressed::{UnboundProgressBar, UnboundProgressBarStyle};
///
/// let style = UnboundProgressBarStyle::default();
/// let bar = UnboundProgressBar::new(0..).set_style(style);
/// ```
pub struct UnboundProgressBarStyle {
    fg_symbol: char,
    tip_symbol: char,
}

impl Default for UnboundProgressBarStyle {
    fn default() -> Self {
        Self {
            fg_symbol: '=',
            tip_symbol: '>',
        }
    }
}

impl UnboundProgressBarStyle {
    pub fn get_fg_symbol(&self) -> char {
        self.fg_symbol
    }

    pub fn set_fg_symbol(mut self, fg_symbol: char) -> Self {
        self.fg_symbol = fg_symbol;
        self
    }

    pub fn get_tip_symbol(&self) -> char {
        self.tip_symbol
    }

    pub fn set_tip_symbol(mut self, tip_symbol: char) -> Self {
        self.tip_symbol = tip_symbol;
        self
    }
}

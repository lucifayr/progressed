pub struct UnboundProgressBarStyle {
    fg: char,
    tip: char,
}

impl Default for UnboundProgressBarStyle {
    fn default() -> Self {
        Self { fg: '=', tip: '>' }
    }
}

impl UnboundProgressBarStyle {
    pub fn get_fg(&self) -> char {
        self.fg
    }

    pub fn set_fg(mut self, fg: char) -> Self {
        self.fg = fg;
        self
    }

    pub fn get_tip(&self) -> char {
        self.tip
    }

    pub fn set_tip(mut self, tip: char) -> Self {
        self.tip = tip;
        self
    }
}

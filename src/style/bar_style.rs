use crate::defaults::{
    DEFAULT_BAR_SURROUND, DEFAULT_BG_SYMBOL, DEFAULT_COUNTER_SURROUND, DEFAULT_FG_SYMBOL,
    DEFAULT_TIP_SYMBOL,
};

#[derive(Debug, Clone)]
pub struct ProgressBarStyle {
    fg_symbol: char,
    bg_symbol: char,
    tip_symbol: char,
    show_counter: bool,
    counter_surround: (char, char),
    bar_surround: (char, char),
}

impl Default for ProgressBarStyle {
    fn default() -> Self {
        Self {
            fg_symbol: DEFAULT_FG_SYMBOL,
            bg_symbol: DEFAULT_BG_SYMBOL,
            tip_symbol: DEFAULT_TIP_SYMBOL,
            show_counter: false,
            counter_surround: DEFAULT_COUNTER_SURROUND,
            bar_surround: DEFAULT_BAR_SURROUND,
        }
    }
}

impl ProgressBarStyle {
    pub fn arch() -> Self {
        Self {
            fg_symbol: '#',
            bg_symbol: '-',
            tip_symbol: DEFAULT_TIP_SYMBOL,
            show_counter: true,
            counter_surround: (' ', ' '),
            bar_surround: ('[', ']'),
        }
    }

    pub fn cargo() -> Self {
        Self {
            fg_symbol: '=',
            bg_symbol: ' ',
            tip_symbol: '>',
            show_counter: false,
            counter_surround: DEFAULT_COUNTER_SURROUND,
            bar_surround: ('[', ']'),
        }
    }

    pub fn build(&mut self) -> Self {
        // TODO: This is inefficient
        self.clone()
    }

    pub fn get_fg_symbol(&self) -> char {
        self.fg_symbol
    }

    pub fn set_fg_symbol(&mut self, fg_symbol: char) -> &mut Self {
        self.fg_symbol = fg_symbol;
        self
    }

    pub fn get_bg_symbol(&self) -> char {
        self.bg_symbol
    }

    pub fn set_bg_symbol(&mut self, bg_symbol: char) -> &mut Self {
        self.bg_symbol = bg_symbol;
        self
    }

    pub fn get_tip_symbol(&self) -> char {
        self.tip_symbol
    }

    pub fn set_tip_symbol(&mut self, tip_symbol: char) -> &mut Self {
        self.tip_symbol = tip_symbol;
        self
    }

    pub fn get_show_counter(&self) -> bool {
        self.show_counter
    }

    pub fn set_show_counter(&mut self, show_counter: bool) -> &mut Self {
        self.show_counter = show_counter;
        self
    }

    pub fn get_counter_surround(&self) -> (char, char) {
        self.counter_surround
    }

    pub fn set_counter_surround(&mut self, counter_surround: (char, char)) -> &mut Self {
        self.counter_surround = counter_surround;
        self
    }

    pub fn get_bar_surround(&self) -> (char, char) {
        self.bar_surround
    }

    pub fn set_bar_surround(&mut self, bar_surround: (char, char)) -> &mut Self {
        self.bar_surround = bar_surround;
        self
    }
}

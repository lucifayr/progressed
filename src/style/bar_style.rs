use crate::{
    defaults::{
        DEFAULT_BAR_SURROUND, DEFAULT_BG_SYMBOL, DEFAULT_COUNTER_SURROUND, DEFAULT_FG_SYMBOL,
        DEFAULT_TIP_SYMBOL,
    },
    symbols::blocks::FULL_BLOCK,
};

use super::layout::ProgressBarLayout;

/// Used to style a progress bar.
///
/// ```
/// use progressed::{ProgressBar, ProgressBarStyle};
///
/// let style = ProgressBarStyle::default()
///     .set_fg_symbol('=')
///     .set_show_time(true);
///
/// let bar = ProgressBar::new(0..100).set_style(style);
/// ```
///
/// ## Styles
/// There a few defaults styles that are modeled after common progress bars.
///
/// ### Default
/// (25/50) |########################                       |  50%
///
/// ### Arch
/// 00:00:00 [######----------------------------------------]  12%
///
///
/// ### Cargo
/// [====================================>                       ]
///
///
/// ### Smooth
/// 86% |██████████████████████████████████████▉        | 00:00:02
#[derive(Debug, Clone)]
pub struct ProgressBarStyle {
    fg_symbol: char,
    bg_symbol: char,
    tip_symbol: char,
    smooth: bool,
    show_time: bool,
    show_counter: bool,
    show_percentage: bool,
    counter_surround: (char, char),
    bar_surround: (char, char),
    layout: ProgressBarLayout,
}

impl Default for ProgressBarStyle {
    fn default() -> Self {
        Self {
            fg_symbol: DEFAULT_FG_SYMBOL,
            bg_symbol: DEFAULT_BG_SYMBOL,
            tip_symbol: DEFAULT_TIP_SYMBOL,
            smooth: false,
            show_time: false,
            show_counter: true,
            show_percentage: true,
            counter_surround: DEFAULT_COUNTER_SURROUND,
            bar_surround: DEFAULT_BAR_SURROUND,
            layout: ProgressBarLayout::PercentageRight,
        }
    }
}

impl ProgressBarStyle {
    pub fn smooth() -> Self {
        Self {
            fg_symbol: FULL_BLOCK,
            bg_symbol: ' ',
            tip_symbol: ' ',
            smooth: true,
            show_time: true,
            show_counter: false,
            show_percentage: true,
            counter_surround: (' ', ' '),
            bar_surround: ('|', '|'),
            layout: ProgressBarLayout::TimerRight,
        }
    }

    pub fn arch() -> Self {
        Self {
            fg_symbol: '#',
            bg_symbol: '-',
            tip_symbol: '-',
            smooth: false,
            show_time: true,
            show_counter: false,
            show_percentage: true,
            counter_surround: (' ', ' '),
            bar_surround: ('[', ']'),
            layout: ProgressBarLayout::PercentageRight,
        }
    }

    pub fn cargo() -> Self {
        Self {
            fg_symbol: '=',
            bg_symbol: ' ',
            tip_symbol: '>',
            smooth: false,
            show_time: false,
            show_counter: false,
            show_percentage: false,
            counter_surround: DEFAULT_COUNTER_SURROUND,
            bar_surround: ('[', ']'),
            layout: ProgressBarLayout::PercentageRight,
        }
    }

    pub fn get_fg_symbol(&self) -> char {
        self.fg_symbol
    }

    pub fn set_fg_symbol(mut self, fg_symbol: char) -> Self {
        self.fg_symbol = fg_symbol;
        self
    }

    pub fn get_bg_symbol(&self) -> char {
        self.bg_symbol
    }

    pub fn set_bg_symbol(mut self, bg_symbol: char) -> Self {
        self.bg_symbol = bg_symbol;
        self
    }

    pub fn get_tip_symbol(&self) -> char {
        self.tip_symbol
    }

    pub fn set_tip_symbol(mut self, tip_symbol: char) -> Self {
        self.tip_symbol = tip_symbol;
        self
    }

    pub fn get_is_smooth(&self) -> bool {
        self.smooth
    }

    pub fn set_is_smooth(mut self, is_smooth: bool) -> Self {
        self.smooth = is_smooth;
        self
    }

    pub fn get_show_time(&self) -> bool {
        self.show_time
    }

    pub fn set_show_time(mut self, show_time: bool) -> Self {
        self.show_time = show_time;
        self
    }

    pub fn get_show_counter(&self) -> bool {
        self.show_counter
    }

    pub fn set_show_counter(mut self, show_counter: bool) -> Self {
        self.show_counter = show_counter;
        self
    }

    pub fn get_show_percentage(&self) -> bool {
        self.show_percentage
    }

    pub fn set_show_percentage(mut self, show_percentage: bool) -> Self {
        self.show_percentage = show_percentage;
        self
    }

    pub fn get_counter_surround(&self) -> (char, char) {
        self.counter_surround
    }

    pub fn set_counter_surround(mut self, counter_surround: (char, char)) -> Self {
        self.counter_surround = counter_surround;
        self
    }

    pub fn get_bar_surround(&self) -> (char, char) {
        self.bar_surround
    }

    pub fn set_bar_surround(mut self, bar_surround: (char, char)) -> Self {
        self.bar_surround = bar_surround;
        self
    }

    pub fn get_layout(&self) -> ProgressBarLayout {
        self.layout
    }

    pub fn set_layout(mut self, layout: ProgressBarLayout) -> Self {
        self.layout = layout;
        self
    }
}

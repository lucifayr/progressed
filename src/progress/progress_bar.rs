use std::{io::stdout, time::Instant};

use crossterm::ExecutableCommand;

use crate::{
    defaults::DEFAULT_WIDTH,
    style::{ProgressBarLayout, ProgressBarStyle},
    symbols::blocks::get_block_by_progress,
};

/// Draws a progress bar while iterating over a finite range.
/// ```no_run
///   use progressed::ProgressBar;
///
///   let bar = ProgressBar::new(0..100).set_title("Running Job: ");
///   for _ in bar {
///       // prints progress bar
///   }
///
/// ```
pub struct ProgressBar<I: ExactSizeIterator> {
    data: I,
    current_index: usize,
    max_len: usize,
    width: usize,
    title: String,
    start_time: Instant,
    style: ProgressBarStyle,
    start_pos: (u16, u16),
}

impl<I: ExactSizeIterator> ProgressBar<I> {
    pub fn new(iter: I) -> Self {
        let len = iter.len();
        let width = if let Ok((w, _)) = crossterm::terminal::size() {
            (w / 2) as usize
        } else {
            DEFAULT_WIDTH
        };

        let start_pos = crossterm::cursor::position().unwrap_or((0, 0));
        Self {
            data: iter,
            current_index: 0,
            max_len: len,
            width,
            title: String::new(),
            start_time: Instant::now(),
            style: ProgressBarStyle::default(),
            start_pos,
        }
    }
}

fn draw_counter(surround: (char, char), index: usize, len: usize, show: bool) -> String {
    if !show {
        return String::new();
    }

    let surround_left = surround.0;
    let surround_right = surround.1;
    let max_str_width = len.to_string().len();

    format!(" {surround_left}{index:max_str_width$}/{len}{surround_right} ")
}

fn draw_percentage(progress: f64, show: bool) -> String {
    if !show {
        return String::new();
    }

    format!(" {percent:3.0}% ", percent = progress * 100.0)
}

fn draw_time(start_time: Instant, show: bool) -> String {
    if !show {
        return String::new();
    }

    let duration = start_time.elapsed();
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let hours = (duration.as_secs() / 60) / 60;

    format!(" {:02}:{:02}:{:02} ", hours, minutes, seconds)
}

impl<I: ExactSizeIterator> Iterator for ProgressBar<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let progress = self.current_index as f64 / self.max_len as f64;
        let str_len = (progress * self.width as f64) as usize;

        let counter = draw_counter(
            self.style.get_counter_surround(),
            self.current_index,
            self.data.len(),
            self.style.get_show_counter(),
        );

        let percentage = draw_percentage(progress, self.style.get_show_percentage());
        let time = draw_time(self.start_time, self.style.get_show_time());

        let title = &self.title;
        let surround_left = self.style.get_bar_surround().0;
        let surround_right = self.style.get_bar_surround().1;
        let fg_symbol = self.style.get_fg_symbol().to_string();
        let bg_symbol = self.style.get_bg_symbol().to_string();
        let fg = vec![fg_symbol.clone(); str_len].join("");
        let bg = vec![bg_symbol; self.width - str_len].join("");

        let tip = if str_len != self.width && str_len != 0 {
            self.style.get_tip_symbol().to_string()
        } else {
            fg_symbol
        };

        let tip = if self.style.get_is_smooth() {
            if str_len != self.width {}
            let tip_decimal = (progress * self.width as f64) % 1.0;

            format!("{}", get_block_by_progress(tip_decimal))
        } else {
            tip
        };

        if stdout().execute(crossterm::cursor::Hide).is_ok() {}

        let (x, y) = self.start_pos;
        if let Ok(_) = stdout().execute(crossterm::cursor::MoveTo(x, y)) {}

        match self.style.get_layout() {
            ProgressBarLayout::AllRight => {
                print!("{title}{surround_left}{fg}{tip}{bg}{surround_right}{counter}{percentage}{time}");
            }
            ProgressBarLayout::AllLeft => {
                print!("{title}{counter}{percentage}{time}{surround_left}{fg}{tip}{bg}{surround_right}");
            }
            ProgressBarLayout::TimerRight => {
                print!("{title}{counter}{percentage}{surround_left}{fg}{tip}{bg}{surround_right}{time}");
            }
            ProgressBarLayout::CounterRight => {
                print!("{title}{percentage}{time}{surround_left}{fg}{tip}{bg}{surround_right}{counter}");
            }
            ProgressBarLayout::PercentageRight => {
                print!("{title}{counter}{time}{surround_left}{fg}{tip}{bg}{surround_right}{percentage}");
            }
            ProgressBarLayout::TimerAndCounterRight => {
                print!("{title}{percentage}{surround_left}{fg}{tip}{bg}{surround_right}{counter}{time}");
            }
            ProgressBarLayout::TimerAndPercentageRight => {
                print!("{title}{counter}{surround_left}{fg}{tip}{bg}{surround_right}{percentage}{time}");
            }
            ProgressBarLayout::CounterAndPercentageRight => {
                print!("{title}{time}{surround_left}{fg}{tip}{bg}{surround_right}{counter}{percentage}");
            }
        }

        if str_len == self.width {
            print!("\n");
            if stdout().execute(crossterm::cursor::Show).is_ok() {}
        }
        self.current_index += 1;
        self.data.next()
    }
}

impl<I: ExactSizeIterator> ProgressBar<I> {
    pub fn set_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn set_style(mut self, style: ProgressBarStyle) -> Self {
        self.style = style;
        self
    }

    pub fn set_title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        self
    }
}

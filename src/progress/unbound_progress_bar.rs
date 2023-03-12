use std::io::stdout;

use crossterm::ExecutableCommand;

use crate::{defaults::DEFAULT_WIDTH, style::UnboundProgressBarStyle};

pub struct UnboundProgressBar<I: Iterator> {
    data: I,
    current_index: usize,
    title: String,
    style: UnboundProgressBarStyle,
    max_width: usize,
    start_pos: (u16, u16),
}

impl<I: Iterator> UnboundProgressBar<I> {
    pub fn new(iter: I) -> Self {
        let start_pos = crossterm::cursor::position().unwrap_or((0, 0));
        let width = if let Ok((w, _)) = crossterm::terminal::size() {
            (w) as usize
        } else {
            DEFAULT_WIDTH
        };
        Self {
            data: iter,
            current_index: 0,
            title: String::new(),
            style: UnboundProgressBarStyle::default(),
            max_width: width - start_pos.0 as usize,
            start_pos,
        }
    }

    pub fn set_style(mut self, style: UnboundProgressBarStyle) -> Self {
        self.style = style;
        self
    }
    pub fn set_title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        self
    }

    pub fn set_max_width(mut self, max_width: usize) -> Self {
        self.max_width = max_width;
        self
    }
}

impl<I: Iterator> Iterator for UnboundProgressBar<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let str_len = self.current_index;

        let title = &self.title;
        let max_width = self.max_width - self.title.len();
        let width = str_len % max_width;

        let tip = self.style.get_tip();
        let fg_symbol = self.style.get_fg();
        let fg = vec![fg_symbol.to_string(); width].join("");
        let bg = vec![" "; max_width - width - 1].join("");

        if stdout().execute(crossterm::cursor::Hide).is_ok() {}

        let (x, y) = self.start_pos;
        if let Ok(_) = stdout().execute(crossterm::cursor::MoveTo(x, y)) {}

        print!("{title}{fg}{tip}{bg}");

        self.current_index += 1;
        self.data.next()
    }
}

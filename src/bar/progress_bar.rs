use crate::{defaults::DEFAULT_WIDTH, style::bar_style::ProgressBarStyle};

pub struct ProgressBar {}

impl ProgressBar {
    pub fn from_range<I: ExactSizeIterator>(iter: I) -> FiniteProgessBar<I> {
        let len = iter.len();
        FiniteProgessBar {
            data: iter,
            current_index: 0,
            max_len: len,
            width: DEFAULT_WIDTH,
            style: ProgressBarStyle::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FiniteProgessBar<I: ExactSizeIterator> {
    data: I,
    current_index: usize,
    max_len: usize,
    width: usize,
    style: ProgressBarStyle,
}

impl<I: ExactSizeIterator> Iterator for FiniteProgessBar<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let progress = self.current_index as f64 / self.max_len as f64;
        let str_len = (progress * self.width as f64) as usize;

        let counter = if self.style.get_show_counter() {
            let surround_left = self.style.get_counter_surround().0;
            let surround_right = self.style.get_counter_surround().1;
            let index = self.current_index;
            let len = self.max_len;
            let max_str_width = len.to_string().len();
            format!("{surround_left}{index:max_str_width$}/{len}{surround_right} ")
        } else {
            "".to_owned()
        };

        let surround_left = self.style.get_bar_surround().0;
        let surround_right = self.style.get_bar_surround().1;
        let fg_symbol = self.style.get_fg_symbol().to_string();
        let bg_symbol = self.style.get_bg_symbol().to_string();
        let fg = vec![fg_symbol; str_len].join("");
        let bg = vec![bg_symbol; self.width - str_len].join("");

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{counter}{surround_left}{fg}{bg}{surround_right}");

        self.current_index += 1;
        self.data.next()
    }
}

impl<I: ExactSizeIterator> FiniteProgessBar<I> {
    pub fn set_width(&mut self, width: usize) -> &mut Self {
        self.width = width;
        self
    }

    pub fn set_style(&mut self, style: ProgressBarStyle) -> &mut Self {
        self.style = style;
        self
    }
}

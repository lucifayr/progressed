use crate::{defaults::DEFAULT_WIDTH, style::bar_style::ProgressBarStyle};

pub struct ProgressBar {}

impl ProgressBar {
    pub fn from_range<I: ExactSizeIterator>(iter: I) -> FiniteProgessBar<I> {
        let len = iter.len();
        FiniteProgessBar {
            data: iter,
            current_index: 0,
            width: DEFAULT_WIDTH,
            max_len: len,
            style: ProgressBarStyle::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FiniteProgessBar<I: ExactSizeIterator> {
    data: I,
    current_index: usize,
    width: usize,
    max_len: usize,
    style: ProgressBarStyle,
}

impl<I: ExactSizeIterator> Iterator for FiniteProgessBar<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let progress = self.current_index as f64 / self.max_len as f64;
        let str_len = (progress * self.width as f64) as usize;

        let fg_symbol = self.style.get_fg_symbol().to_string();
        let bg_symbol = self.style.get_bg_symbol().unwrap_or(' ').to_string();
        let fg = vec![fg_symbol; str_len].join("");
        let bg = vec![bg_symbol; self.width - str_len].join("");

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("({}/{}) |{fg}{bg}|", self.current_index, self.max_len);

        self.current_index += 1;
        self.data.next()
    }
}

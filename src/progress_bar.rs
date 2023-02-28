#[derive(Debug, Clone)]
pub struct FiniteProgessBar<I: Iterator + ExactSizeIterator> {
    data: I,
    current_index: usize,
    width: usize,
    max_len: usize,
}

impl<I: Iterator + ExactSizeIterator> FiniteProgessBar<I> {
    pub fn from(iter: I) -> Self {
        let len = iter.len();
        Self {
            data: iter,
            current_index: 0,
            width: 30,
            max_len: len,
        }
    }
}

impl<I: Iterator + ExactSizeIterator> Iterator for FiniteProgessBar<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let progress = self.current_index as f64 / self.max_len as f64;
        let str_len = (progress * self.width as f64) as usize;
        let fg = vec!["#"; str_len].join("");
        let bg = vec!["-"; self.width - str_len].join("");

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("|{fg}{bg}|");

        self.current_index += 1;
        self.data.next()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_progress_bar() {}
}

use crate::style::bar_style::ProgressBarStyle;

pub struct UnboundedProgressBar<I: Iterator> {
    data: I,
    current_index: usize,
    style: ProgressBarStyle,
}

impl<I: Iterator> UnboundedProgressBar<I> {
    pub fn new(iter: I) -> Self {
        Self {
            data: iter,
            current_index: 0,
            style: ProgressBarStyle::default(),
        }
    }
}

impl<I: Iterator> Iterator for UnboundedProgressBar<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let counter = if self.style.get_show_counter() {
            let surround_left = self.style.get_counter_surround().0;
            let surround_right = self.style.get_counter_surround().1;
            let index = self.current_index;
            format!("{surround_left}{index:5}{surround_right} ")
        } else {
            "".to_owned()
        };

        let surround_left = self.style.get_bar_surround().0;
        let fg_symbol = self.style.get_fg_symbol().to_string();
        let fg = vec![fg_symbol.clone(); self.current_index].join("");
        let tip = self.style.get_tip_symbol().to_string();

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{counter}{surround_left}{fg}{tip}");

        self.current_index += 1;
        self.data.next()
    }
}

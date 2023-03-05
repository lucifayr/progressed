use std::io::stdout;

use crossterm::ExecutableCommand;

use crate::style::LoadingSpinnerStyle;

pub struct LoadingSpinner {
    style: LoadingSpinnerStyle,
    current_index: usize,
    start_pos: (u16, u16),
}

impl Default for LoadingSpinner {
    fn default() -> Self {
        let start_pos = crossterm::cursor::position().unwrap_or((0, 0));
        Self {
            style: LoadingSpinnerStyle::default(),
            current_index: 0,
            start_pos,
        }
    }
}

impl LoadingSpinner {
    pub fn tick(&mut self) {
        let symbols = self.style.get_spinner_symbols();
        let symbol = match symbols.get(self.current_index) {
            Some(symbol) => symbol,
            None => {
                self.current_index = 0;
                if let Some(symbol) = symbols.get(0) {
                    symbol
                } else {
                    &' '
                }
            }
        };

        if stdout().execute(crossterm::cursor::Hide).is_ok() {}

        let (x, y) = self.start_pos;
        if let Ok(_) = stdout().execute(crossterm::cursor::MoveTo(x, y)) {}

        print!("{symbol}");
        self.current_index += 1;
    }

    pub fn set_style(mut self, style: LoadingSpinnerStyle) -> Self {
        self.style = style;
        self.current_index = 0;
        self
    }
}

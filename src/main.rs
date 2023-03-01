use std::{thread, time::Duration};

use progressive::{bar::progress_bar::ProgressBar, style::bar_style::ProgressBarStyle};

fn main() {
    for _ in ProgressBar::new(0..100)
        .set_width(40)
        .set_style(ProgressBarStyle::arch())
    {
        thread::sleep(Duration::from_millis(25));
    }
}

use std::{thread, time::Duration};

use progressive::{bar::progress_bar::ProgressBar, style::bar_style::ProgressBarStyle};

fn main() {
    for _ in ProgressBar::new(0..500)
        .set_style(ProgressBarStyle::arch())
        .set_title("progress bar: ")
    {
        thread::sleep(Duration::from_millis(25));
    }
}

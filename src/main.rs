use std::{thread, time::Duration};

use progressive::{bar::progress_bar::ProgressBar, style::bar_style::ProgressBarStyle};

fn main() {
    for _ in ProgressBar::from_range(0..100).set_style(ProgressBarStyle::cargo()) {
        thread::sleep(Duration::from_millis(25));
    }
}

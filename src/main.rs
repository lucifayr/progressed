use std::{thread, time::Duration};

use progressive::{bar::progress_bar::ProgressBar, style::bar_style::ProgressBarStyle};

fn main() {
    for _ in ProgressBar::new(0..50)
        .set_style(ProgressBarStyle::smooth())
        .set_title("progress bar: ")
    {
        thread::sleep(Duration::from_millis(50));
    }

    for _ in ProgressBar::new(0..50)
        .set_style(ProgressBarStyle::arch())
        .set_title("progress bar 2: ")
    {
        thread::sleep(Duration::from_millis(50));
    }

    for _ in ProgressBar::new(0..50)
        .set_style(ProgressBarStyle::default())
        .set_title("progress bar 3: ")
    {
        thread::sleep(Duration::from_millis(50));
    }

    for _ in ProgressBar::new(0..50)
        .set_style(ProgressBarStyle::cargo())
        .set_title("progress bar 4: ")
    {
        thread::sleep(Duration::from_millis(50));
    }
}

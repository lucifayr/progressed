use std::{thread, time::Duration};

use progressive::{bar::progress_bar::ProgressBar, style::bar_style::ProgressBarStyle};

fn main() {
    for _ in ProgressBar::new(0..50)
        .set_style(ProgressBarStyle::arch().set_show_counter(true))
        .set_title("progress bar: ")
    {
        thread::sleep(Duration::from_millis(50));
    }

    println!("hmmmmmm");
    println!("hmmmmmm");

    thread::sleep(Duration::from_secs(50));
}

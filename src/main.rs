use std::{thread, time::Duration};

use progressive::{ProgressBar, ProgressBarStyle};

fn main() {
    let bar = ProgressBar::new(0..100);
    for _ in bar {}

    println!("Starting progress bar 1...");
    for _ in ProgressBar::new(0..50)
        .set_style(ProgressBarStyle::smooth())
        .set_title("progress bar: ")
    {
        thread::sleep(Duration::from_millis(50));
    }

    println!("Starting progress bar 2...");
    for _ in ProgressBar::new(0..50)
        .set_style(ProgressBarStyle::arch())
        .set_title("progress bar 2: ")
    {
        thread::sleep(Duration::from_millis(50));
    }

    println!("Starting progress bar 3...");
    for _ in ProgressBar::new(0..50)
        .set_style(ProgressBarStyle::default())
        .set_title("progress bar 3: ")
    {
        thread::sleep(Duration::from_millis(50));
    }

    println!("Starting progress bar 4...");
    for _ in ProgressBar::new(0..50)
        .set_style(ProgressBarStyle::cargo())
        .set_title("progress bar 4: ")
    {
        thread::sleep(Duration::from_millis(50));
    }
}

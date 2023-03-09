use std::{thread, time::Duration};

use progressed::{LoadingSpinner, LoadingSpinnerStyle, ProgressBar, ProgressBarStyle};

fn main() {
    let mut spinner = LoadingSpinner::default()
        .set_style(LoadingSpinnerStyle::fancy())
        .set_title("loading spinner: ");
    thread::spawn(move || loop {
        spinner.tick();
        thread::sleep(Duration::from_millis(100));
    });

    println!("");
    println!("Starting progress bar 1...");
    for _ in ProgressBar::new(0..50)
        .set_style(ProgressBarStyle::smooth())
        .set_title("smooth: ")
    {
        for _ in ProgressBar::new(0..50)
            .set_style(ProgressBarStyle::arch())
            .set_title("\narch: ")
        {
            thread::sleep(Duration::from_millis(50));
        }
    }

    println!("Starting progress bar 2...");
    for _ in ProgressBar::new(0..50)
        .set_style(ProgressBarStyle::arch())
        .set_title("arch: ")
    {
        thread::sleep(Duration::from_millis(50));
    }

    println!("Starting progress bar 3...");
    for _ in ProgressBar::new(0..50)
        .set_style(ProgressBarStyle::default())
        .set_title("default: ")
    {
        thread::sleep(Duration::from_millis(50));
    }

    println!("Starting progress bar 4...");
    for _ in ProgressBar::new(0..50)
        .set_style(ProgressBarStyle::cargo())
        .set_title("cargo: ")
    {
        thread::sleep(Duration::from_millis(50));
    }
}

use std::{thread, time::Duration};

use progressive::progress_bar::ProgressBar;

fn main() {
    for i in ProgressBar::from_range(0..100) {
        println!("Number is {i}");
        thread::sleep(Duration::from_millis(100));
    }
}

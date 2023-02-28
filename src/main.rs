use std::{thread, time::Duration};

use progressive::progress_bar::FiniteProgessBar;

fn main() {
    let bar = FiniteProgessBar::from(0..20);
    println!("{bar:?}");

    for i in bar {
        println!("Number is {i}");
        thread::sleep(Duration::from_millis(100));
    }
}

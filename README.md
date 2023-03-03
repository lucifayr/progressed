# Progressive
An easy to use and highly configurable library to show the progress of applications, calculations,
etc.

# Example Usage
``` rust
use std::{thread, time::Duration};
use progressive::{ProgressBar, ProgressBarStyle};

fn main() {
for _ in ProgressBar::new(0..100)
    .set_style(ProgressBarStyle::default())
    .set_title("progress bar: ")
    {
           thread::sleep(Duration::from_millis(50));
    }
}
```

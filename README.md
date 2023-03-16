# Progressed

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

An easy to use and highly configurable library to show the progress of applications, calculations,
etc.

## Example Usage
``` rust
use std::{thread, time::Duration};
use progressed::{ProgressBar, ProgressBarStyle};

fn main() {
for _ in ProgressBar::new(0..100)
    .set_style(ProgressBarStyle::default())
    .set_title("progress bar: ")
    {
           thread::sleep(Duration::from_millis(50));
    }
}
```

See the [documentation](https://docs.rs/progressed/latest/progressed) for more detailed information.

## Alternatives
- [progress](https://crates.io/crates/progress)
- [progressing](https://crates.io/crates/progressing) 

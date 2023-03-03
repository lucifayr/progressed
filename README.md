# Progressive

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Docs][docs.rs/self/badge]][docs.rs/self]

An easy to use and highly configurable library to show the progress of applications, calculations,
etc.

## Example Usage
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

See the [documentation](https://docs.rs/crate/progressive/latest) for more detailed information.

## Alternatives
- [progress](https://crates.io/crates/progress)
- [progressing](https://crates.io/crates/progressing) 

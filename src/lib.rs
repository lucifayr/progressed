//! # Progressive
//!
//! An easy to use and highly configurable library to show progress of applications, calculations,
//! etc.
//!
//! # Example Usage
//! ```no_run
//! use std::{thread, time::Duration};
//! use progressive::{ProgressBar, ProgressBarStyle};
//!
//! for _ in ProgressBar::new(0..100)
//!     .set_style(ProgressBarStyle::default())
//!     .set_title("progress bar: ")
//! {
//!        // thread::sleep(Duration::from_millis(50));
//! }
//! ```

pub use progress::ProgressBar;
pub use style::ProgressBarLayout;
pub use style::ProgressBarStyle;

mod defaults;
mod progress;
mod style;
mod symbols;

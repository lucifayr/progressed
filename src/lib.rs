//! # Progressed
//!
//! An easy to use and highly configurable library to show the progress of applications, calculations,
//! etc.
//!
//! # Example Usage
//! ```no_run
//! use std::{thread, time::Duration};
//! use progressed::{ProgressBar, ProgressBarStyle};
//!
//! for _ in ProgressBar::new(0..100)
//!     .set_style(ProgressBarStyle::default())
//!     .set_title("progress bar: ")
//! {
//!        thread::sleep(Duration::from_millis(50));
//! }
//! ```

pub use progress::LoadingSpinner;
pub use progress::ProgressBar;
pub use progress::UnboundProgressBar;
pub use style::LoadingSpinnerStyle;
pub use style::ProgressBarLayout;
pub use style::ProgressBarStyle;
pub use style::UnboundProgressBarStyle;

mod defaults;
mod progress;
mod style;
mod symbols;

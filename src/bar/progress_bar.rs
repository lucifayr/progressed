use super::{finite_bar::FiniteProgressBar, unbounded_bar::UnboundedProgressBar};

pub struct ProgressBar {}

impl ProgressBar {
    pub fn from_range<I: ExactSizeIterator>(iter: I) -> FiniteProgressBar<I> {
        FiniteProgressBar::new(iter)
    }

    pub fn from_unbounded_range<I: Iterator>(iter: I) -> UnboundedProgressBar<I> {
        UnboundedProgressBar::new(iter)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ProgressBarLayout {
    AllLeft,
    AllRight,
    TimerRight,
    CounterRight,
    PercentageRight,
    TimerAndCounterRight,
    TimerAndPercentageRight,
    CounterAndPercentageRight,
}

use std::{thread, time::Duration};

use progressive::{
    bar::progress_bar::ProgressBar,
    style::{bar_style::ProgressBarStyle, layout::ProgressBarLayout},
};

fn main() {
    for _ in ProgressBar::new(0..500)
        .set_style(
            ProgressBarStyle::default()
                .set_show_time(false)
                .set_show_counter(true)
                .set_show_percentage(true)
                .set_layout(ProgressBarLayout::TimerAndPercentageRight),
        )
        .set_title("progress bar: ")
    {
        thread::sleep(Duration::from_millis(100));
    }
}

pub static FULL_BLOCK: char = '█';
pub static LEFT_SEVEN_EIGHTHS_BLOCK: char = '▉';
pub static LEFT_THREE_QUARTERS_BLOCK: char = '▊';
pub static LEFT_FIVE_EIGHTHS_BLOCK: char = '▋';
pub static LEFT_HALF_BLOCK: char = '▌';
pub static LEFT_THREE_EIGHTHS_BLOCK: char = '▍';
pub static LEFT_ONE_QUARTER_BLOCK: char = '▎';
pub static LEFT_ONE_EIGHTH_BLOCK: char = '▏';

pub fn get_block_by_progress(progress: f64) -> char {
    match progress {
        p if p < 1.0 / 8.0 => LEFT_ONE_EIGHTH_BLOCK,
        p if p < 2.0 / 8.0 => LEFT_ONE_QUARTER_BLOCK,
        p if p < 3.0 / 8.0 => LEFT_THREE_EIGHTHS_BLOCK,
        p if p < 4.0 / 8.0 => LEFT_HALF_BLOCK,
        p if p < 5.0 / 8.0 => LEFT_FIVE_EIGHTHS_BLOCK,
        p if p < 6.0 / 8.0 => LEFT_THREE_QUARTERS_BLOCK,
        p if p < 7.0 / 8.0 => LEFT_SEVEN_EIGHTHS_BLOCK,
        _ => FULL_BLOCK,
    }
}

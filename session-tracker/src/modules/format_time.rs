use std::time::{Duration};

pub fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let minutes = seconds / 60;
    let hour = seconds / 3600;
    
    if hour > 0 {
        format!("{} : {} : {} hours", hour, minutes % 60, seconds % 60)
    } else if minutes > 0 {
        format!("{} : {} minutes", minutes, seconds % 60)
    } else {
        format!("{} seconds", seconds)
    }
}

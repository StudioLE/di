#[allow(dead_code)]
#[must_use]
pub fn format_duration_fixed(duration: u32) -> String {
    let (hours, minutes, seconds) = duration_to_separate(duration);
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

#[must_use]
pub fn format_duration_human(duration: u32) -> String {
    let mut segments = Vec::new();
    let (hours, minutes, seconds) = duration_to_separate(duration);
    if hours > 0 {
        segments.push(format!("{hours}h"));
    }
    if minutes > 0 {
        segments.push(format!("{minutes}m"));
    }
    if seconds > 0 {
        segments.push(format!("{seconds}s"));
    }
    segments.join(" ")
}

#[allow(clippy::integer_division)]
fn duration_to_separate(duration: u32) -> (u32, u32, u32) {
    let hours = duration / (60 * 60);
    let minutes = (duration % (60 * 60)) / 60;
    let seconds = duration % 60;
    (hours, minutes, seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _duration_to_separate() {
        assert_eq!(duration_to_separate(3665), (1, 1, 5));
        assert_eq!(duration_to_separate(u32::MAX), (1_193_046, 28, 15));
        assert_eq!(duration_to_separate(0), (0, 0, 0));
        assert_eq!(duration_to_separate(1), (0, 0, 1));
        assert_eq!(duration_to_separate(60), (0, 1, 0));
    }

    #[test]
    fn _format_duration_fixed() {
        assert_eq!(format_duration_fixed(1), "00:00:01");
        assert_eq!(format_duration_fixed(60), "00:01:00");
        assert_eq!(format_duration_fixed(3665), "01:01:05");
    }

    #[test]
    fn _format_duration_human() {
        assert_eq!(format_duration_human(1), "1s");
        assert_eq!(format_duration_human(60), "1m");
        assert_eq!(format_duration_human(3665), "1h 1m 5s");
    }
}

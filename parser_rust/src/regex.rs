use regex::Regex;

pub struct TimeParser {
    regex: Regex,
}

impl TimeParser {
    pub fn new() -> Self {
        TimeParser {
            regex: Self::parse_regex(),
        }
    }

    fn parse_regex() -> Regex {
        // Define regex patterns for different time formats
        let hour12 = "1[0-2]|[1-9]";
        let hour24 = "2[0-3]|1[0-9]|[0-9]";
        let minute = "[0-5][0-9]";
        let am_pm = "AM|PM";

        // For 12-hour format with minutes
        let hour12_full = format!(r"(?P<hour12_full>{})", hour12);
        let minute_full = format!(r"(?P<minute_full>{})", minute);
        let ampm_full = format!(r"(?P<ampm_full>{})", am_pm);

        // For 24-hour format
        let hour24 = format!(r"(?P<hour24>{})", hour24);
        let minute24 = format!(r"(?P<minute24>{})", minute);

        // For 12-hour format without minutes
        let hour12_short = format!(r"(?P<hour12_short>{})", hour12);
        let ampm_short = format!(r"(?P<ampm_short>{})", am_pm);

        // Combine patterns into a single regex
        let time_full12 = format!(r"^{}:{}{}$", hour12_full, minute_full, ampm_full);
        let time_full24 = format!(r"^{}:{}$", hour24, minute24);
        let time_short = format!(r"^{}{}$", hour12_short, ampm_short);

        // Compile the combined regex
        Regex::new(&format!("{}|{}|{}", time_full12, time_full24, time_short)).unwrap()
    }

    fn hour12_to_24(hour: u32, am_pm: &str) -> u32 {
        match (am_pm, hour) {
            ("AM", 12) => 0,
            ("AM", _) => hour,
            ("PM", 12) => 12,
            ("PM", _) => hour + 12,
            _ => hour, // fallback, should not happen if input is valid
        }
    }

    pub fn parse(&self, time_str: &str) -> Option<u32> {
        if let Some(caps) = self.regex.captures(time_str) {
            if let (Some(hour), Some(minute), Some(am_pm)) = (
                caps.name("hour12_full"),
                caps.name("minute_full"),
                caps.name("ampm_full"),
            ) {
                let h: u32 = hour.as_str().parse().ok()?;
                let m: u32 = minute.as_str().parse().ok()?;
                let am_pm = am_pm.as_str();
                return Some(Self::hour12_to_24(h, am_pm) * 60 + m);
            } else if let (Some(hour), Some(minute)) = (caps.name("hour24"), caps.name("minute24"))
            {
                let h: u32 = hour.as_str().parse().ok()?;
                let m: u32 = minute.as_str().parse().ok()?;
                return Some(h * 60 + m);
            } else if let (Some(hour), Some(am_pm)) =
                (caps.name("hour12_short"), caps.name("ampm_short"))
            {
                let h: u32 = hour.as_str().parse().ok()?;
                let am_pm = am_pm.as_str();
                return Some(Self::hour12_to_24(h, am_pm) * 60);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_parser_ok() {
        let table = vec![
            ("4PM", 16 * 60),
            ("12AM", 0),
            ("12PM", 12 * 60),
            ("7:38PM", 19 * 60 + 38),
            ("23:42", 23 * 60 + 42),
            ("3:16", 3 * 60 + 16),
            ("3:16AM", 3 * 60 + 16),
            ("12:00AM", 0),
            ("12:00PM", 12 * 60),
            ("1AM", 1 * 60),
            ("1PM", 13 * 60),
            ("0:00", 0),
            ("23:59", 23 * 60 + 59),
            ("9AM", 9 * 60),
            ("9:05PM", 21 * 60 + 5),
        ];

        for (input, expected) in table {
            let parser = TimeParser::new();
            let result = parser.parse(input);
            assert_eq!(result, Some(expected), "Failed to parse '{}'", input);
        }
    }

    #[test]
    fn test_time_parser_ng() {
        let table = vec!["00:00", "24:00", "13:60", "0AM", "12:61PM", "abc"];

        for input in table {
            let parser = TimeParser::new();
            let result = parser.parse(input);
            assert_eq!(result, None, "Expected error for '{}'", input);
        }
    }
}

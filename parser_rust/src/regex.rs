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
        let hour12 = r"(1[0-2]|[1-9])";
        let hour24 = r"(2[0-3]|1[0-9]|[0-9])";
        let minute = r"([0-5][0-9])";
        let am_pm = r"(AM|PM)";

        let time_full12 = format!(r"^{}:{}{}$", hour12, minute, am_pm);
        let time_full24 = format!(r"^{}:{}$", hour24, minute);
        let time_short = format!(r"^{}{}$", hour12, am_pm);

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
            if let Some(hour) = caps.get(1) {
                let h: u32 = hour.as_str().parse().unwrap();
                let m: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
                let am_pm = caps.get(3).unwrap().as_str();
                return Some(Self::hour12_to_24(h, am_pm) * 60 + m);
            } else if let Some(hour) = caps.get(4) {
                let h: u32 = hour.as_str().parse().unwrap();
                let m: u32 = caps.get(5).unwrap().as_str().parse().unwrap();
                return Some(h * 60 + m);
            } else if let Some(hour) = caps.get(6) {
                let h: u32 = hour.as_str().parse().unwrap();
                let am_pm = caps.get(7).unwrap().as_str();
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

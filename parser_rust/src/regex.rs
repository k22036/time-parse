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
        let hour12 = r"(1[0-2]|0?[1-9])";
        let hour24 = r"(2[0-3]|1[0-9]|0?[0-9])";
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

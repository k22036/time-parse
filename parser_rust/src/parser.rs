peg::parser! {
    pub grammar time_parser() for str {
        pub rule time() -> u32
            = full12_time() / full24_time() / short_time()

        rule full12_time() -> u32
            = h:hour12() ":" m:minute() am_pm:am_pm() {
                let mut hour = h;
                if am_pm == "PM" && hour != 12 {
                    hour += 12;
                } else if am_pm == "AM" && hour == 12 {
                    hour = 0;
                }
                hour * 60 + m
            }

        rule full24_time() -> u32
            = h:hour24() ":" m:minute() {
                h * 60 + m
            }

        rule short_time() -> u32
            = h:hour12() am_pm:am_pm() {
                let mut hour = h;
                if am_pm == "PM" && hour != 12 {
                    hour += 12;
                } else if am_pm == "AM" && hour == 12 {
                    hour = 0;
                }
                hour * 60
            }

        rule digit() -> u32
            = n:$(['0'..='9']) { n.parse().unwrap() }

        rule hour12() -> u32
            = n:$("10" / "11" / "12" / ['1'..='9']) { n.parse().unwrap() }
        rule hour24() -> u32
            = n:$("1" digit() / "2" ['0'..='3'] / digit()) { n.parse().unwrap() }

        rule minute() -> u32
            = n:$(['0'..='5'] digit()) { n.parse().unwrap() }

        rule am_pm() -> &'input str
            = s:$("AM" / "PM") { s }
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
            let result = time_parser::time(input);
            assert_eq!(result, Ok(expected), "Failed to parse '{}'", input);
        }
    }

    #[test]
    fn test_time_parser_ng() {
        let table = vec!["00:00", "24:00", "13:60", "0AM", "12:61PM", "abc"];

        for input in table {
            let result = time_parser::time(input);
            assert!(result.is_err(), "Expected error for '{}'", input);
        }
    }
}

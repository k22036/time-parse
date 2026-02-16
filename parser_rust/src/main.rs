mod parser;
mod regex;

fn main() {
    let time = "12AM";

    print!("peg-parser: ");
    let ret = parser::time_parser::time(time);
    match ret {
        Ok(minutes) => println!("Parsed time: {} minutes after midnight", minutes),
        Err(e) => println!("Failed to parse time: {}", e),
    }

    print!("regex-parser: ");
    let regex_parser = regex::TimeParser::new();
    match regex_parser.parse(time) {
        Some(minutes) => println!("Parsed time: {} minutes after midnight", minutes),
        None => println!("Failed to parse time"),
    }
}

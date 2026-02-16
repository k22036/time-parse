mod parser;

fn main() {
    let time = "12AM";
    let ret = parser::time_parser::time(time);
    match ret {
        Ok(minutes) => println!("Parsed time: {} minutes after midnight", minutes),
        Err(e) => println!("Failed to parse time: {}", e),
    }
}

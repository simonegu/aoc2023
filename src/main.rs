use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
mod day1;
mod day2;
mod day3;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    let file_path = &args[2];
    // println!("{file_path}");
    let lines = read_lines(file_path).unwrap();
    match day.as_str() {
        "1a" => day1::day1a(lines),
        "1b" => day1::day1b(lines),
        "2a" => day2::day2a(lines),
        "2b" => day2::day2b(lines),
        "3a" => day3::day3a(lines),
        "3b" => day3::day3b(lines),
        _ => println!("No valid day!"),
    }
}

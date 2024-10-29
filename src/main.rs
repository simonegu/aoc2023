use std::env;
mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    let file_path = &args[2];
    println!("{file_path}");
    match day.as_str() {
        "1a" => day1::day1a(file_path),
        "1b" => day1::day1b(file_path),
        "2a" => day2::day2a(file_path),
        "2b" => day2::day2b(file_path),
        _ => println!("No valid day!"),
    }
}

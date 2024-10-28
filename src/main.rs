use std::env;
mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    let file_path = &args[2];
    println!("{file_path}");
    match day.parse::<i32>().unwrap() {
        1 => day1::day1a(file_path),
        _ => println!("No valid day!"),
    }
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn day1a(file_path: &String) {
    let mut res = 0;
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(c) = line {
                let nums : Vec<&str> =  c.matches(char::is_numeric).collect();
                if nums.len() >= 1 {
                    let n1 = nums[0].parse::<u32>().unwrap();
                    let n2 = nums[nums.len()-1].parse::<u32>().unwrap();
                    res += 10*n1 + n2;
                } else {
                    println!("size error");
                }
            }
        }
    }
    println!("result {}", res);
}


pub fn day1b(file_path: &String) {
    let numbers = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut res = 0;
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(c) = line {
                // search for
                let mut min_index = 100;
                let mut max_index = 0;
                let mut n1 = 0;
                let mut n2 = 0;
                for el in &numbers {
                    if let Some(num) = c.find(el.0){
                        if num <= min_index {
                            min_index = num;
                            n1 = el.1.clone();
                        }
                    }
                    if let Some(num) = c.rfind(el.0){
                        if num >= max_index {
                            max_index = num;
                            n2 = el.1.clone();
                        }
                    }
                }
                // println!(" number: {}{}", n1, n2);
                res += 10*n1 + n2;
            }
        }
    }
    println!("result {}", res);
}

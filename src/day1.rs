use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

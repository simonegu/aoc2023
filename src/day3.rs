use itertools::concat;
use regex::Regex;
use std::fs::File;
use std::io;

pub fn day3a(lines: io::Lines<io::BufReader<File>>) {
    let mut res = 0;
    let mut priv_line: String = "".to_string();
    let re_numbers = Regex::new(r"\d+").unwrap();
    for line in lines {
        if let Ok(c) = line {
            let symbols_collections = ['$', '*', '/', '#', '@', '+', '=', '-', '%', '&'];
            // find special char index
            let s1: Vec<_> = c.match_indices(&symbols_collections).collect();
            let s2: Vec<_> = priv_line.match_indices(&symbols_collections).collect();
            let symbols = concat([s1.clone(), s2]);
            // find numbers and index...
            let n1 = re_numbers.find_iter(&c);
            let n2 = re_numbers.find_iter(&priv_line);
            for number in n1 {
                // match current symbols with current numbers and previous symbols with current numbers
                let n_start: i32 = number.start() as i32;
                let n_end: i32 = number.end() as i32;
                // match prev number with current symbols
                for (uidx, _sym) in &symbols {
                    let idx: i32 = (*uidx) as i32;
                    if idx >= n_start - 1 && idx <= n_end {
                        // print!("{}({}, {} [{},{}]), ", number.as_str(),*_sym, uidx, n_start, n_end);
                        res += number.as_str().parse::<i32>().unwrap();
                        break;
                    }
                }
            }

            for number in n2 {
                let n_start: i32 = number.start() as i32;
                let n_end: i32 = number.end() as i32;
                // match prev number with current symbols
                for (uidx, _sym) in &s1 {
                    let idx: i32 = (*uidx) as i32;
                    if idx >= n_start - 1 && idx <= n_end {
                        // print!("{}({}, {} [{},{}]), ", number.as_str(),*_sym, uidx, n_start, n_end);
                        res += number.as_str().parse::<i32>().unwrap();
                        break;
                    }
                }
            }
            // TODO: consider storing the searches for the previous line
            // collect prev line
            priv_line = c;
        }
    }
    println!("result {}", res);
}

pub fn day3b(lines: io::Lines<io::BufReader<File>>) {
    let mut res = 0;
    let mut line_1: String = "".to_string();
    let mut line_2: String = "".to_string();
    let re_numbers = Regex::new(r"\d+").unwrap();
    for line in lines {
        if let Ok(c) = line {
            let symbols_collections = ['*'];
            // find special char index
            let symbols: Vec<_> = line_1.match_indices(&symbols_collections).collect();
            // find numbers and index...
            let n1: Vec<_> = re_numbers.find_iter(&c).collect();
            let n2: Vec<_> = re_numbers.find_iter(&line_1).collect();
            let n3: Vec<_> = re_numbers.find_iter(&line_2).collect();
            let numbers = concat([n3, n2, n1]);
            for (uidx, _sym) in &symbols {
                let mut gear_count = 0;
                let mut gear_match = 0;
                let idx: i32 = (*uidx) as i32;
                for number in &numbers {
                    let n_start: i32 = (*number).start() as i32;
                    let n_end: i32 = (*number).end() as i32;
                    if idx >= n_start - 1 && idx <= n_end {
                        gear_count += 1;
                        if gear_count == 2 {
                            println!("{}:{}", gear_match, (*number).as_str());
                            res += gear_match * (*number).as_str().parse::<i32>().unwrap();
                            break;
                        } else {
                            gear_match = (*number).as_str().parse::<i32>().unwrap();
                        }
                    }
                }
            }
            // collect prev lines
            line_2 = line_1;
            line_1 = c;
        }
    }
    println!("result {}", res);
}

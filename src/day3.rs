use std::fs::File;
use std::io;
use regex::Regex;
use itertools::concat;

pub fn day3a(lines: io::Lines<io::BufReader<File>>){
    let mut res = 0;
    let mut priv_line: String = "".to_string();
    let re_numbers = Regex::new(r"\d+").unwrap();
    for line in lines {
        if let Ok(c) = line {
            let symbols_collections = ['$','*','/','#','@','+','=','-','%','&'];
            // find special char index
            let s1: Vec<_> = c.match_indices(&symbols_collections).collect();
            let s2: Vec<_> = priv_line.match_indices(&symbols_collections).collect();
            let symbols = concat([s1.clone(),s2]);
            // find numbers and index...
            let n1 = re_numbers.find_iter(&c);
            let n2 = re_numbers.find_iter(&priv_line);
            for number in n1 {
                // match current symbols with current numbers and previous symbols with current numbers
                let n_start: i32 = number.start() as i32;
                let n_end: i32 = number.end() as i32;
                // match prev number with current symbols
                for (uidx,_sym) in &symbols {
                    let idx: i32 = (*uidx) as i32;
                    if idx >= n_start-1 && idx <= n_end {
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
                for (uidx,_sym) in &s1 {
                    let idx: i32 = (*uidx) as i32;
                    if idx >= n_start-1 && idx <= n_end {
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

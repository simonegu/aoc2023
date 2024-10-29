use std::fs::File;
use std::io;
use regex::Regex;
use std::cmp;

pub fn day2a(lines: io::Lines<io::BufReader<File>>){
    let red = 12;
    let green = 13;
    let blue = 14;
    let mut res = 0;
    let re_game = Regex::new(r"Game (\d+)").unwrap();
    let re_blue = Regex::new(r"(\d+) blue").unwrap();
    let re_green = Regex::new(r"(\d+) green").unwrap();
    let re_red = Regex::new(r"(\d+) red").unwrap();
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(c) = line {
            // split the string on the ;
            let game: Vec<&str> = c.split(":").collect();
            let game_captures = re_game.captures(game[0]).unwrap();
            let game_num = game_captures.get(1).unwrap().as_str().parse::<i32>().unwrap_or_default();

            // extract blue, green and red numbers
            let mut add_game = true;
            let sets = game[1].split(";");
            for set in sets {
                let mut red_count = 0;
                let mut blue_count = 0;
                let mut green_count = 0;

                if let Some(cap) = re_red.captures(set) {
                    red_count = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                }
                if let Some(cap) = re_blue.captures(set) {
                    blue_count = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                }
                if let Some(cap) = re_green.captures(set) {
                    green_count = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                }
                // check if smaller than max add to result
                if red_count > red || blue_count > blue || green_count > green {
                    add_game &= false;
                }
            }
            if add_game {
                res += game_num;
            }
        }
    }
    println!("result {}", res);
}

pub fn day2b(lines: io::Lines<io::BufReader<File>>){
    let mut res = 0;
    let re_blue = Regex::new(r"(\d+) blue").unwrap();
    let re_green = Regex::new(r"(\d+) green").unwrap();
    let re_red = Regex::new(r"(\d+) red").unwrap();
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(c) = line {
            // split the string on the :
            let game: Vec<&str> = c.split(":").collect();

            // extract blue, green and red numbers
            let mut red_max = 0;
            let mut blue_max = 0;
            let mut green_max = 0;
            let sets = game[1].split(";");
            for set in sets {
                if let Some(cap) = re_red.captures(set) {
                    red_max = cmp::max(red_max, cap.get(1).unwrap().as_str().parse::<i32>().unwrap());
                }
                if let Some(cap) = re_blue.captures(set) {
                    blue_max = cmp::max(blue_max, cap.get(1).unwrap().as_str().parse::<i32>().unwrap());
                }
                if let Some(cap) = re_green.captures(set) {
                    green_max = cmp::max(green_max, cap.get(1).unwrap().as_str().parse::<i32>().unwrap());
                }
            }
            res += red_max*blue_max*green_max;
        }
    }
    println!("result {}", res);
}

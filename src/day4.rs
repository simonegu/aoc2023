use std::collections::HashMap;
use std::fs::File;
use std::io;
use regex::Regex;

pub fn day4a(lines: io::Lines<io::BufReader<File>>){
    let mut res = 0;
    let re_numbers = Regex::new(r"\d+").unwrap();
    for line in lines {
        if let Ok(c) = line {
            let dummy: Vec<_> = c.split(":").collect();
            let numbers: Vec<_> = dummy[1].split("|").collect();

            let winning_num_str = re_numbers.find_iter(numbers[0]);
            let mut winning_nums: Vec<_> = winning_num_str.map(|c| c.as_str().parse::<i32>().unwrap()).collect();
            winning_nums.sort();

            let num_str = re_numbers.find_iter(numbers[1]);
            let mut nums: Vec<_> = num_str.map(|c| c.as_str().parse::<i32>().unwrap()).collect();
            nums.sort();

            let mut i = 0;
            let mut j = 0;
            let mut points = 0;
            while i < nums.len() && j < winning_nums.len() {
                if nums[i] < winning_nums[j] {
                    i+=1;
                } else if nums[i] > winning_nums[j] {
                    j+=1;
                } else {
                    if points == 0 {
                        points = 1;
                    } else {
                        points *= 2;
                    }
                    i+=1;
                    j+=1;
                }
            }
            res += points;
        }
    }
    println!("result {}", res);
}

pub fn day4b(lines: io::Lines<io::BufReader<File>>){
    let mut winnings = HashMap::new();
    let mut res = 0;
    let re_numbers = Regex::new(r"\d+").unwrap();
    for line in lines {
        if let Ok(c) = line {
            let dummy: Vec<_> = c.split(":").collect();
            let game_num_str = re_numbers.find(dummy[0]).unwrap();
            let game_num = game_num_str.as_str().parse::<i32>().unwrap();
            let numbers: Vec<_> = dummy[1].split("|").collect();

            let winning_num_str = re_numbers.find_iter(numbers[0]);
            let mut winning_nums: Vec<_> = winning_num_str.map(|c| c.as_str().parse::<i32>().unwrap()).collect();
            winning_nums.sort();

            let num_str = re_numbers.find_iter(numbers[1]);
            let mut nums: Vec<_> = num_str.map(|c| c.as_str().parse::<i32>().unwrap()).collect();
            nums.sort();

            let mut i = 0;
            let mut j = 0;
            let mut points = 0;
            while i < nums.len() && j < winning_nums.len() {
                if nums[i] < winning_nums[j] {
                    i+=1;
                } else if nums[i] > winning_nums[j] {
                    j+=1;
                } else {
                    points += 1;
                    i+=1;
                    j+=1;
                }
            }
            let wins: Vec<_> = (game_num+1..game_num+1+points).collect();
            // println!("{:#?}",wins);
            // println!("game: {}", game_num);
            match winnings.get(&game_num) {
                Some(count) => {
                    winnings.insert(game_num, count+1);
                }
                None => { winnings.insert(game_num, 1); }
            }
            for w in wins {
                let prev_count = *winnings.get(&game_num).unwrap();
                for _i in 0..prev_count {
                    match winnings.get(&w) {
                        Some(count) => { winnings.insert(w, count+1); }
                        None => { winnings.insert(w, 1); }
                    }
                }
            }
            // winnings.insert(w, (game_num+1));
            // res += points;
        }
        // println!("{:#?}", winnings);
        res = winnings.clone().into_iter().filter_map(|x| Some(x.1)).sum();
    }
    println!("result {}", res);
}

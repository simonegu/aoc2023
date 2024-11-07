use std::fs::File;
use std::io;

struct Race {
    time: u64,
    distance: u64,
}

pub fn day6a(_lines: io::Lines<io::BufReader<File>>){
    let races = vec![
        Race{time: 61677571, distance: 430103613071150},
        // Race{time: 61, distance: 430},
        // Race{time: 67, distance: 1036},
        // Race{time: 75, distance: 1307},
        // Race{time: 71, distance: 1150},
    ];
    // let re_numbers = Regex::new(r"\d+").unwrap();
    // for line in lines {
    //     if let Ok(c) = line {
    //         re_numbers.find_iter(&c)
    //     }
    // }
    //let mut wins = 0;
    let mut res = 1;
    for race in races {
        let mut wins = 0;
        for t in 0..race.time {
            let speed = t;
            let dist = speed * (race.time - t);
            if dist > race.distance {
                wins += 1;
            }
        }
        res *= wins;
    }
    println!("result {}", res);
}

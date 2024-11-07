use std::collections::HashMap;
use std::fs::File;
use std::{cmp, i64, io};

#[derive(Debug, Clone, Copy)]
struct Mapping {
    source: i64,
    destination: i64,
    length: i64,
}

// define enum of mappings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Maps {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}
impl Maps {
    const VALUES: [Self; 7] = [
    Self::SeedToSoil,
    Self::SoilToFertilizer,
    Self::FertilizerToWater,
    Self::WaterToLight,
    Self::LightToTemperature,
    Self::TemperatureToHumidity,
    Self::HumidityToLocation,
    ];
}

fn strings_to_maps(s: &str) -> Option<Maps> {
    match s {
        "seed-to-soil" => Some(Maps::SeedToSoil),
        "soil-to-fertilizer" => Some(Maps::SoilToFertilizer),
        "fertilizer-to-water" => Some(Maps::FertilizerToWater),
        "water-to-light" => Some(Maps::WaterToLight),
        "light-to-temperature" => Some(Maps::LightToTemperature),
        "temperature-to-humidity" => Some(Maps::TemperatureToHumidity),
        "humidity-to-location" => Some(Maps::HumidityToLocation),
        _ => None,
    }
}

pub fn day5a(mut lines: io::Lines<io::BufReader<File>>){
    let mut mappings = HashMap::new();
    let first_line = lines.next().unwrap().unwrap();
    let seeds_str: Vec<_> = first_line.split(":").collect();
    let  mut seed_list: Vec<_> = seeds_str[1].split(" ").collect();
    seed_list = seed_list[1..].to_vec();
    let seeds: Vec<_> = seed_list.iter().map(|x| x.parse::<i64>().unwrap()).collect();
    // println!("{:#?}", seeds);
    let mut current_map = None;
    for line in lines {
        if let Ok(c) = line {
            // println!("{}", c);
            if c.find("map").is_some() {
                // match entry..
                let entry: Vec<_> = c.split(" ").collect();
                current_map = strings_to_maps(entry[0]);
            } else if c.find(char::is_numeric).is_some() {
                // is a mapping instruction
                let entries_str: Vec<_> = c.split(" ").collect();
                let entries: Vec<_> = entries_str.iter().map(|x| x.parse::<i64>().unwrap()).collect();
                let mapping = Mapping{source: entries[1], destination: entries[0],length: entries[2]};
                if let Some(key) = current_map {
                    mappings.entry(key).or_insert_with(Vec::new).push(mapping);
                }

            } else {
                // is a carriage return ignore it
            }
        }
    }
    // println!("{:#?}", mappings);

    let mut res = i64::MAX;
    // loop through the seeds
    for seed in seeds {
        // println!("seed: {}", seed);
        let mut curr = seed;
        for i in Maps::VALUES {
            if let Some(rules) = mappings.get(&i) {
                for rule in rules {
                    if curr >= rule.source && curr < rule.source + rule.length {
                        // mapping happening
                        let inc = curr - rule.source;
                        curr = rule.destination + inc;
                        break;
                    }

                }
                // println!("{:?}: {}", i, curr);
            }
        }
        // println!("{}", curr);
        res = cmp::min(res, curr);
    }
    println!("result {}", res);
}

fn seed_to_location(mappings: &HashMap<Maps, Vec<Mapping>>, seed: i64) -> i64 {
    let mut curr = seed;
    for i in Maps::VALUES {
        if let Some(rules) = mappings.get(&i) {
            for rule in rules {
                if curr >= rule.source && curr < rule.source + rule.length {
                    // mapping happening
                    let inc = curr - rule.source;
                    curr = rule.destination + inc;
                    break;
                }

            }
            // println!("{:?}: {}", i, curr);
        }
    }
    curr
}

fn seeds_batch(res: &mut i64, mappings: &HashMap<Maps, Vec<Mapping>>, seed_start: i64, seed_length: i64) {
    for seed in seed_start..seed_start+seed_length {
        // println!("seed: {}", seed);
        let curr = seed_to_location(&mappings, seed);
        // println!("{}", curr);
        *res = cmp::min(*res, curr);
    }
}

pub fn day5b(mut lines: io::Lines<io::BufReader<File>>){
    let mut mappings = HashMap::new();
    let first_line = lines.next().unwrap().unwrap();
    let seeds_str: Vec<_> = first_line.split(":").collect();
    let  mut seed_list: Vec<_> = seeds_str[1].split(" ").collect();
    seed_list = seed_list[1..].to_vec();
    let seeds: Vec<_> = seed_list.iter().map(|x| x.parse::<i64>().unwrap()).collect();
    // println!("{:#?}", seeds);
    let mut current_map = None;
    for line in lines {
        if let Ok(c) = line {
            // println!("{}", c);
            if c.find("map").is_some() {
                // match entry..
                let entry: Vec<_> = c.split(" ").collect();
                current_map = strings_to_maps(entry[0]);
            } else if c.find(char::is_numeric).is_some() {
                // is a mapping instruction
                let entries_str: Vec<_> = c.split(" ").collect();
                let entries: Vec<_> = entries_str.iter().map(|x| x.parse::<i64>().unwrap()).collect();
                let mapping = Mapping{source: entries[1], destination: entries[0],length: entries[2]};
                if let Some(key) = current_map {
                    mappings.entry(key).or_insert_with(Vec::new).push(mapping);
                }

            } else {
                // is a carriage return ignore it
            }
        }
    }
    // println!("{:#?}", mappings);
    let threads = seeds.len()/2;
    println!("running on {} threads", threads);
    let mut results: Vec<i64> = vec![i64::MAX;threads];
    let mappings_vec = vec![mappings.clone(); threads];
    {
        let threads_results: Vec<_> = results.chunks_mut(1).collect();
        crossbeam::scope(|spawner| {
            for (i, res) in threads_results.into_iter().enumerate() {
                let seed_start = seeds[i*2];
                let seed_length = seeds[i*2+1];
                let map = &mappings_vec[i];
                spawner.spawn(move |_| {
                    seeds_batch(&mut res[0], map, seed_start, seed_length);
                });
            }
        }).unwrap();
    }

    // println!("{:#?}", results);
    let min = results.iter().min().unwrap();
    println!("result {}",min);
}

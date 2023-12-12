use std::{fs::File, io, collections::{HashMap, HashSet}};

use crate::files::read_lines;



#[derive(Clone, Copy)]
enum SpringStatus {
    Unknown,
    Operational,
    Damaged
}

impl SpringStatus {
    pub fn from(c: char) -> SpringStatus {
        match c {
            '?' => SpringStatus::Unknown,
            '.' => SpringStatus::Operational,
            '#' => SpringStatus::Damaged,
            _ => panic!("unknown")
        }
    }

    pub fn print(&self) -> char {
        match self {
            SpringStatus::Unknown => '?',
            SpringStatus::Operational => '.',
            SpringStatus::Damaged => '#',
        }
    }
}

fn print_config(configuration: &Vec<SpringStatus>) {
    for s in configuration {
        print!("{}", s.print());
    }

    println!();
}

fn valid(configuration: &Vec<SpringStatus>, damaged_segments: Vec<usize>) -> bool {
    let mut segment_idx = 0;

    let mut damaged_count = 0;
    for status in configuration {
        match status {
            SpringStatus::Unknown => panic!("don't try to validate something with unknown status"),
            SpringStatus::Operational => {
                if damaged_count > 0 {
                    if damaged_count != damaged_segments[segment_idx] {
                        return false;
                    }
                    if damaged_count == damaged_segments[segment_idx] {
                        segment_idx += 1;
                    }
                    damaged_count = 0;

                }
            },
            SpringStatus::Damaged => {
                damaged_count += 1;
            }
        }
    }

    if damaged_count > 0 {
        if segment_idx != damaged_segments.len() - 1 {
            return false;
        }
        if damaged_count != damaged_segments[segment_idx] {
            return false;
        }

        segment_idx += 1;
    }


    segment_idx == damaged_segments.len()
}


fn arrange(
    configuration: &mut Vec<SpringStatus>, 
    damaged_segments: &Vec<usize>, 
    config_idx: usize,
    segment_idx: usize,
    damaged_count: usize,
    memo: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if config_idx == configuration.len() {
        if segment_idx == damaged_segments.len() && damaged_count == 0 {
            return 1;
        }
        if segment_idx == damaged_segments.len() - 1 && damaged_segments[segment_idx] == damaged_count { 
           return 1;
        }

        // println!("{} {} cfg_idx: {} damaged: {}", segment_idx, damaged_segments.len(), config_idx, damaged_count);
        return 0;
    }
    if memo.contains_key(&(config_idx, segment_idx, damaged_count)) {
        return *memo.get(&(config_idx, segment_idx, damaged_count)).unwrap();
    }

    let mut count = 0;
    match configuration[config_idx] {
        SpringStatus::Unknown => {
            // make it operational
           if damaged_count > 0 {
                if segment_idx < damaged_segments.len() && damaged_segments[segment_idx] == damaged_count {
                    count += arrange(configuration,
                        damaged_segments, 
                        config_idx + 1, 
                        segment_idx + 1, 
                        0,
                        memo
                    );
                }
            } else {
                count += arrange(configuration, damaged_segments, config_idx + 1, segment_idx, damaged_count, memo);
            }

            count += arrange(configuration,
                damaged_segments, 
                config_idx + 1, 
                segment_idx, 
                damaged_count + 1,
                memo
            );
            // panic!("xd");
        },
        SpringStatus::Operational => {
            if damaged_count > 0 {
                if segment_idx < damaged_segments.len() && damaged_segments[segment_idx] == damaged_count {
                    count += arrange(configuration,
                        damaged_segments, 
                        config_idx + 1, 
                        segment_idx + 1, 
                        0,
                        memo
                    );
                }
            } else {
                count += arrange(configuration, damaged_segments, config_idx + 1, segment_idx, damaged_count, memo)
            }
        }
        SpringStatus::Damaged => {
            count += arrange(configuration,
                damaged_segments, 
                config_idx + 1, 
                segment_idx, 
                damaged_count + 1,
                memo
            )
        }
    }

    memo.insert((config_idx, segment_idx, damaged_count), count);
    count
}

fn parse(s: &str) -> (Vec<SpringStatus>, Vec<usize>) {
    let parts = s.split(" ").collect::<Vec<&str>>();

    let mut configuration = Vec::new();
    let mut damaged_segments = Vec::new();

    for c in parts[0].chars() {
        configuration.push(SpringStatus::from(c));
    }

    for segment_size in parts[1].split(',') {
        damaged_segments.push(segment_size.parse::<usize>().unwrap());
    }

    (configuration, damaged_segments)
}


pub fn part1() {
    let lines: io::Lines<io::BufReader<File>> = read_lines("input/12.txt").unwrap();

    let mut data = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            data.push(parse(&line));
        }
    }

    let mut ans = 0;
    for (old_config, old_segments) in data {
        let mut config: Vec<SpringStatus> = Vec::new();
        for i in 0..5 {
            for s in &old_config {
                config.push(*s);
            }
            if i != 4 {
                config.push(SpringStatus::Unknown);
            }
        }
        let elems = old_segments.len();
        let segments = old_segments.into_iter().cycle().take(elems * 5).collect::<Vec<usize>>(); 
        let mut memo = HashMap::new();
        let count = arrange(&mut config, 
            &segments, 
            0, 
            0, 
            0,
            &mut memo
        );

        ans += count;
    }

    println!("Part1: {ans}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_configuration() {
        for s in vec![
            "#.#.### 1,1,3",
            ".#...#....###. 1,1,3",
            ".#.###.#.###### 1,3,1,6",
            "####.#...#... 4,1,1",
            "#....######..#####. 1,6,5",
            ".###.##....# 3,2,1",
        ] {
            let (a, b) = parse(s);
            assert!(valid(&a, b));
        }

    }

    #[test]
    fn calculates_outcomes() {
        let (mut config, segments) = parse("????.######..#####. 1,6,5");

        // let mut config: Vec<SpringStatus> = Vec::new();
        // for i in 0..5 {
        //     for s in &old_config {
        //         config.push(*s);
        //     }
        //     if i != 4 {
        //         config.push(SpringStatus::Unknown);
        //     }
        // }
        // let elems = old_segments.len();
        // let segments = old_segments.into_iter().cycle().take(elems * 5).collect::<Vec<usize>>();
        let mut memo = HashMap::new();
        let count = arrange(&mut config, &segments, 0, 0, 0, &mut memo);
        println!("count: {}", count);
    }
}
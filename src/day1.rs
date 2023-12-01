use std::collections::HashMap;
use std::{fs::File};
use std::io::{self};

use crate::files::read_lines;

fn calibration_value(line: &str) -> i32 {
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;
    for char in line.chars() {
        if !char.is_numeric() {
            continue;
        }
        if first.is_none() {
            first = Some(char);
        }

        last = Some(char);
    }

    return format!("{}{}", first.unwrap(), last.unwrap()).parse().unwrap();
}

fn calibration_value_v2(line: &str) -> i32 {
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;
    
    let chars: Vec<char> = line.chars().collect();

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);


    // left, right
    let mut left = 0;

    let mut nums: Vec<u32> = Vec::new();

    for index in 0..chars.len() {
        if chars[index].is_numeric() {
            left = index + 1;
            let num = chars[index].to_digit(10).unwrap();
            nums.push(num);
        } else {
            let curr_str: String = chars[left..index + 1].iter().collect();
            for written_num in map.keys() {
                if curr_str.contains(*written_num) {
                    left = index;
                    nums.push(*map.get(*written_num).unwrap());
                }
            }
        }
    }

    return format!("{}{}", nums.first().unwrap(), nums.last().unwrap()).parse().unwrap();
}


pub fn part1() {
    let lines: io::Lines<io::BufReader<File>> = read_lines("input/1.txt")
        .unwrap();

    let mut ans = 0;
    for line in lines {
        if let Ok(line) = line {
            let value = calibration_value(&line);
            ans += value;
        }
    }

    println!("Part1: {ans}")
}

pub fn part2() {
    let lines: io::Lines<io::BufReader<File>> = read_lines("input/1.txt")
        .unwrap();

    let mut ans = 0;
    for line in lines {
        if let Ok(line) = line {
            let value = calibration_value_v2(&line);
            ans += value;
        }
    }

    println!("Part2: {ans}")
}
use std::{fs::File};
use std::io::{self};

use crate::files::read_lines;

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
pub fn part1() {
    let lines: io::Lines<io::BufReader<File>> = read_lines("input/2.txt")
        .unwrap();
    const REDS: i32 = 12;
    const GREENS: i32 = 13;
    const BLUES: i32 = 14;

    let mut ans = 0;
    for line in lines {
        if let Ok(line) = line {
            let s: Vec<&str> = line.split(": ").collect();
            let x: Vec<&str> = s[0].split(" ").collect();
            let game_id: u32 = x[1].parse().unwrap();
            let rounds: Vec<&str> = s[1].split("; ").collect();
            let mut valid_game = true;
            for round in rounds {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                let balls: Vec<&str> = round.split(", ").collect();
                for ball in balls {
                    let ball: Vec<&str> = ball.split(" ").collect();
                    let count: i32 = ball[0].parse().unwrap();
                    match ball[1] {
                        "red" => {
                            red += count;
                        },
                        "green" => {
                            green += count;
                        },
                        "blue" => {
                            blue += count;
                        },
                        _ => { panic!("should not happen") }
                    }
                }
                if red > REDS || blue > BLUES || green > GREENS {
                    valid_game = false;
                    break;
                }
            }


            if valid_game {
                ans += game_id;
            }
        }
    }

    println!("part1: {ans}");
}


pub fn part2() {
    let lines: io::Lines<io::BufReader<File>> = read_lines("input/2.txt")
        .unwrap();

    let mut ans = 0;
    for line in lines {
        if let Ok(line) = line {
            let s: Vec<&str> = line.split(": ").collect();
            let x: Vec<&str> = s[0].split(" ").collect();
            let game_id: u32 = x[1].parse().unwrap();
            let rounds: Vec<&str> = s[1].split("; ").collect();

            let mut max_reds = 0;
            let mut max_blues = 0;
            let mut max_greens = 0;
            for round in rounds {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                let balls: Vec<&str> = round.split(", ").collect();
                for ball in balls {
                    let ball: Vec<&str> = ball.split(" ").collect();
                    let count: i32 = ball[0].parse().unwrap();
                    match ball[1] {
                        "red" => {
                            red += count;
                        },
                        "green" => {
                            green += count;
                        },
                        "blue" => {
                            blue += count;
                        },
                        _ => { panic!("should not happen") }
                    }
                }

                max_reds = std::cmp::max(red, max_reds);
                max_blues = std::cmp::max(blue, max_blues);
                max_greens = std::cmp::max(green, max_greens);

            }


            ans += max_reds * max_greens * max_blues;
            // println!("{max_reds} {max_greens} {max_blues}");
        }
    }

    println!("part2 : {ans}");
}
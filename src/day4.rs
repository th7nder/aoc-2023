use std::{io, fs::File, collections::{HashSet, HashMap}};

use crate::files::read_lines;

#[derive(Debug)]
struct Card {
    id: u32,
    winning: HashSet<u32>,
    owned: Vec<u32>,
    count: u32,
}

impl Card {
    fn new(id: u32, winning: HashSet<u32>, owned: Vec<u32>) -> Card {
        Card {
            id,
            winning,
            owned,
            count: 1,
        }
    }
    

    fn matching_numbers(&self) -> i32 {
        let mut matching = 0;
        for score in &self.owned {
            if self.winning.contains(&score) {
                matching += 1
            }
        }

        matching
    }
}

fn read_cards() -> Vec<Card> {
    let lines: io::Lines<io::BufReader<File>> = read_lines("input/4.txt").unwrap();

    let mut cards: Vec<Card> = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            let v: Vec<&str> = line.split(": ").collect();
            let id: Vec<&str> = v[0].split("Card").collect();
            let id = id[1].trim().parse::<u32>().unwrap();

            let numbers: Vec<&str> = v[1].split(" | ").collect();
            let winning: HashSet<u32> = numbers[0].split(" ")
                .filter(|n| n.parse::<u32>().is_ok())
                .map(|n| n.parse().unwrap()).collect();
            let owned: Vec<u32> = numbers[1].split(" ")
                .filter(|n| n.parse::<u32>().is_ok())
                .map(|n| n.parse().unwrap()).collect();
            let card = Card::new(id, winning, owned);

            cards.push(card);
        }
    }
    
    cards
}

pub fn part1() {
    let cards = read_cards();


    let mut ans = 0;
    for card in cards {
        let matching = card.matching_numbers();
        if matching > 0 {
            let card_score: i32 = (2 as i32).pow((matching - 1) as u32);
            println!("Card {} -> {}", card.id, card_score);
            ans += card_score;
        }
    }

    println!("Part1: {ans}")
}


pub fn part2() {
    let cards = read_cards();

    let mut stack = HashMap::new();
    let mut last_card = 0;
    for card in cards { 
        last_card = card.id;
        stack.insert(card.id, card);
    }


    for card_id in 1..=last_card {
        let c = stack.get_mut(&card_id).unwrap();
        let matching = c.matching_numbers();
        let won_instances = c.count;
        println!("We got {} instance of card {}", won_instances, card_id);
        if matching > 0 {
            for won_card_index in card_id + 1..=(card_id + matching as u32) {
                let c = stack.get_mut(&won_card_index).unwrap();
                c.count += won_instances;
                println!("Card {} won card: {}, instances {}", card_id, won_card_index, won_instances);
            }
        }
    }

    let mut ans = 0;
    for card_id in 1..=last_card {
        ans += stack.get(&card_id).unwrap().count;
    }

    println!("Part2: {}", ans);
}
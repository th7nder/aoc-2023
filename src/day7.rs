use std::{collections::HashMap, cmp, io, fs::File};

use crate::files::read_lines;



#[derive(PartialEq, PartialOrd, Clone, Copy, Eq, Hash, Debug, Ord)]
enum Card {
    J,
    Two,
    Three,
    Four, 
    Five, 
    Six, 
    Seven, 
    Eight, 
    Nine,
    T,
    Q,
    K,
    A 
}

impl Card {
    fn new(from: char) -> Card {
        match from {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("don't know this")
        }
    }
}

struct Hand {
    hand_kind: HandKind,
    cards: Vec<Card>,
    bid: u32
}

impl Hand { 
    fn parse(cards: &str, bid: u32) -> Hand {
        let cards: Vec<Card> = cards.chars()
            .map(|c| Card::new(c)).collect();


        Hand::new(cards, bid)
    }

    fn new(cards: Vec<Card>, bid: u32) -> Hand {
        let mut counter: HashMap<Card, u32> = HashMap::new();
        for card in &cards {
            *counter.entry(*card).or_insert(0) += 1;
        }

        let mut fives = 0;
        let mut fours = 0;
        let mut threes = 0;
        let mut twos = 0;
        let mut ones  = 0;
        let jokers = *counter.get(&Card::J).unwrap_or(&0);


        for (card, count) in counter.iter() {
            if *card == Card::J {
                continue;
            }
            match count {
                5 => fives += 1,
                4 => fours += 1,
                3 => threes += 1,
                2 => twos += 1,
                1 => ones += 1,
                _ => panic!("don't know")
            }
        }


        for _ in 0..jokers {
            // fours into fives
            if fours == 1 {
                fours -= 1;
                fives += 1;
            // full house into fours
            } else if threes == 1 && twos == 1 {
                twos -= 1;
                threes -= 1;

                fours += 1;
            // three of a kind into full hose?
            } else if threes == 1 {
                threes -= 1;
                fours += 1;
            // 2233J
            // JJ334
            } else if twos >= 1 {
                twos -= 1;
                threes += 1;
            // J3456
            } else if ones >= 1 {
                ones -= 1;
                twos += 1;
            }
        }
        
        let hand_kind = if fives == 1 {
            HandKind::FiveOfAKind
        } else if fours == 1 {
            HandKind::FourOfAKind
        } else if threes == 1 && twos == 1 {
            HandKind::FullHouse
        } else if threes == 1 {
            HandKind::ThreeOfAKind
        } else if twos == 2 {
            HandKind::TwoPair
        } else if twos == 1 {
            HandKind::OnePair
        } else if ones >= 1 {
            HandKind::HighCard
        } else if jokers == 5 || jokers == 4 {
            HandKind::FiveOfAKind
        } else {
            panic!("WOOT! {:?}", cards);
        };

        Hand {
            cards,
            hand_kind,
            bid
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_kind != other.hand_kind {
            return false;
        }

        let matching = self.cards.iter()
            .zip(other.cards.iter())
            .filter(|&(a, b)| a == b).count();

        matching == self.cards.len() && matching == other.cards.len()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_kind.partial_cmp(&other.hand_kind) {
            Some(core::cmp::Ordering::Equal) => {
                for i in 0..5 {
                    match self.cards.get(i).unwrap().partial_cmp(other.cards.get(i).unwrap()) {
                        Some(core::cmp::Ordering::Equal) => continue,
                        ord => return ord,
                    }
                }

                Some(core::cmp::Ordering::Equal)
            }
            ord => return ord,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Hand {

}

#[derive(PartialEq, PartialOrd, Eq, Debug)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn score(hands: &Vec<Hand>) -> u32 {
    let mut ans = 0;
    for (idx, hand) in hands.iter().enumerate() {
        ans += ((idx + 1) as u32) * hand.bid;
    }
    ans
}


pub fn part1() {
    let lines: io::Lines<io::BufReader<File>> = read_lines("input/6.txt").unwrap();

    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            let split: Vec<&str> = line.split(" ").collect();
            let cards = split.get(0).unwrap();
            let bid = split.get(1).unwrap().parse::<u32>().unwrap();

            hands.push(Hand::parse(cards, bid));
        }
    }

    hands.sort();

    // for hand in &hands {
    //     println!("{:?} {:?}", hand.hand_kind, hand.cards);
    // }
    println!("Part1: {}", score(&hands));
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn compares_cards() {
        assert!(Card::A > Card::T);
        assert!(Card::A == Card::A);
    }

    #[test]
    fn compares_hands() {
        assert!(HandKind::FiveOfAKind > HandKind::FullHouse);
    }

    #[test]
    fn hands() {
        let hand = Hand::new(vec![Card::A,Card::A,Card::A,Card::A,Card::A], 0);
        let hand2 = Hand::new(vec![Card::A,Card::A,Card::Eight,Card::A,Card::A], 0);
        let hand3 = Hand::new(vec![Card::Two,Card::Three,Card::Three,Card::Three,Card::Two], 0);
        let hand4 = Hand::new(vec![Card::T,Card::T,Card::T,Card::Nine,Card::Eight], 0);
        let hand5 = Hand::new(vec![Card::Two,Card::Three,Card::Four,Card::Three,Card::Two], 0);
        let hand6 = Hand::new(vec![Card::A,Card::Two,Card::Three,Card::A,Card::Four], 0);
        let hand7 = Hand::new(vec![Card::Two,Card::Three,Card::Four,Card::Five,Card::Six], 0);


        assert_eq!(HandKind::FiveOfAKind, hand.hand_kind);
        assert_eq!(HandKind::FourOfAKind, hand2.hand_kind);
        assert_eq!(HandKind::FullHouse, hand3.hand_kind);
        assert_eq!(HandKind::ThreeOfAKind, hand4.hand_kind);
        assert_eq!(HandKind::TwoPair, hand5.hand_kind);
        assert_eq!(HandKind::OnePair, hand6.hand_kind);
        assert_eq!(HandKind::HighCard, hand7.hand_kind);

    }

    #[test]
    fn hands_cmp() {
        let hand = Hand::new(vec![Card::Three,Card::Two,Card::T,Card::Three,Card::K], 765);
        let hand2 = Hand::new(vec![Card::K, Card::K, Card::Six, Card::Seven, Card::Seven], 28);
        let hand3 = Hand::new(vec![Card::Three,Card::Two,Card::T,Card::Three,Card::Nine], 765);


        assert!(hand2 > hand);
        assert!(hand > hand3);
    }

    #[test]
    fn sanity() {
        let mut hands = vec![
            Hand::parse("32T3K", 765),
            Hand::parse("T55J5", 684),
            Hand::parse("KK677", 28),
            Hand::parse("KTJJT", 220),
            Hand::parse("QQQJA", 483),
        ];

        hands.sort();

        assert_eq!(765, hands.get(0).unwrap().bid);

        assert_eq!(6440, score(&hands));
    }
}
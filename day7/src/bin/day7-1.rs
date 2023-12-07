use nom::bytes::complete::take_while;
use nom::{
    character::{complete::digit1, is_alphanumeric},
    IResult,
};
use std::cmp::Ordering;
use std::fs;
use std::str;

#[derive(Debug, Eq)]
struct Hand {
    cards: String,
    score: Score,
    bid: i32,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.score != other.score {
            self.score.partial_cmp(&other.score)
        } else {
            let mut o = other.cards.chars();
            for card in self.cards.chars() {
                let x = o.next().unwrap();
                if card != x {
                    let a = score_card(card);
                    let b = score_card(x);
                    println!("{a} > {b}");
                    let c = a.partial_cmp(&b);
                    dbg!(&c);
                    return c;
                }
            }
            return self.cards.partial_cmp(&other.cards);
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Score {
    High = 1,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

fn main() {
    let path = "day7-input.txt";
    let input = fs::read_to_string(path).expect("fail");
    let result = process(input);
    println!("{}", result);
}

fn parse_hand(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(is_alphanumeric)(i)
}

fn parse_bid(i: &str) -> IResult<&str, &str> {
    digit1(&i[1..])
}

fn score_card(card: char) -> i32 {
    match card {
        '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => card.to_string().parse::<i32>().unwrap(),
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!(),
    }
}

fn eval_hand(hand: &str) -> Score {
    let mut cards: Vec<i32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for card in hand.chars() {
        match card {
            'A' => cards[0] += 1,
            'K' => cards[1] += 1,
            'Q' => cards[2] += 1,
            'J' => cards[3] += 1,
            'T' => cards[4] += 1,
            '9' => cards[5] += 1,
            '8' => cards[6] += 1,
            '7' => cards[7] += 1,
            '6' => cards[8] += 1,
            '5' => cards[9] += 1,
            '4' => cards[10] += 1,
            '3' => cards[11] += 1,
            '2' => cards[12] += 1,
            _ => panic!(),
        }
    }
    let max = *cards.iter().max().unwrap();
    match max {
        5 => Score::Five,
        4 => Score::Four,
        3 => {
            if cards.contains(&2) {
                Score::Full
            } else {
                Score::Three
            }
        }
        2 => {
            cards.retain(|x| *x == 2);
            if cards.len() > 1 {
                Score::Two
            } else {
                Score::One
            }
        }
        1 => Score::High,
        _ => panic!(),
    }
}

pub fn process(input: String) -> i32 {
    let mut games: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let game = parse_hand(line.as_bytes()).unwrap();
        let hand = str::from_utf8(game.1).unwrap();
        let right = str::from_utf8(game.0).unwrap();
        let bid = parse_bid(right).unwrap().1.parse::<i32>().unwrap();
        let score = eval_hand(hand);
        let elem = Hand {
            cards: hand.to_string(),
            score: score,
            bid: bid,
        };
        games.push(elem);
    }
    games.sort();
    games
        .iter()
        .enumerate()
        .map(|(index, game)| game.bid * (index as i32 + 1))
        .sum()
}

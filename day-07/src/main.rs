use std::cmp::Reverse;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::mpsc::TrySendError::Full;
use anyhow::anyhow;
use utils::read_file;

fn main() {
    let lines = read_file("day-07/src/input.txt");

    let mut hands : Vec<_> = lines.iter().map(|line| Hand::from_str(line).unwrap()).collect();
    hands.sort_by_key(|x| (HandType::from(&x.cards).rank(), x.cards));
    let winnings : u64= hands.iter().enumerate().map(|(rank, hand)| (rank as u64 + 1) * hand.bid ).sum();
    println!("{:?}", winnings);

    let mut hands : Vec<_> = lines.iter().map(|line| Hand::from_str(line).unwrap()).collect();
    hands.sort_by_key(|x| {
        (
            HandType::from_part_2(&x.cards).rank(),
            x.cards.iter().map(Card::rank2).collect::<Vec<_>>(),
        )
    });
    let winnings : u64= hands.iter().enumerate().map(|(rank, hand)| (rank as u64 + 1) * hand.bid ).sum();
    println!("{:?}", winnings);

}

#[derive(Debug)]
enum HandType {
    FiveOfAKind(Card),
    FourOfAKind(Card),
    FullHouse(Card, Card),
    ThreeOfAKind(Card),
    TwoPairs(Card, Card),
    OnePairs(Card),
    HighCard(Card),
}

impl HandType {
    fn rank(&self) -> u64 {
        match self {
            Self::HighCard(_) => 0,
            Self::OnePairs(_) => 1,
            Self::TwoPairs(_, _) => 2,
            Self::ThreeOfAKind(_) => 3,
            Self::FullHouse(_, _) => 4,
            Self::FourOfAKind(_) => 5,
            Self::FiveOfAKind(_) => 6,
        }
    }

    fn from_part_2(value: &[Card; 5]) -> Self {
        let mut counts = [0; 13];
        let mut repeats: Vec<_> =
            value.iter().map(|c| {
                counts[c.rank2()] += 1;
                (*c, counts[c.rank2()])
            })
                .filter(|(_, count)| *count >= 2)
                .filter(|(card, _)| card != &Card::J)
                .collect::<HashMap<Card, u64>>()
                .into_iter()
                .collect();

        // repeats.sort_by_key(|c| c.1);
        // repeats.reverse();
        repeats.sort_by_key(|(card, count)| Reverse((*count, card.rank())));

        let jokers = counts[0];
        if jokers == 5 {
            return HandType::FiveOfAKind(Card::A);
        }

        if !repeats.is_empty() {
            repeats[0].1 += jokers;
        } else {
            repeats.push((
                value.iter().cloned().max_by_key(Card::rank2).unwrap(),
                jokers + 1,
            ))
        }

        match repeats.len() {
            2=> {
                match repeats[0] {
                    (card, 4) => HandType::FourOfAKind(card),
                    (_, 3) => HandType::FullHouse(repeats[0].0, repeats[1].0),
                    (_, 2) => HandType::TwoPairs(repeats[0].0, repeats[1].0),
                    _=> unreachable!(),
                }
            },
            1 => {
                match repeats[0] {
                    (card, 5) => HandType::FiveOfAKind(card),
                    (card, 4) => HandType::FourOfAKind(card),
                    (card, 3) => HandType::ThreeOfAKind(card),
                    (card, 2) => HandType::OnePairs(card),
                    (card, 1) => HandType::HighCard(card),
                    _=> unreachable!(),
                }
            },
            _=> {
                HandType::HighCard(*value.iter().max_by_key(|c| c.rank()).unwrap())
            }
        }
    }
}

impl From<&[Card; 5]> for HandType {
    fn from(value: &[Card; 5]) -> Self {
        let mut counts = [0; 13];
        let mut repeats: Vec<_> =
            value.iter().map(|c| {
                counts[c.rank()] += 1;
                (*c, counts[c.rank()])
            })
                .filter(|(_, count)| *count >= 2)
                .collect::<HashMap<Card, u64>>()
                .into_iter()
                .collect();

        repeats.sort_by_key(|c| c.1);
        repeats.reverse();
        match repeats.len() {
            2=> {
                match repeats[0] {
                    (_, 3) => HandType::FullHouse(repeats[0].0, repeats[1].0),
                    (_, 2) => HandType::TwoPairs(repeats[0].0, repeats[1].0),
                    _=> unreachable!(),
                }
            },
            1 => {
                match repeats[0] {
                    (card, 5) => HandType::FiveOfAKind(card),
                    (card, 4) => HandType::FourOfAKind(card),
                    (card, 3) => HandType::ThreeOfAKind(card),
                    (card, 2) => HandType::OnePairs(card),
                    _=> unreachable!(),
                }
            },
            _=> {
                HandType::HighCard(*value.iter().max_by_key(|c| c.rank()).unwrap())
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
    Num(u8),
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn rank(&self) -> usize {
        match self {
            Self::Num(num) => (*num as usize) - 2,
            Self::T => 8,
            Self::J => 9,
            Self::Q => 10,
            Self::K => 11,
            Self::A => 12,
        }
    }

    fn rank2(&self) -> usize {
        match self {
            Self::J => 0,
            Self::Num(num) => (*num as usize) - 1,
            Self::T => 9,
            Self::Q => 10,
            Self::K => 11,
            Self::A => 12,
        }
    }
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0'..='9' => Ok(Self::Num(value.to_digit(10).unwrap().try_into().unwrap())),
            'T' => Ok(Self::T),
            'J' => Ok(Self::J),
            'Q' => Ok(Self::Q),
            'K' => Ok(Self::K),
            'A' => Ok(Self::A),
            _ => Err(anyhow!("Unknown card type {}", value)),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();

        let mut chars = parts[0].chars();
        Ok(Hand {
            cards: [
                Card::try_from(chars.next().unwrap()).unwrap(),
                Card::try_from(chars.next().unwrap()).unwrap(),
                Card::try_from(chars.next().unwrap()).unwrap(),
                Card::try_from(chars.next().unwrap()).unwrap(),
                Card::try_from(chars.next().unwrap()).unwrap(),
            ],
            bid: parts[1].parse().unwrap(),
        })
    }
}

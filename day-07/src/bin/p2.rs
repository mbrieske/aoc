use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Card {
    A = 14,
    K = 13,
    Q = 12,
    J = 1,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl Card {
    fn from(c: char) -> Card {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    bid: usize,
    cards: [Card; 5],
    r#type: Type,
}

impl Hand {
    fn from(line: &str) -> Hand {
        let cards = line
            .chars()
            .take(5)
            .map(Card::from)
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();
        let bid = line.split_once(' ').unwrap().1.parse::<usize>().unwrap();
        let r#type = Hand::get_type(&cards);
        Hand { bid, cards, r#type }
    }

    fn get_type(cards: &[Card; 5]) -> Type {
        let mut counts: HashMap<&Card, u8> = HashMap::new();

        for card in cards.iter() {
            *counts.entry(card).or_insert(0) += 1;
        }
        let jokers = counts.remove(&Card::J).unwrap_or(0);

        let mut counts = counts.into_values().collect::<Vec<u8>>();
        counts.sort();
        if let Some(most) = counts.last_mut() {
            *most += jokers;
        } else {
            counts.push(jokers);
        }

        match counts.pop().unwrap() {
            5 => Type::FiveOfAKind,
            4 => Type::FourOfAKind,
            3 => match counts.pop().unwrap() {
                2 => Type::FullHouse,
                _ => Type::ThreeOfAKind,
            },
            2 => match counts.pop().unwrap() {
                2 => Type::TwoPair,
                _ => Type::OnePair,
            },
            1 => Type::HighCard,
            _ => unreachable!(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.r#type != other.r#type {
            self.r#type.cmp(&other.r#type)
        } else {
            let (my_card, other_card) = self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find(|(my_card, other_card)| my_card != other_card)
                .unwrap();
            my_card.cmp(other_card)
        }
    }
}

fn main() {
    let reader = BufReader::new(File::open("res/input").unwrap());
    println!("{}", puzzle(reader));
}

fn puzzle<R: BufRead>(reader: R) -> usize {
    let lines = reader.lines().map_while(Result::ok);
    let mut hands = lines.map(|line| Hand::from(&line)).collect::<Vec<Hand>>();
    hands.sort();
    hands
        .iter()
        .zip(1_usize..)
        .fold(0, |acc, (hand, rank)| acc + rank * hand.bid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn example() {
        let example = String::from(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );

        assert_eq!(puzzle(BufReader::new(Cursor::new(example))), 5905);
    }

    #[test]
    fn card_comp() {
        let c1 = Card::Ten;
        let c2 = Card::Nine;
        assert!(c1 > c2);
        assert!(c1 != c2);
    }

    #[test]
    fn type_comp() {
        assert!(Type::FiveOfAKind > Type::FourOfAKind);
        assert!(Type::FourOfAKind == Type::FourOfAKind);
    }

    #[test]
    fn create_hand() {
        let line = "T55J5 684";
        let expected = Hand {
            bid: 684,
            cards: [Card::Ten, Card::Five, Card::Five, Card::J, Card::Five],
            r#type: Type::FourOfAKind,
        };
        assert_eq!(Hand::from(line), expected);
    }

    #[test]
    fn calculate_type() {
        let cards = [Card::Ten, Card::Ten, Card::Ten, Card::J, Card::Five];
        let expected = Type::FourOfAKind;
        assert_eq!(Hand::get_type(&cards), expected);
    }

    #[test]
    fn compare_hands_1() {
        let hand1 = Hand::from("T9876 123");
        let hand2 = Hand::from("954A2 123");

        assert!(hand1 > hand2);
    }

    #[test]
    fn compare_hands_2() {
        let hand1 = Hand::from("TT876 123");
        let hand2 = Hand::from("954A2 123");

        assert!(hand1 > hand2);
    }

    #[test]
    fn compare_hands_3() {
        let hand1 = Hand::from("TT876 123");
        let hand2 = Hand::from("95AA2 123");

        assert!(hand1 > hand2);
    }
}

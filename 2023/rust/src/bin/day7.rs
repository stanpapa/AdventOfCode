use std::{collections::HashMap, hash::Hash, io::Error};

use libaoc::io::input::Input;

#[derive(Debug, Default, PartialEq, Ord, PartialOrd, Eq, Hash, Clone, Copy)]
enum CamelCard1 {
    Numbered(u8),
    T,
    J,
    Q,
    K,
    #[default]
    A,
}

#[derive(Debug, Default, PartialEq, Ord, PartialOrd, Eq, Hash, Clone, Copy)]
enum CamelCard2 {
    J,
    Numbered(u8),
    T,
    Q,
    K,
    #[default]
    A,
}

trait CamelCard:
    std::fmt::Debug + Default + PartialEq + Ord + PartialOrd + Eq + Clone + Copy + Hash
{
    fn new(c: &char) -> Self;

    fn find_hand_type(card_count: &HashMap<Self, u8>, hand: &[Self; 5]) -> CamelCardHand<Self>;
}

impl CamelCard for CamelCard1 {
    fn new(c: &char) -> Self {
        match c {
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => Self::Numbered(c.to_digit(10).unwrap() as u8),
        }
    }

    /// Determine type of hand
    ///
    /// The type is determined by how many occurences of a card type there are
    fn find_hand_type(card_count: &HashMap<Self, u8>, hand: &[Self; 5]) -> CamelCardHand<Self> {
        if card_count.values().any(|&count| count == 5) {
            CamelCardHand::FiveOfAKind(*hand)
        } else if card_count.values().any(|&count| count == 4) {
            CamelCardHand::FourOfAKind(*hand)
        } else if card_count.values().any(|&count| count == 3) {
            if card_count.values().any(|&count| count == 2) {
                CamelCardHand::FullHouse(*hand)
            } else {
                CamelCardHand::ThreeOfAKind(*hand)
            }
        } else if card_count.values().any(|&count| count == 2) {
            if card_count.values().filter(|&count| *count == 2).count() == 2 {
                CamelCardHand::TwoPair(*hand)
            } else {
                CamelCardHand::OnePair(*hand)
            }
        } else {
            CamelCardHand::HighCard(*hand)
        }
    }
}

impl CamelCard for CamelCard2 {
    fn new(c: &char) -> Self {
        match c {
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => Self::Numbered(c.to_digit(10).unwrap() as u8),
        }
    }

    /// Determine type of hand
    ///
    /// The type is determined by how many occurences of a card type there are
    fn find_hand_type(card_count: &HashMap<Self, u8>, hand: &[Self; 5]) -> CamelCardHand<Self> {
        let count_joker = card_count.get(&Self::J);

        if card_count.values().any(|&count| count == 5) {
            CamelCardHand::FiveOfAKind(*hand)
        } else if card_count.values().any(|&count| count == 4) {
            match count_joker {
                Some(n) => {
                    if *n == 4 || *n == 1 {
                        // 4 jokers + 1 card or 1 joker + 4 cards
                        return CamelCardHand::FiveOfAKind(*hand);
                    } else {
                        // 4 identical cards, no joker
                        return CamelCardHand::FourOfAKind(*hand);
                    }
                }
                None => CamelCardHand::FourOfAKind(*hand),
            }
        } else if card_count.values().any(|&count| count == 3) {
            match count_joker {
                Some(n) => {
                    if *n == 3 {
                        if card_count.values().any(|&count| count == 2) {
                            // 3 jokers + 2 identical cards
                            return CamelCardHand::FiveOfAKind(*hand);
                        } else {
                            // 3 jokers + any other card
                            return CamelCardHand::FourOfAKind(*hand);
                        }
                    } else if *n == 2 {
                        // 2 jokers + 3 other cards
                        return CamelCardHand::FiveOfAKind(*hand);
                    } else {
                        // 1 jokers + 3 other cards
                        return CamelCardHand::FourOfAKind(*hand);
                    }
                }
                None => {
                    if card_count.values().any(|&count| count == 2) {
                        return CamelCardHand::FullHouse(*hand);
                    } else {
                        return CamelCardHand::ThreeOfAKind(*hand);
                    }
                }
            }
        } else if card_count.values().any(|&count| count == 2) {
            match count_joker {
                Some(n) => {
                    if *n == 2 && card_count.values().filter(|&count| *count == 2).count() == 2 {
                        // 2 jokers + another pair
                        return CamelCardHand::FourOfAKind(*hand);
                    } else {
                        // 1 joker, 2 pairs
                        if card_count.values().filter(|&count| *count == 2).count() == 2 {
                            return CamelCardHand::FullHouse(*hand);
                        } else {
                            // 1 joker + a pair
                            return CamelCardHand::ThreeOfAKind(*hand);
                        }
                    }
                }
                None => {
                    if card_count.values().filter(|&count| *count == 2).count() == 2 {
                        // 2 pairs, no joker
                        return CamelCardHand::TwoPair(*hand);
                    } else {
                        // one pair
                        return CamelCardHand::OnePair(*hand);
                    }
                }
            }
        } else {
            match count_joker {
                Some(_n) => CamelCardHand::OnePair(*hand),
                None => CamelCardHand::HighCard(*hand),
            }
        }
    }
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
enum CamelCardHand<T: CamelCard> {
    HighCard([T; 5]),
    OnePair([T; 5]),
    TwoPair([T; 5]),
    ThreeOfAKind([T; 5]),
    FullHouse([T; 5]),
    FourOfAKind([T; 5]),
    FiveOfAKind([T; 5]),
}

/// Parses a hand and converts it into a fixed-size array of CamelCards
fn convert_hand<T: CamelCard>(hand: &str) -> [T; 5] {
    // collect Vec of CamelCards
    let cards = hand.chars().map(|c| T::new(&c)).collect::<Vec<T>>();

    // convert Vec to fixed-size array
    let mut array: [T; 5] = Default::default();
    array.copy_from_slice(&cards);

    // return fixed-size array
    array
}

/// Count the number of occurences of a card type
///
/// # Returns
/// number of occurences per card type
fn count_hand<T: CamelCard>(hand: &[T; 5]) -> HashMap<T, u8> {
    let mut count_cards = HashMap::new();

    hand.iter().for_each(|&card| {
        let count = count_cards.entry(card).or_insert(0);
        *count += 1;
    });

    count_cards
}

fn solve<T: CamelCard>(hands: &str) -> u64 {
    let mut hand_types = hands
        .lines()
        .map(|hand_bid| {
            // split line in the hand of cards and the bid
            let (h, bid) = hand_bid.split_once(' ').unwrap();

            // convert the hand into a fixed-size array of CamelCard enums
            let hand = convert_hand(h);

            // count how many occurences there are per card type
            let card_count = count_hand(&hand);

            // determine type of hand
            let hand_type = T::find_hand_type(&card_count, &hand);

            // return type of hand together with the bid
            (hand_type, bid.parse::<u64>().unwrap())
        })
        .collect::<Vec<(CamelCardHand<T>, u64)>>();

    // sort hands
    hand_types.sort();

    // calculate winnings
    hand_types
        .iter()
        .enumerate()
        .map(|(i, (_hand_type, bid))| (i as u64 + 1) * bid)
        .sum()
}

fn main() -> Result<(), Error> {
    let inp = Input::new().to_string();

    println!("{}", solve::<CamelCard1>(&inp));
    println!("{}", solve::<CamelCard2>(&inp));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part_1() {
        assert_eq!(super::solve::<CamelCard1>(INPUT), 6440);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::solve::<CamelCard2>(INPUT), 5905);
    }
}

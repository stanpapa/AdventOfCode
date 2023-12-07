use std::{collections::HashMap, io::Error};

use libaoc::io::input::Input;

#[derive(Debug, Default, PartialEq, Ord, PartialOrd, Eq, Hash, Clone, Copy)]
enum CamelCard {
    Numbered(u8),
    T,
    J,
    Q,
    K,
    #[default]
    A,
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
enum CamelCardHand {
    HighCard([CamelCard; 5]),
    OnePair([CamelCard; 5]),
    TwoPair([CamelCard; 5]),
    ThreeOfAKind([CamelCard; 5]),
    FullHouse([CamelCard; 5]),
    FourOfAKind([CamelCard; 5]),
    FiveOfAKind([CamelCard; 5]),
}

/// Parses a hand and converts it into a fixed-size array of CamelCards
fn convert_hand(hand: &str) -> [CamelCard; 5] {
    // collect Vec of CamelCards
    let cards = hand
        .chars()
        .map(|c| match c {
            'T' => CamelCard::T,
            'J' => CamelCard::J,
            'Q' => CamelCard::Q,
            'K' => CamelCard::K,
            'A' => CamelCard::A,
            _ => CamelCard::Numbered(c.to_digit(10).unwrap() as u8),
        })
        .collect::<Vec<CamelCard>>();

    // convert Vec to fixed-size array
    let mut array: [CamelCard; 5] = Default::default();
    array.copy_from_slice(&cards);

    // return fixed-size array
    array
}

/// Count the number of occurences of a card type
///
/// # Returns
/// number of occurences per card type
fn count_hand(hand: &[CamelCard; 5]) -> HashMap<CamelCard, u8> {
    let mut count_cards = HashMap::new();

    hand.iter().for_each(|&card| {
        let count = count_cards.entry(card).or_insert(0);
        *count += 1;
    });

    count_cards
}

/// Determine type of hand
///
/// The type is determined by how many occurences of a card type there are
fn find_hand_type(card_count: &HashMap<CamelCard, u8>, hand: &[CamelCard; 5]) -> CamelCardHand {
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

fn part_1(hands: &str) -> u64 {
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
            let hand_type = find_hand_type(&card_count, &hand);

            // return type of hand together with the bid
            (hand_type, bid.parse::<u64>().unwrap())
        })
        .collect::<Vec<(CamelCardHand, u64)>>();

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

    println!("{}", part_1(&inp));
    // println!("{}", part_2(&inp));

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT), 6440);
    }

    // #[test]
    // fn part_2() {
    //     assert_eq!(super::part_2(INPUT), 5905);
    // }
}

use aoc::*;
use counter::Counter;

#[lines]
fn first(lines: Vec<String>) -> usize {
    total_earnings(lines.into_iter().map(Hand::new).collect())
}

#[lines]
fn second(lines: Vec<String>) -> usize {
    total_earnings(lines.into_iter().map(Hand::new_jokers).collect())
}

fn total_earnings(mut hands: Vec<Hand>) -> usize {
    hands.sort();
    hands.into_iter().enumerate().map(|(i, Hand{bid, ..})| (i + 1) * bid).sum()
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct Hand {
    typ: HandType,
    hand: (usize, usize, usize, usize, usize),
    bid: usize,
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    Nothing,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
use HandType::*;

impl Hand {
    fn new(handstr: String) -> Self {
        let (cards, hand, bid) = Self::parse_string(handstr);
        let typ = Self::determine_type(&cards);
        Hand {
            typ,
            hand,
            bid,
        }
    }

    fn new_jokers(handstr: String) -> Self {
        let (cards, hand, bid) = Self::parse_string(handstr.replace("J", "!"));
        let desired_card = cards.iter().filter(|&&c| c > 1).cloned().collect::<Counter<_>>().into_iter().map(|(card, count)| (count, card)).max().map(|(_, card)| card).unwrap_or(14);
        let new_cards: Vec<_> = cards.iter().cloned().map(|c| match c {1 => desired_card, card => card}).collect();
        let typ = Self::determine_type(&new_cards);
        Hand {
            typ,
            hand,
            bid,
        }
    }

    fn parse_string(handstr: String) -> (Vec<usize>, (usize, usize, usize, usize, usize), usize) {
        let bid = (&handstr[6..]).parse().expect("Non-number bid");
        let cards: Vec<usize> = (&handstr[0..5]).chars().map(|c| c.to_digit(10).unwrap_or_else(|| match c {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            '!' => 1,
            c => panic!("Unknown card {c}"),
        }) as usize).collect();
        let [a, b, c, d, e] = cards[..] else { panic!("Hand different than 5 cards") };
        let hand = (a, b, c, d, e);
        (cards, hand, bid)
    }

    fn determine_type(cards: &Vec<usize>) -> HandType {
        let counts: Counter<_> = cards.iter().collect();
        let mut values: Vec<_> = counts.values().cloned().collect();
        values.sort();
        match values[..] {
            [5] => FiveOfAKind,
            [1, 4] => FourOfAKind,
            [2, 3] => FullHouse,
            [1, 1, 3] => ThreeOfAKind,
            [1, 2, 2] => TwoPairs,
            [1, 1, 1, 2] => Pair,
            [1, 1, 1, 1, 1] => Nothing,
            _ => panic!("weird hand"),
        }
    }
}

const EXAMPLE: &str = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

aoc!();
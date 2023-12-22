use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone)]
struct Hand {
    hand_string: String,
    hand_strength: HandType,
    hand_bid: u32,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveKind = 6,
    FourKind = 5,
    FullHouse = 4,
    ThreeKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

pub fn answer(input_path: &str) -> Result<u64, &'static str> {
    let lines: Vec<Hand> = read_to_string(input_path)
        .map_err(|_| "Unable to read input string.")?
        .lines()
        .map(|line| parse_hand(line))
        .collect::<Result<Vec<Hand>, &'static str>>()?;

    let mut sorted_lines = lines.clone();
    sorted_lines.sort_by(|a, b| {
        a.hand_strength.cmp(&b.hand_strength).then_with(|| {
            let a_strength: Vec<u32> = hand_to_strengths(&a.hand_string);
            let b_strength: Vec<u32> = hand_to_strengths(&b.hand_string);

            a_strength.cmp(&b_strength)
        })
    });

    let mut output: u64 = 0;
    for (index, hand) in sorted_lines.iter().enumerate() {
        output += hand.hand_bid as u64 * (index + 1) as u64;
    }

    Ok(output)
}

fn parse_hand(line: &str) -> Result<Hand, &'static str> {
    let parts = validate_hand(line)?;

    // Get the number of instances each card occurrs
    let hand_hash = parts[0].chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    let mut sorted_hand: Vec<(&char, &u16)> = hand_hash.iter().collect();
    sorted_hand.sort_by(|a, b| {
        let a_strength = card_to_strength(a.0).unwrap_or(0);
        let b_strength = card_to_strength(b.0).unwrap_or(0);

        b.1.cmp(a.1).then_with(|| b_strength.cmp(&a_strength))
    });
    let top_two: Vec<(&char, &u16)> = sorted_hand.into_iter().take(2).collect();
    let hand_strength = get_hand_strength(top_two)?;

    Ok(Hand {
        hand_string: parts[0].to_string(),
        hand_strength: hand_strength,
        hand_bid: parts[1]
            .parse::<u32>()
            .map_err(|_| "Unable to parse hand bid")?,
    })
}

fn get_hand_strength(top_two_cards: Vec<(&char, &u16)>) -> Result<HandType, &'static str> {
    match top_two_cards
        .first()
        .ok_or("Unable to get first card from hand")?
    {
        // High Card
        (&_, &1) => Ok(HandType::HighCard),
        // One Pair/Two Pair
        (&_, &2) => {
            if top_two_cards
                .get(1)
                .ok_or("Unable to get second card from hand")?
                .1
                == &2
            {
                Ok(HandType::TwoPair)
            } else {
                Ok(HandType::OnePair)
            }
        }
        // Three of a kind/Full house
        (&_, &3) => {
            if top_two_cards
                .get(1)
                .ok_or("Unable to get second card from hand")?
                .1
                == &2
            {
                Ok(HandType::FullHouse)
            } else {
                Ok(HandType::ThreeKind)
            }
        }
        // Four of a kind
        (&_, &4) => Ok(HandType::FourKind),
        // Five of a kind
        (&_, &5) => Ok(HandType::FiveKind),
        _ => return Err("Error parsing hand"),
    }
}

fn validate_hand(line: &str) -> Result<Vec<&str>, &'static str> {
    // Validate line
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 2 {
        return Err("Unable to parse hand, no bid found.");
    }
    if parts[0].len() != 5 {
        println!("{}", parts[1]);
        return Err("Hand length was invalid.");
    }

    Ok(parts)
}

fn hand_to_strengths(hand: &str) -> Vec<u32> {
    hand.chars().filter_map(|c| card_to_strength(&c)).collect()
}

fn card_to_strength(c: &char) -> Option<u32> {
    match c {
        'A' => Some(13),
        'K' => Some(12),
        'Q' => Some(11),
        'J' => Some(10),
        'T' => Some(9),
        '9' => Some(8),
        '8' => Some(7),
        '7' => Some(6),
        '6' => Some(5),
        '5' => Some(4),
        '4' => Some(3),
        '3' => Some(2),
        '2' => Some(1),
        '1' => Some(0),
        _ => None,
    }
}

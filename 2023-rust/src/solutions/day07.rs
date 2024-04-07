use std::cmp::Ordering;

fn cards_to_vec(string: &str, joker_rule: bool) -> Vec<i64> {
    let mut cards = vec![0; 13];
    for char in string.chars() {
        match char {
            'A' => { cards[12] += 1; },
            'K' => { cards[11] += 1; },
            'Q' => { cards[10] += 1; },
            'J' => { cards[9] += 1; },
            'T' => { cards[8] += 1; },
            '2'..='9' => { cards[(char as usize) - ('2' as usize)] += 1; },
            _ => {}
        }
    }
    if joker_rule && cards[9] > 0 {
        let (max, count) = cards.iter().enumerate().max_by_key(|(_,&x)| x).unwrap();
        let joker_count = cards[9];
        if max != 9 {
            cards[max] += joker_count;
        } else if *count < 5 {
            cards[9] = 0;
            let (max, _) = cards.iter().enumerate().max_by_key(|(_,&x)| x).unwrap();
            cards[max] += joker_count;
        }
    }
    cards
}

fn get_hand_value(cards: &[i64]) -> i64 {
    match cards.iter().max() {
        Some(&max) if max == 5 => 7,
        Some(&max) if max == 4 => 6,
        Some(&max) if max == 3 => if cards.contains(&2) { 5 } else { 4 },
        Some(&max) if max == 2 => if cards.iter().filter(|&&x| x == 2).count() == 2 { 3 } else { 2 },
        _ => 1,
    }
}

fn get_card_value(card: &char, joker_rule: bool) -> i64 {
    match card {
        'A' => { 14 },
        'K' => { 13 },
        'Q' => { 12 },
        'J' => if joker_rule { 1 } else { 11 },
        'T' => { 10 },
        '9' => { 9 },
        '8' => { 8 },
        '7' => { 7 },
        '6' => { 6 },
        '5' => { 5 },
        '4' => { 4 },
        '3' => { 3 },
        '2' => { 2 },
        _ => panic!()
    }
}

fn compare(a: &(&str, i64), b: &(&str, i64), joker_rule: bool) -> Ordering {
    if a.0 == b.0 { return Ordering::Equal; }
    let cards_a = cards_to_vec(a.0, joker_rule);
    let cards_b = cards_to_vec(b.0, joker_rule);
    let value_a = get_hand_value(&cards_a);
    let value_b = get_hand_value(&cards_b);

    if value_a > value_b {
        Ordering::Greater
    } else if value_a < value_b {
        Ordering::Less
    } else {
        for (a, b) in  a.0.chars().zip(b.0.chars()) {
            let value_a = get_card_value(&a, joker_rule);
            let value_b = get_card_value(&b, joker_rule);
            if value_a > value_b {
                return Ordering::Greater
            } else if value_a < value_b {
                return Ordering::Less
            }
        }
        Ordering::Equal
    }
}

fn parse(data: &str) ->Vec<(&str, i64)> {
    data
        .lines()
        .map(|x| x
            .split_once(' ')
            .map(|(x, y)| (x, y.parse::<i64>().unwrap()))
            .unwrap()
        )
        .collect()
}

pub fn part1(data: &str) -> i64 {
    let mut data = parse(data);
    data.sort_by(|a,  b| compare(a, b, false));
    data.into_iter().enumerate().map(|(i, (_, y))| (i as i64 + 1) * y).sum()
}

pub fn part2(data: &str) -> i64 {
    let mut data = parse(data);
    data.sort_by(|a,  b| compare(a, b, true));
    data.into_iter().enumerate().map(|(i, (_, y))| (i as i64 + 1) * y).sum()
}
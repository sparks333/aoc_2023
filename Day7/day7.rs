use std::fs::read_to_string;
use std::cmp::Ordering;

fn convert_label(c:char) -> usize
{
    match c
    {
        'A'=>return 12,
        'K'=>return 11,
        'Q'=>return 10,
        'J'=>return 9,
        'T'=>return 8, 
        '9'=>return 7,
        '8'=>return 6,
        '7'=>return 5,
        '6'=>return 4,
        '5'=>return 3,
        '4'=>return 2,
        '3'=>return 1,
        '2'=>return 0,
        _=>return 0
    }
}

fn convert_label_joker(c:char) -> usize
{
    match c
    {
        'A'=>return 12,
        'K'=>return 11,
        'Q'=>return 10,
        'T'=>return 9, 
        '9'=>return 8,
        '8'=>return 7,
        '7'=>return 6,
        '6'=>return 5,
        '5'=>return 4,
        '4'=>return 3,
        '3'=>return 2,
        '2'=>return 1,
        'J'=>return 0,
        _=>return 0
    }
}
struct Hand
{
    hand:Vec<char>,
    card_count:Vec<i32>,
    bid:i32,
    wild:bool
}

impl Hand 
{
    pub fn new(hand:Vec<char>, bid:i32, wild:bool) -> Self 
    {
        let mut card_count:Vec<i32> = vec![0; 13];
        let mut wild_count = 0;
        
        for c in &hand
        {
            if wild
            {
                if *c == 'J'
                {
                    wild_count += 1;
                }
                else
                {
                    card_count[convert_label(*c)] += 1; 
                }
            }
            else 
            {
                card_count[convert_label(*c)] += 1;
            }
        }

        card_count.sort();
        card_count.reverse();

        if wild
        {
            card_count[0] += wild_count;
        }

        Self {
            hand: hand,
            card_count: card_count,
            bid: bid,
            wild: wild,
        }
    }

    pub fn cmp(&self, other: &Hand) -> Ordering
    {
        for i in 0..5
        {
            if self.card_count[i] != other.card_count[i]
            {
                return self.card_count[i].cmp(&other.card_count[i]); 
            }
        } 

        for j in 0..self.hand.len()
        {
            if self.hand[j] != other.hand[j]
            {
                if self.wild
                {
                    return convert_label_joker(self.hand[j]).cmp(&convert_label_joker(other.hand[j]));
                }
                else 
                {
                    return convert_label(self.hand[j]).cmp(&convert_label(other.hand[j]));
                }
            }
        }
        return Ordering::Equal;
    }

}

fn main() 
{

    let mut board:Vec<Hand> = vec![];

    for line in read_to_string("input.txt").unwrap().lines()
    {
        let split_line:Vec<String> = line.split(" ").map(str::to_string).collect();
        let hand:Vec<char> = split_line[0].chars().collect();
        let bid = split_line[1].parse::<i32>().unwrap();

        board.push(Hand::new(hand, bid, false));

    }

    board.sort_by(|a, b| a.cmp(b));

    let mut total:i64 = 0;
    let mut index = 1;

    for h in board
    {
        total += (index * h.bid) as i64;
        index += 1;
    }

    println!("Part 1: {}", total);

    board = vec![];

    for line in read_to_string("input.txt").unwrap().lines()
    {
        let split_line:Vec<String> = line.split(" ").map(str::to_string).collect();
        let hand:Vec<char> = split_line[0].chars().collect();
        let bid = split_line[1].parse::<i32>().unwrap();

        board.push(Hand::new(hand, bid, true));
    }

    board.sort_by(|a, b| a.cmp(b));

    total = 0;
    index = 1;

    for h in board
    {
        total += (index * h.bid) as i64;
        index += 1;
    }

    println!("Part 2: {}", total);

}

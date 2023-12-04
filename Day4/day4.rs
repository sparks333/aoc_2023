use std::{fs::read_to_string};

fn score(board:&Vec<String>, index:usize) -> i32
{
    let line:String = (*board[index]).to_string();
    let split_line:Vec<String> = line.split(|x| (x == ' ') || (x == ':') || (x == ',')).filter(|s| !s.is_empty()).map(str::to_string).collect();
    let separator = split_line.iter().position(|x| x == "|").unwrap();

    let mut common:Vec<String> = vec![];

    for i in 2..separator
    {
        for j in (separator+1)..split_line.len()
        {
            if split_line[i] == split_line[j]
            {
                common.push(split_line[i].clone());
            }
        }
    }
    
    let mut total = common.len() as i32;
    for i in 0..common.len()
    {
        total += score(board, index+i+1);
    }
    return total;

}

fn main() {

    let mut total = 0;

    for line in read_to_string("input.txt").unwrap().lines()
    {
        let split_line:Vec<String> = line.split(|x| (x == ' ') || (x == ':') || (x == ',')).filter(|s| !s.is_empty()).map(str::to_string).collect();
        let separator = split_line.iter().position(|x| x == "|").unwrap();

        let mut common:Vec<String> = vec![];

        for i in 2..separator
        {
            for j in (separator+1)..split_line.len()
            {
                if split_line[i] == split_line[j]
                {
                    common.push(split_line[i].clone());
                }
            }
        }

        let common_len = common.len() as u32;

        if common_len != 0
        {
            total += i32::pow(2, common_len - 1);
        }
    }
    println!("Part 1: {}", total);

    let mut board:Vec<String> = vec![];

    for line in read_to_string("input.txt").unwrap().lines()
    {
        board.push(line.to_string());
    }

    total = board.len() as i32;
    for i in 0..board.len()
    {
        total += score(&board, i);
    }

    println!("Part 2: {}", total);

}
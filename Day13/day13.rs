use std::{fs::read_to_string, cmp::min};

fn find_horiz_smudge(map:&Vec<Vec<char>>) -> i32
{
    let mut errors = 0;
    for i in 0..map[0].len()-1
    {
        let mut found = true;
        for j in 0..min(i+1, map[0].len() - (i+1))
        {
            let idx1 = i-j;
            let idx2 = i+j+1;
            for k in 0..map.len()
            {
                if map[k][idx1] != map[k][idx2]
                {
                    errors += 1;
                    if errors > 1
                    {
                        found = false;
                        errors = 0;
                        break;
                    }
                }
            }
            if found == false
            {
                break;
            }
        }
        if found && errors == 1
        {
            return i as i32;
        }
    }
    return -1;
}

fn find_vert_smudge(map:&Vec<Vec<char>>) -> i32
{
    let mut errors = 0;
    for i in 0..map.len()-1
    {
        let mut found = true;
        for j in 0..min(i+1, map.len() - (i+1))
        {
            let idx1 = i-j;
            let idx2 = i+j+1;
            for k in 0..map[0].len()
            {
                if map[idx1][k] != map[idx2][k]
                {
                    errors += 1;
                    if errors > 1
                    {
                        found = false;
                        errors = 0;
                        break;
                    }
                }
            }
            if found == false
            {
                break;
            }
        }
        if found && errors == 1
        {
            return i as i32;
        }
    }
    return -1;
}

fn find_horiz(map:&Vec<Vec<char>>) -> i32
{
    for i in 0..map[0].len()-1
    {
        let mut found = true;
        for j in 0..min(i+1, map[0].len() - (i+1))
        {
            let idx1 = i-j;
            let idx2 = i+j+1;
            for k in 0..map.len()
            {
                if map[k][idx1] != map[k][idx2]
                {
                    found = false;
                    break;
                }
            }
            if found == false
            {
                break;
            }
        }
        if found
        {
            return i as i32;
        }
    }
    return -1;
}

fn find_vert(map:&Vec<Vec<char>>) -> i32
{
    for i in 0..map.len()-1
    {
        let mut found = true;
        for j in 0..min(i+1, map.len() - (i+1))
        {
            let idx1 = i-j;
            let idx2 = i+j+1;
            for k in 0..map[0].len()
            {
                if map[idx1][k] != map[idx2][k]
                {
                    found = false;
                    break;
                }
            }
            if found == false
            {
                break;
            }
        }
        if found
        {
            return i as i32;
        }
    }
    return -1;
}

fn main() {

    let mut board:Vec<Vec<Vec<char>>> = vec![];
    let mut sub_board:Vec<Vec<char>> = vec![];

    for line in read_to_string("input.txt").unwrap().lines()
    {
        if line.len() == 0
        {
            board.push(sub_board);
            sub_board = vec![];
        }
        else
        {
            sub_board.push(line.chars().collect());
        }
    }
    board.push(sub_board);

    let mut tot = 0;
    for j in &board
    {
        let x = find_horiz(&j);
        if x == -1
        {
            let y = find_vert(&j);
            tot += 100*(y+1);
        }
        else 
        {
            tot += x+1;    
        }
    }
    println!("Part 1: {}", tot);

    tot = 0;

    for j in &board
    {
        let x = find_horiz_smudge(&j);
        if x == -1
        {
            let y = find_vert_smudge(&j);
            tot += 100*(y+1);
        }
        else 
        {
            tot += x+1;    
        }
    }
    println!("Part 2: {}", tot);

}

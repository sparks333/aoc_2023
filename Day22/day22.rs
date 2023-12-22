use std::{fs::read_to_string, cmp};

fn main() {

    let mut bricks:Vec<((i32, i32, i32), (i32, i32, i32))> = vec![];

    for line in read_to_string("input.txt").unwrap().lines() {
        let split_line:Vec<String> = line.split(['~', ',']).map(str::to_string).collect();
        let x1 = split_line[0].parse::<i32>().unwrap();
        let y1 = split_line[1].parse::<i32>().unwrap();
        let z1 = split_line[2].parse::<i32>().unwrap();
        let x2 = split_line[3].parse::<i32>().unwrap();
        let y2 = split_line[4].parse::<i32>().unwrap();
        let z2 = split_line[5].parse::<i32>().unwrap();
        bricks.push(((x1, y1, z1), (x2, y2, z2)));
    }

    let mut board:Vec<(((i32, i32, i32), (i32, i32, i32)), Vec<usize>, bool)> = vec![];

    bricks.sort_by(|a, b| a.0.2.partial_cmp(&b.0.2).unwrap());
    bricks.reverse();

    board.push((bricks.pop().unwrap(), vec![], true));

    loop
    {
        if bricks.len() == 0 { break; }
        let mut a = bricks.pop().unwrap();
        let mut support_index = vec![];
        loop {
            let mut supported = false;
            for idx in 0..board.len()
            {
                let b = board[idx].0;
                if a.0.2 == 1 || a.1.2 == 1 { supported = true; break; }
                if cmp::max(a.0.0, b.0.0) <= cmp::min(a.1.0, b.1.0) &&
                   cmp::max(a.0.1, b.0.1) <= cmp::min(a.1.1, b.1.1) &&
                   cmp::max(a.0.2-1, b.0.2) <= cmp::min(a.1.2-1, b.1.2) {
                    support_index.push(idx);
                    supported = true;
                }
            }
            if supported
            {
                break;
            }
            a.0.2 -=1;
            a.1.2 -=1;
        }
        board.push((a, support_index, true));
    }

    let mut safe = 0;

    for i in 0..board.len()
    {
        let mut done: bool = false;
        for j in 0..board.len()
        {
            if board[j].1.contains(&i) && board[j].1.len() == 1
            {
                done = true;
                break;
            }
        }
        if !done
        {
            safe += 1;
        }
    }

    println!("Part 1: {}", safe);

    for i in 0..board.len()
    {
        if board[i].1.len() == 0
        {
            board[i].2 = false;
        }
    }

    let mut tot = 0;
    for i in 0..board.len()
    {
        let mut disintegrate = vec![i];
        let mut new_board = board.clone();
        loop {
            let mut done = true;
            let mut new_disintegrate = vec![];
            for d in 0..disintegrate.len()
            {
                for j in 0..new_board.len()
                {
                    if new_board[j].2 { new_board[j].1.retain(|&x| x != disintegrate[d]); }
                }
                for j in 0..new_board.len()
                {
                    if new_board[j].2 == true && new_board[j].1.len() == 0 && new_board[j].0.0.2 > 1
                    {
                        done = false;
                        new_disintegrate.push(j);
                        new_board[j].2 = false;
                    }
                }
            }
            disintegrate = new_disintegrate.clone();
            tot += new_disintegrate.len();
            if done == true
            {
                break;
            }
        }
    }
    println!("Part 2: {}", tot);
}

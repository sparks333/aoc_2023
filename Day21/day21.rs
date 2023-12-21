use std::fs::read_to_string;
use std::collections::HashMap;
use polyfit_rs::polyfit_rs::polyfit;

const PART_1_STEPS:i64 = 64;
const PART_2_STEPS:f64 = 26501365.0;

fn main() {
    let mut board:Vec<Vec<char>> = vec![];
    let mut start_x = 0;
    let mut start_y = 0;
    let mut y:usize = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        let subline:Vec<char> = line.chars().collect();
        if subline.contains(&'S') { 
            start_x = subline.iter().position(|&c| c == 'S').unwrap();
            start_y = y;
        }
        board.push(subline);
        y += 1;
    }

    let mut steps:Vec<(usize, usize, i64)> = vec![(start_x, start_y, 0)];
    let mut cost = HashMap::new();
    let part_2_board = board.clone();

    loop 
    {
        if steps.len() == 0 { break; }
        let step = steps.pop().unwrap();
        if step.2 >= PART_1_STEPS { board[step.1][step.0] = 'O'; continue; };
        if cost.contains_key(&(step.0, step.1))
        {
            let step_cost:&mut Vec<i64> = cost.get_mut(&(step.0, step.1)).unwrap();
            if step_cost.contains(&step.2){ continue; }
            step_cost.push(step.2);
        } else {
            cost.insert((step.0, step.1), vec![step.2]);
        }

        if step.0 > 0 && board[step.1][step.0-1] != '#' {
            steps.push((step.0-1, step.1, step.2+1));
        }

        if step.0 < board[0].len()-1 && board[step.1][step.0+1] != '#' {
            steps.push((step.0+1, step.1, step.2+1));
        }

        if step.1 > 0 && board[step.1-1][step.0] != '#' {
            steps.push((step.0, step.1-1, step.2+1));
        }
        
        if step.1 < board.len()-1 && board[step.1+1][step.0] != '#' {
            steps.push((step.0, step.1+1, step.2+1));
        }
    }

    let mut tot = 0;
    for y in 0..board.len() { for x in 0..board[0].len() { if board[y][x] == 'O' { tot += 1; } } }
    println!("Part 1: {}", tot);

    let board_float = board.len() as f64;
    let test_x:Vec<f64> = vec![(board_float * 0.5)-0.5, (board_float * 1.5)-0.5, (board_float * 2.5)-0.5];
    let mut test_y:Vec<f64> = vec![];

    for x in test_x.clone()
    {
        let mut hash_board: HashMap<(i32, i32), i32> = HashMap::new();
        let mut cost = HashMap::new();
        let mut steps = vec![(start_x as i32, start_y as i32, 0)];
        let board = part_2_board.clone();

        let board_x = board[0].len() as i32;
        let board_y = board.len() as i32;
        let mut step_x;
        let mut step_y;

        loop 
        {
            if steps.len() == 0 { break; }
            let step = steps.pop().unwrap();

            if step.2 >= x as i64 {
                if !hash_board.contains_key(&(step.0, step.1)) { hash_board.insert((step.0, step.1), 1); }
                continue; 
            }

            if cost.contains_key(&(step.0, step.1))
            {
                let step_cost:&mut Vec<i64> = cost.get_mut(&(step.0, step.1)).unwrap();
                if step_cost.contains(&step.2){ continue; }
                step_cost.push(step.2);
            } else {
                cost.insert((step.0, step.1), vec![step.2]);
            }

            step_x = (step.0-1).rem_euclid(board_x) as usize;
            step_y = step.1.rem_euclid(board_y) as usize;
            if board[step_y][step_x] != '#' {
                steps.push((step.0-1, step.1, step.2+1));
            }

            step_x = (step.0+1).rem_euclid(board_x) as usize;
            if board[step_y][step_x] != '#' {
                steps.push((step.0+1, step.1, step.2+1));
            }

            step_x = step.0.rem_euclid(board_x) as usize;
            step_y = (step.1-1).rem_euclid(board_y) as usize;
            if board[step_y][step_x] != '#' {
                steps.push((step.0, step.1-1, step.2+1));
            }
            
            step_y = (step.1+1).rem_euclid(board_y) as usize;
            if board[step_y][step_x] != '#' {
                steps.push((step.0, step.1+1, step.2+1));
            }
        }
        test_y.push(hash_board.len() as f64);
    }

    let eqn = polyfit(&test_x, &test_y, 2).unwrap();

    let tot = eqn[2]*PART_2_STEPS*PART_2_STEPS + eqn[1]*PART_2_STEPS + eqn[0];

    println!("Part 2: {}", tot as i64);
}
use std::fs::read_to_string;
use priority_queue::DoublePriorityQueue;

fn main() {
    let mut board:Vec<Vec<i32>> = vec![];

    for line in read_to_string("input.txt").unwrap().lines()
    {
        board.push(line.chars().map(|x| x.to_digit(10).unwrap() as i32).collect());
    }

    let mut path = DoublePriorityQueue::new();

    let target = ((board[0].len()-1) as i32, (board.len()-1) as i32);

    path.push(((0, 0), 0, ('?', 0)), (board.len() + board[0].len()) as i32);

    let mut cost = vec![vec![vec![i32::MAX; 3]; board[0].len()]; board.len()];

    let mut step;
    let mut x:i32;
    let mut y:i32;
    let mut path_cost;
    let mut dir;
    let mut dir_count;

    loop {
        step = path.pop_min().unwrap();
        x = step.0.0.0;
        y = step.0.0.1;
        path_cost = step.0.1;
        dir = step.0.2.0;
        dir_count = step.0.2.1;
        
        if dir_count > 2
        {
            continue;
        }

        let y_us = y as usize;
        let x_us = x as usize;

        if cost[y_us][x_us][dir_count] < path_cost
        {
            continue;
        }
        else
        {
            cost[y_us][x_us][dir_count] = path_cost;
        }

        if x == target.0 && y == target.1
        {
            break;
        }

        let n_steps;
        if dir == 'N'
        {
            n_steps = dir_count + 1;
        }
        else 
        {
            n_steps = 0;
        }

        let s_steps;
        if dir == 'S'
        {
            s_steps = dir_count + 1;
        }
        else 
        {
            s_steps = 0;
        }

        let e_steps;
        if dir == 'E'
        {
            e_steps = dir_count + 1;
        }
        else 
        {
            e_steps = 0;
        }

        let w_steps;
        if dir == 'W'
        {
            w_steps = dir_count + 1;
        }
        else 
        {
            w_steps = 0;
        }

        if x > 0 && dir != 'E'
        {
            let new_cost = path_cost + board[y_us][x_us-1];
            let heuristic = ((x-1) - target.0).abs() + (y - target.1).abs();
            path.push_increase(((x-1, y), new_cost, ('W', w_steps)), new_cost + heuristic);
        }
        if x < (board[0].len()-1) as i32 && dir != 'W'
        {
            let new_cost = path_cost + board[y_us][x_us+1];
            let heuristic =  ((x+1) - target.0).abs() + (y - target.1).abs();
            path.push_increase(((x+1, y), new_cost, ('E', e_steps)), new_cost + heuristic);
        }
        if y > 0 && dir != 'S'
        {
            let new_cost = path_cost + board[y_us-1][x_us];
            let heuristic = (x - target.0).abs() + ((y-1) - target.1).abs();
            path.push_increase(((x, y-1), new_cost, ('N', n_steps)), new_cost + heuristic);
        }
        if y < (board.len()-1) as i32 && dir != 'N'
        {
            let new_cost = path_cost + board[y_us+1][x_us];
            let heuristic = (x - target.0).abs() + ((y+1) - target.1).abs();
            path.push_increase(((x, y+1), new_cost, ('S', s_steps)), new_cost + heuristic);
        }
    }
    println!("Part 1: {}",  path_cost);

    let mut path = DoublePriorityQueue::new();

    let target = ((board[0].len()-1) as i32, (board.len()-1) as i32);

    path.push(((0, 0), 0, ('?', 0)), (board.len() + board[0].len()) as i32);

    let mut cost = vec![vec![vec![vec![i32::MAX; 11]; 5]; board[0].len()]; board.len()];

    let mut step;
    let mut x:i32;
    let mut y:i32;
    let mut path_cost;
    let mut dir;
    let mut dir_count;

    loop {
        step = path.pop_min().unwrap();
        x = step.0.0.0;
        y = step.0.0.1;
        path_cost = step.0.1;
        dir = step.0.2.0;
        dir_count = step.0.2.1;
        
        if dir_count > 9
        {
            continue;
        }

        let y_us = y as usize;
        let x_us = x as usize;

        if x == target.0 && y == target.1
        {
            break;
        }

        if dir_count < 3
        {
            if (dir == 'W' || dir == '?') && x > 3
            {
                let new_cost = path_cost + board[y_us][x_us-1] + board[y_us][x_us-2] + board[y_us][x_us-3] + board[y_us][x_us-4];
                let heuristic = ((x-4) - target.0).abs() + (y - target.1).abs();
                path.push_increase(((x-4, y), new_cost, ('W', 3)), new_cost + heuristic);
            }
            if (dir == 'E' || dir == '?') && x < (board[0].len()-4) as i32
            {
                let new_cost = path_cost + board[y_us][x_us+1] + board[y_us][x_us+2] + board[y_us][x_us+3] + board[y_us][x_us+4];
                let heuristic =  ((x+4) - target.0).abs() + (y - target.1).abs();
                path.push_increase(((x+4, y), new_cost, ('E', 3)), new_cost + heuristic);
            }
            if (dir == 'N' || dir == '?') && y > 3
            {
                let new_cost = path_cost + board[y_us-1][x_us] + board[y_us-2][x_us] + board[y_us-3][x_us] + board[y_us-4][x_us];
                let heuristic = (x - target.0).abs() + ((y-4) - target.1).abs();
                path.push_increase(((x, y-4), new_cost, ('N', 3)), new_cost + heuristic);
            }
            if (dir == 'S' || dir == '?') && y < (board.len()-4) as i32
            {
                let new_cost = path_cost + board[y_us+1][x_us] + board[y_us+2][x_us] + board[y_us+3][x_us] + board[y_us+4][x_us];
                let heuristic: i32 = (x - target.0).abs() + ((y+4) - target.1).abs();
                path.push_increase(((x, y+4), new_cost, ('S', 3)), new_cost + heuristic);
            }
        }
        else
        {
            let dir_index;
            match dir
            {
                'N'=>dir_index = 0,
                'E'=>dir_index = 1,
                'S'=>dir_index = 2,
                'W'=>dir_index = 3,
                _=>dir_index = 4,
            }

            if cost[y_us][x_us][dir_index][dir_count] < path_cost
            {
                continue;
            }
            else
            {
                cost[y_us][x_us][dir_index][dir_count] = path_cost;
            }

            if dir == 'N'
            {
                path.push_increase(((x, y), path_cost, ('E', 0)), path_cost + (x - target.0).abs() + (y - target.1).abs());
                path.push_increase(((x, y), path_cost, ('W', 0)), path_cost + (x - target.0).abs() + (y - target.1).abs());

                if y > 0
                {
                    let new_cost = path_cost + board[y_us-1][x_us];
                    let heuristic = (x - target.0).abs() + ((y-1) - target.1).abs();
                    path.push_increase(((x, y-1), new_cost, ('N', dir_count+1)), new_cost + heuristic);
                }
            }
            if dir == 'S'
            {
                path.push_increase(((x, y), path_cost, ('E', 0)), path_cost + (x - target.0).abs() + (y - target.1).abs());
                path.push_increase(((x, y), path_cost, ('W', 0)), path_cost + (x - target.0).abs() + (y - target.1).abs());

                if y < (board.len()-1) as i32
                {
                    let new_cost = path_cost + board[y_us+1][x_us];
                    let heuristic = (x - target.0).abs() + ((y+1) - target.1).abs();
                    path.push_increase(((x, y+1), new_cost, ('S', dir_count+1)), new_cost + heuristic);
                }   
            }            
            if dir == 'E'
            {
                path.push_increase(((x, y), path_cost, ('N', 0)), path_cost + (x - target.0).abs() + (y - target.1).abs());
                path.push_increase(((x, y), path_cost, ('S', 0)), path_cost + (x - target.0).abs() + (y - target.1).abs());

                if x < (board[0].len()-1) as i32
                {
                    let new_cost = path_cost + board[y_us][x_us+1];
                    let heuristic =  ((x+1) - target.0).abs() + (y - target.1).abs();
                    path.push_increase(((x+1, y), new_cost, ('E', dir_count+1)), new_cost + heuristic);
                }
            }
            if dir == 'W'
            {
                path.push_increase(((x, y), path_cost, ('N', 0)), path_cost + (x - target.0).abs() + (y - target.1).abs());
                path.push_increase(((x, y), path_cost, ('S', 0)), path_cost + (x - target.0).abs() + (y - target.1).abs());

                if x > 0
                {
                    let new_cost = path_cost + board[y_us][x_us-1];
                    let heuristic =  ((x-1) - target.0).abs() + (y - target.1).abs();
                    path.push_increase(((x-1, y), new_cost, ('W', dir_count+1)), new_cost + heuristic);
                }
            }
        }
    }
    println!("Part 2: {}",  path_cost);
}

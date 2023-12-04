use std::{fs::read_to_string};

fn main() {
    {
        let mut board:Vec<Vec<i32>> = vec![];

        for line in read_to_string("input.txt").unwrap().lines()
        {
            let mut sub_board:Vec<i32> = vec![-1; 1];
            let mut val:i32 = 0;
            let mut digit_count = 0;
            for c in line.trim().chars()
            {
                if c.is_ascii_digit()
                {
                    val = val * 10 + c.to_digit(10).unwrap() as i32;
                    digit_count += 1;
                }
                else 
                {
                    while digit_count > 0
                    {
                        sub_board.push(val);
                        digit_count -= 1;
                    }
                    val = 0;
                    if c == '.'
                    {
                        sub_board.push(-1);
                    }
                    else 
                    {
                        sub_board.push(-10);
                    }
                    digit_count = 0;
                }
            }

            while digit_count > 0
            {
                sub_board.push(val);
                digit_count -= 1;
            }

            sub_board.push(-1);
            board.push(sub_board.clone());
        }

        let x_max = board[0].len();

        board.insert(0, vec![-1; x_max]);
        board.push(vec![-1; x_max]);

        let y_max = board.len();

        let mut not_part = false;
        let mut is_number = false;
        let mut num = 0;
        let mut tot = 0;

        for j in 1..y_max-1
        {
            for i in 1..x_max-1
            {
                if board[j][i] > 0
                {
                    if is_number == false
                    {
                        not_part = true;
                        num = board[j][i];
                    }
                    is_number = true;

                    if board[j-1][i-1] == -10 || 
                    board[j-1][i+0] == -10 ||
                    board[j-1][i+1] == -10 ||
                    board[j+0][i-1] == -10 ||
                    board[j+0][i+1] == -10 ||
                    board[j+1][i-1] == -10 ||
                    board[j+1][i+0] == -10 ||
                    board[j+1][i+1] == -10 
                    {
                        not_part = false;
                    }

                }
                else
                {
                    if is_number == true && not_part == false
                    {
                        tot += num;
                    }
                    is_number = false;
                }
            }
        }

        println!("Part 1: {}", tot);
    }

    {
        let mut board:Vec<Vec<i32>> = vec![];

        for line in read_to_string("input.txt").unwrap().lines()
        {
            let mut sub_board:Vec<i32> = vec![-1; 1];
            let mut val:i32 = 0;
            let mut digit_count = 0;
            for c in line.trim().chars()
            {
                if c.is_ascii_digit()
                {
                    val = val * 10 + c.to_digit(10).unwrap() as i32;
                    digit_count += 1;
                }
                else 
                {
                    while digit_count > 0
                    {
                        sub_board.push(val);
                        digit_count -= 1;
                    }
                    val = 0;
                    if c == '*'
                    {
                        sub_board.push(-10);
                    }
                    else
                    {
                        sub_board.push(-1);
                    }
                    
                    digit_count = 0;
                }
            }

            while digit_count > 0
            {
                sub_board.push(val);
                digit_count -= 1;
            }

            sub_board.push(-1);
            board.push(sub_board.clone());
        }

        let x_max = board[0].len();

        board.insert(0, vec![-1; x_max]);
        board.push(vec![-1; x_max]);

        let y_max = board.len();

        let mut tot:i64 = 0;

        for j in 1..y_max-1
        {
            for i in 1..x_max-1
            {
                let mut adjacent_count = 0;
                let mut gear_ratio:i64 = 1;
                let mut part_nums:Vec<i32> = vec![];
                if board[j][i] == -10
                {
                    if board[j-1][i-1] > 0 && !part_nums.contains(&board[j-1][i-1])
                    {
                        adjacent_count += 1;
                        gear_ratio *= board[j-1][i-1] as i64;
                        part_nums.push(board[j-1][i-1]);
                    }

                    if board[j-1][i+0] > 0 && !part_nums.contains(&board[j-1][i+0])
                    {
                        adjacent_count += 1;
                        gear_ratio *= board[j-1][i+0] as i64;
                        part_nums.push(board[j-1][i+0]);
                    }

                    if board[j-1][i+1] > 0 && !part_nums.contains(&board[j-1][i+1])
                    {
                        adjacent_count += 1;
                        gear_ratio *= board[j-1][i+1] as i64;
                        part_nums.push(board[j-1][i+1]);
                    }

                    if board[j+0][i-1] > 0 && !part_nums.contains(&board[j+0][i-1])
                    {
                        adjacent_count += 1;
                        gear_ratio *= board[j+0][i-1] as i64;
                        part_nums.push(board[j+0][i-1]);
                    }

                    if board[j+0][i+1] > 0 && !part_nums.contains(&board[j+0][i+1])
                    {
                        adjacent_count += 1;
                        gear_ratio *= board[j+0][i+1] as i64;
                        part_nums.push(board[j+0][i+1]);
                    }

                    if board[j+1][i-1] > 0 && !part_nums.contains(&board[j+1][i-1])
                    {
                        adjacent_count += 1;
                        gear_ratio *= board[j+1][i-1] as i64;
                        part_nums.push(board[j+1][i-1]);
                    }

                    if board[j+1][i+0] > 0 && !part_nums.contains(&board[j+1][i+0])
                    {
                        adjacent_count += 1;
                        gear_ratio *= board[j+1][i+0] as i64;
                        part_nums.push(board[j+1][i+0]);
                    }

                    if board[j+1][i+1] > 0 && !part_nums.contains(&board[j+1][i+1])
                    {
                        adjacent_count += 1;
                        gear_ratio *= board[j+1][i+1] as i64;
                        part_nums.push(board[j+1][i+1]);
                    }
                    if adjacent_count == 2
                    {
                        tot += gear_ratio;
                    }
                }
            }
        }
        println!("Part 2: {}", tot);
    }
}
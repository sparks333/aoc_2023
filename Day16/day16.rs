use std::{fs::read_to_string, cmp};

fn eval_board(init:(usize, usize, char), board:&mut Vec<Vec<(char, (bool, bool, bool, bool))>>) -> u32
{
    let mut beams:Vec<(usize, usize, char)> = vec![init; 1];

    loop 
    {
        let mut new_beams:Vec<(usize, usize, char)> = vec![];

        for i in beams
        {
            if i.2 == 'N'
            {
                if board[i.1][i.0].1.0 == true
                {
                    continue;
                }
                else
                {
                    board[i.1][i.0].1.0 = true
                }
            }
            else if i.2 == 'S'
            {
                if board[i.1][i.0].1.1 == true
                {
                    continue;
                }
                else
                {
                    board[i.1][i.0].1.1 = true
                }
            }
            else if i.2 == 'E'
            {
                if board[i.1][i.0].1.2 == true
                {
                    continue;
                }
                else
                {
                    board[i.1][i.0].1.2 = true
                }
            }
            else if i.2 == 'W' 
            {
                if board[i.1][i.0].1.3 == true
                {
                    continue;
                }
                else
                {
                    board[i.1][i.0].1.3 = true
                }
            }        

            if i.2 == 'E'
            {
                if board[i.1][i.0].0 == '\\'
                {
                    if i.1 != board.len()-1
                    {
                        new_beams.push((i.0, i.1+1, 'S'));
                    }
                }
                else if board[i.1][i.0].0 == '/'
                {
                    if i.1 != 0
                    {
                        new_beams.push((i.0, i.1-1, 'N'));
                    }
                }
                else if board[i.1][i.0].0 == '|'
                {
                    if i.1 != board.len()-1
                    {
                        new_beams.push((i.0, i.1+1, 'S'));
                    }
                    if i.1 != 0
                    {
                        new_beams.push((i.0, i.1-1, 'N'));
                    }
                }
                else
                {
                    if board[i.1][i.0].0 == '.'
                    {
                        board[i.1][i.0].0 = '#';
                    }
                    if i.0 != board[0].len()-1
                    {
                        new_beams.push((i.0+1, i.1, i.2));
                    }
                }
            }
            else if i.2 == 'W'
            {
                if board[i.1][i.0].0 == '\\'
                {
                    if i.1 != 0
                    {
                        new_beams.push((i.0, i.1-1, 'N'));
                    }
                }
                else if board[i.1][i.0].0 == '/'
                {
                    if i.1 != board.len()-1
                    {
                        new_beams.push((i.0, i.1+1, 'S'));
                    }
                }
                else if board[i.1][i.0].0 == '|'
                {
                    if i.1 != board.len()-1
                    {
                        new_beams.push((i.0, i.1+1, 'S'));
                    }
                    if i.1 != 0
                    {
                        new_beams.push((i.0, i.1-1, 'N')); 
                    }
                }
                else
                {
                    if board[i.1][i.0].0 == '.'
                    {
                        board[i.1][i.0].0 = '#';
                    }
                    if i.0 != 0
                    {
                        new_beams.push((i.0-1, i.1, i.2));
                    }
                }
            }
            else if i.2 == 'N'
            {
                if board[i.1][i.0].0 == '\\'
                {
                    if i.0 != 0
                    {
                        new_beams.push((i.0-1, i.1, 'W'));
                    }
                }
                else if board[i.1][i.0].0 == '/'
                {
                    if i.0 != board[0].len()-1
                    {
                        new_beams.push((i.0+1, i.1, 'E'));
                    }
                }
                else if board[i.1][i.0].0 == '-'
                {
                    if i.0 != board[0].len()-1
                    {
                        new_beams.push((i.0+1, i.1, 'E'));
                    }
                    if i.0 != 0
                    {
                        new_beams.push((i.0-1, i.1, 'W'));
                    }
                }
                else
                {
                    if board[i.1][i.0].0 == '.'
                    {
                        board[i.1][i.0].0 = '#';
                    }
                    if i.1 != 0
                    {
                        new_beams.push((i.0, i.1-1, i.2));
                    }
                }
            }
            else
            {
                if board[i.1][i.0].0 == '\\'
                {
                    if i.0 != board.len()-1
                    {
                        new_beams.push((i.0+1, i.1, 'E'));
                    }
                }
                else if board[i.1][i.0].0 == '/'
                {
                    if i.0 != 0
                    {
                        new_beams.push((i.0-1, i.1, 'W'));
                    }
                }
                else if board[i.1][i.0].0 == '-'
                {
                    if i.0 != board[0].len()-1
                    {
                        new_beams.push((i.0+1, i.1, 'E'));
                    }
                    if i.0 != 0
                    {
                        new_beams.push((i.0-1, i.1, 'W'));
                    }
                }
                else
                {
                    if board[i.1][i.0].0 == '.'
                    {
                        board[i.1][i.0].0 = '#';
                    }
                    if i.1 != board.len()-1
                    {
                        new_beams.push((i.0, i.1+1, i.2));
                    }
                }
            }
        }

        beams = new_beams.clone();

        if beams.len() == 0
        {
            break;
        }
    }

    let mut tot = 0;

    for y in 0..board.len()
    {
        for x in 0..board[0].len()
        {
            if board[y][x].1.0 || board[y][x].1.1 || board[y][x].1.2 || board[y][x].1.3
            {
                tot += 1;
            }
        }
    }
    return tot;
}

fn main() {

    let mut board:Vec<Vec<(char, (bool, bool, bool, bool))>> = vec![];

    for line in read_to_string("input.txt").unwrap().lines()
    {
        let mut sub_board:Vec<(char, (bool, bool, bool, bool))> = vec![];
        for c in line.chars()
        {
            sub_board.push((c, (false, false, false, false)));
        }
        board.push(sub_board);
    }

    let mut pt_1_board = board.clone();

    println!("Part 1: {}", eval_board((0, 0, 'E'), &mut pt_1_board));

    let mut max_tot = 0;

    let mut pt_2_board;

    for y in 0..board.len()
    {
        pt_2_board = board.clone();
        max_tot = cmp::max(max_tot, eval_board((0, y, 'E'), &mut pt_2_board));
        pt_2_board = board.clone();
        max_tot = cmp::max(max_tot, eval_board((board[0].len()-1, y, 'W'), &mut pt_2_board));
    }

    for x in 0..board[0].len()
    {
        pt_2_board = board.clone();
        max_tot = cmp::max(max_tot, eval_board((x, 0, 'S'), &mut pt_2_board));
        pt_2_board = board.clone();
        max_tot = cmp::max(max_tot, eval_board((x, board.len()-1, 'N'), &mut pt_2_board));
    }

    println!("Part 2: {}", max_tot);

}

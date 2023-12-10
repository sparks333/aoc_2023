use std::fs::read_to_string;

fn allowable_dir(board:&Vec<Vec<char>>, pos:&(usize, usize))->Vec<(usize, usize)>
{
    let mut ret:Vec<(usize, usize)> = vec![];
    let x = pos.0;
    let y = pos.1;

    let current = board[y][x];

    if (current == '|' || current == 'L' || current == 'J' || current == 'S') &&
       (board[y-1][x] == '|' || board[y-1][x] == '7' || board[y-1][x] == 'F' || board[y-1][x] == 'S')
    {
        ret.push((x, y-1));
    }
    if (current == '|' || current == 'F' || current == '7' || current == 'S') &&
       (board[y+1][x] == '|' || board[y+1][x] == 'J' || board[y+1][x] == 'L' || board[y+1][x] == 'S')
    {
        ret.push((x, y+1));
    }
    if (current == '-' || current == '7' || current == 'J' || current == 'S') &&
       (board[y][x-1] == '-' || board[y][x-1] == 'F' || board[y][x-1] == 'L' || board[y][x-1] == 'S')
    {
        ret.push((x-1, y));
    }
    if (current == '-' || current == 'F' || current == 'L' || current == 'S') &&
       (board[y][x+1] == '-' || board[y][x+1] == '7' || board[y][x+1] == 'J' || board[y][x+1] == 'S')
    {
        ret.push((x+1, y));
    }
    return ret;
}

fn main() {

    let mut board:Vec<Vec<char>> = vec![];

    let mut start_x = 0;
    let mut start_y = 0;

    let mut y:usize = 1;

    for line in read_to_string("input.txt").unwrap().lines()
    {
        let mut sub_board:Vec<char> = line.chars().collect();
        sub_board.insert(0, '.');
        sub_board.push('.');
        let ret = sub_board.iter().enumerate().find(|x| *x.1 == 'S');
        
        if ret.is_some()
        {
            start_x = ret.unwrap().0;
            start_y = y;
        }
        board.push(sub_board);

        y += 1;
    }

    board.insert(0, vec!['.'; board[0].len()]);
    board.push(vec!['.'; board[0].len()]);

    let mut new_board = vec![vec!['O'; board[0].len()]; board.len()];
    new_board[start_y][start_x] = 'S';

    let start_dirs = allowable_dir(&board, &(start_x, start_y));

    let mut fw_prev = (start_x, start_y);
    let mut fw = start_dirs[0];

    let mut rev_prev = (start_x, start_y);
    let mut rev = start_dirs[1];

    let mut steps = 1;

    loop
    {
        new_board[fw.1][fw.0] = board[fw.1][fw.0];
        new_board[rev.1][rev.0] = board[rev.1][rev.0];

        let mut fw_next = allowable_dir(&board, &fw);
        fw_next.retain(|&x| x != fw_prev);
        fw_prev = fw;
        fw = fw_next[0];
        
        let mut rev_next = allowable_dir(&board, &rev);
        rev_next.retain(|&x| x != rev_prev);
        rev_prev = rev;
        rev = rev_next[0];

        steps += 1;   

        if fw == rev || rev == fw_prev || fw == rev_prev
        {
            new_board[fw.1][fw.0] = board[fw.1][fw.0];
            new_board[rev.1][rev.0] = board[rev.1][rev.0];
            break;
        }
    }

    println!("Part 1: {}", steps);

    let mut fw_prev = (start_x, start_y);
    let mut fw = start_dirs[0];

    loop
    {
        if fw.0 == fw_prev.0 && fw.1 == fw_prev.1-1 // Up
        {
            new_board[fw_prev.1][fw_prev.0] = '^';
        }
        else if fw.0 == fw_prev.0 && fw.1 == fw_prev.1+1 // Down
        {
            new_board[fw_prev.1][fw_prev.0] = 'v';
        }      
        else if fw.0 == fw_prev.0-1 && fw.1 == fw_prev.1 // Left
        {
            new_board[fw_prev.1][fw_prev.0] = '<';
        }
        else if fw.0 == fw_prev.0+1 && fw.1 == fw_prev.1 // Right
        {
            new_board[fw_prev.1][fw_prev.0] = '>';
        }
        else {
            println!("Something went wrong");
        }

        let mut fw_next: Vec<(usize, usize)> = allowable_dir(&board, &fw);
        fw_next.retain(|&x| x != fw_prev);
        fw_prev = fw;
        fw = fw_next[0];

        if fw_prev.0 == start_x && fw_prev.1 == start_y
        {
            break;
        }
    }

    let mut dirboard:Vec<Vec<char>> = vec![];
    for y in 1..new_board.len()
    {
        let mut sub_board:Vec<char> = vec![];
        for x in 0..new_board[y].len()
        {
            if new_board[y-1][x] == 'v'
            {
                sub_board.push('v');
            }
            else if new_board[y][x] == '^'
            {
                sub_board.push('^');
            }
            else if new_board[y][x] == 'O'
            {
                sub_board.push('O')
            }
            else 
            {
                sub_board.push('.')
            }
        }
        dirboard.push(sub_board);
    }

    for y in 0..dirboard.len()
    {
        let mut state = 0;
        for x in 0..dirboard[y].len()
        {
            match state
            {
                0=>
                {
                    if dirboard[y][x] == '^'
                    {
                        state = 1;
                    }
                    if dirboard[y][x] == 'v'
                    {
                        state = -1;
                    }
                },
                1=>
                {
                    if dirboard[y][x] == 'v'
                    {
                        state = 0;
                    }
                    else if dirboard[y][x] == 'O'
                    {
                        dirboard[y][x] = 'I';
                    }
                },                
                -1=>
                {
                    if dirboard[y][x] == '^'
                    {
                        state = 0;
                    }
                    else if dirboard[y][x] == 'O'
                    {
                        dirboard[y][x] = 'I';
                    }
                },
                _=>println!("Error"),
            }
        }
    }

    let mut count = 0;

    for i in dirboard
    {
        for j in i
        {
            if j == 'I'
            {
                count += 1;
            }
        }
    }

    println!("Part 2: {}", count);

}


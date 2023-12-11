use std::fs::read_to_string;

fn main() {

    let mut board:Vec<Vec<char>> = vec![];

    for line in read_to_string("input.txt").unwrap().lines()
    {
        board.push(line.chars().collect());
    }

    let mut row_add:Vec<bool> = vec![true; board.len()];
    let mut col_add:Vec<bool> = vec![true; board[0].len()];

    for y in 0..board.len()
    {
        for x in 0..board[0].len()
        {
            if board[y][x] != '.'
            {
                row_add[y] &= false;
                col_add[x] &= false;
            }
        }
    }

    let mut galaxy:Vec<(i32, i32)> = vec![];
    let mut x_add;
    let mut y_add = 0;
    
    for y in 0..board.len()
    {
        x_add = 0;
        for x in 0..board[0].len()
        {  
            if board[y][x] != '.'
            {
                galaxy.push((x as i32 + x_add, y as i32 + y_add));
            }

            if col_add[x] == true
            {
                x_add += 1;
            }
        }
        if row_add[y] == true
        {
            y_add += 1;
        }
    }

    let mut dist:Vec<i32> = vec![];

    for y in 0..galaxy.len()
    {
        for x in y+1..galaxy.len()
        {
            dist.push((galaxy[y].0 - galaxy[x].0).abs() + (galaxy[y].1 - galaxy[x].1).abs());
        }
    }

    let mut total = 0;
    for i in dist
    {
        total += i;
    }


    println!("Part 1: {}", total);

    let mut galaxy:Vec<(i32, i32)> = vec![];
    let mut x_add;
    let mut y_add = 0;
    
    for y in 0..board.len()
    {
        x_add = 0;
        for x in 0..board[0].len()
        {  
            if board[y][x] != '.'
            {
                galaxy.push((x as i32 + x_add, y as i32 + y_add));
            }

            if col_add[x] == true
            {
                x_add += 1000000 - 1;
            }
        }
        if row_add[y] == true
        {
            y_add += 1000000 - 1;
        }
    }

    let mut dist:Vec<i32> = vec![];

    for y in 0..galaxy.len()
    {
        for x in y+1..galaxy.len()
        {
            dist.push((galaxy[y].0 - galaxy[x].0).abs() + (galaxy[y].1 - galaxy[x].1).abs());
        }
    }

    let mut total:i64 = 0;
    for i in dist
    {
        total += i as i64;
    }

    println!("Part 2: {}", total);

}

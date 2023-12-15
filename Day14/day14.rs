use std::{fs::read_to_string, collections::HashMap};

const MAX_RUNS:i64 = 1000000000;

fn generate_cols(rows:&Vec<Vec<(usize, usize, bool)>>) -> Vec<Vec<(usize, usize, bool)>>
{
    let mut cols:Vec<Vec<(usize, usize, bool)>> = vec![vec![]; rows.len()];

    for i in 0..rows.len()
    {
        for j in &rows[i]
        {
            cols[j.0].push((j.0, j.1, j.2));
        }
    }
    return cols;
}

fn generate_rows(cols:&Vec<Vec<(usize, usize, bool)>>) -> Vec<Vec<(usize, usize, bool)>>
{
    let mut rows:Vec<Vec<(usize, usize, bool)>> = vec![vec![]; cols.len()];

    for i in 0..cols.len()
    {
        for j in &cols[i]
        {
            rows[j.1].push((j.0, j.1, j.2));
        }
    }
    return rows;
}

fn tilt_north(cols:&mut Vec<Vec<(usize, usize, bool)>>)
{
    for col in cols
    {
        let mut space:i32 = 0;
        for y in 0..col.len()
        {
            if col[y].2 == false
            {
                space = col[y].1 as i32 + 1
            }
            else
            {
                col[y].1 = space as usize;
                space += 1;
            }
        }
    }
}

fn tilt_south(cols:&mut Vec<Vec<(usize, usize, bool)>>)
{
    let max_space = (cols.len()-1) as i32;
    for col in cols
    {
        let mut space:i32 = max_space;
        for y in (0..col.len()).rev()
        {
            if col[y].2 == false
            {
                space = col[y].1 as i32 - 1;
            }
            else
            {
                col[y].1 = space as usize;
                space -= 1;
            }
        }
    }
}

fn tilt_east(rows:&mut Vec<Vec<(usize, usize, bool)>>)
{
    let max_space = (rows.len()-1) as i32;
    for row in rows
    {
        let mut space:i32 = max_space;
        for x in (0..row.len()).rev()
        {
            if row[x].2 == false
            {
                space = row[x].0 as i32 - 1;
            }
            else
            {
                row[x].0 = space as usize;
                space -= 1;
            }
        }
    }
}

fn tilt_west(rows:&mut Vec<Vec<(usize, usize, bool)>>)
{
    for row in rows
    {
        let mut space:i32 = 0;
        for x in 0..row.len()
        {
            if row[x].2 == false
            {
                space = row[x].0 as i32 + 1;
            }
            else
            {
                row[x].0 = space as usize;
                space += 1;
            }
        }
    }
}


fn main() {

    let mut rows:Vec<Vec<(usize, usize, bool)>> = vec![];

    let mut x = 0;
    let mut y = 0;

    for line in read_to_string("input.txt").unwrap().lines()
    {
        let mut row:Vec<(usize, usize, bool)> = vec![];
        for c in line.chars()
        {
            if c == 'O'
            {
                row.push((x, y, true));
            }
            if c == '#'
            {
                row.push((x, y, false));
            }
            x += 1;
        }
        rows.push(row);
        y += 1;
        x = 0;
    }

    let mut cols = generate_cols(&rows);

    let part_2_cols = cols.clone();

    tilt_north(&mut cols);

    let mut tot = 0;

    for col in cols
    {
        for j in col
        {
            if j.2 == true
            {
                tot += y-j.1;
            }
        }
    }

    println!("Part 1: {}", tot);

    cols = part_2_cols;

    let mut map = HashMap::new();

    let mut count = 0;
    let mut loop_found = false;

    loop
    {
        tilt_north(&mut cols);
        rows = generate_rows(&cols);
        tilt_west(&mut rows);
        cols = generate_cols(&rows);
        tilt_south(&mut cols);
        rows = generate_rows(&cols);
        tilt_east(&mut rows);
        cols = generate_cols(&rows);

        if map.contains_key(&cols) && !loop_found
        {
            let loop_len = (count - map.get(&cols).unwrap())+1;
            let remaining_count = (MAX_RUNS - count) % loop_len;
            count = (MAX_RUNS - remaining_count) + 1;
            loop_found = true;
        }
        else
        {
            count += 1;
            if count == MAX_RUNS
            {
                break;
            }
            map.insert(cols.clone(), count);
        }
    }

    tot = 0;

    for col in cols
    {
        for j in col
        {
            if j.2 == true
            {
                tot += y-j.1;
            }
        }
    }

    println!("Part 2: {}", tot);

}

use std::fs::read_to_string;

fn next_num(input:&Vec<i32>)->i32
{
    let mut diff:Vec<i32> = vec![];
    let mut done:bool = true;
    for i in 1..input.len()
    {
        let d = input[i] - input[i-1];
        if d != 0
        {
            done = false;
        }
        diff.push(input[i]-input[i-1]);
    }
    
    if done
    {
        return input.last().unwrap() + diff.last().unwrap();
    }
    else
    {
        return input.last().unwrap() + next_num(&diff);
    }
}

fn main()
{
    let mut last:Vec<i32> = vec![];
    let mut first:Vec<i32> = vec![];

    for line in read_to_string("input.txt").unwrap().lines()
    {
        let mut split_line:Vec<i32> = line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
        last.push(next_num(&split_line));
        split_line.reverse();
        first.push(next_num(&split_line));
    }

    let mut total:i32 = 0;

    for i in last
    {
        total += i;
    }

    println!("Part 1: {}", total);

    total = 0;

    for i in first
    {
        total += i;
    }

    println!("Part 2: {}", total);


}
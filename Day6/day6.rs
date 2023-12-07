use std::{fs::read_to_string};

fn main()
{
    let mut state = 0;

    let mut times:Vec<i32> = vec![];
    let mut times_string:Vec<String> = vec![];
    let mut distances:Vec<i32> = vec![];
    let mut distances_string:Vec<String> = vec![];

    for line in read_to_string("input.txt").unwrap().lines()
    {
        if state == 0
        {
            times_string = line.split(|x| (x == ' ')).filter(|s| !s.is_empty()).map(str::to_string).collect();
            times = times_string[1..].iter().map(|x| x.parse::<i32>().unwrap()).collect();
            state += 1;
        }
        else
        {
            distances_string = line.split(|x| (x == ' ')).filter(|s| !s.is_empty()).map(str::to_string).collect();
            distances = distances_string[1..].iter().map(|x| x.parse::<i32>().unwrap()).collect();
        }
    }

    let mut total = 1;

    for i in 0..times.len()
    {
        let b = -times[i] as f32;
        let c = distances[i] as f32;

        let mut first_win = (-b - ((b*b) - (4.0*c)).sqrt())/(2.0);
        let mut last_win =  (-b + ((b*b) - (4.0*c)).sqrt())/(2.0);

        if first_win.fract() == 0.0
        {
            first_win += 1.0;
        }

        if last_win.fract() == 0.0
        {
            last_win -= 1.0;
        }

        let range = (last_win.floor() - first_win.ceil()) as i32 + 1;

        total *= range;

    }

    println!("Part 1: {}", total);

    let time:i64 = times_string[1..].join("").parse::<i64>().unwrap();
    let distance:i64 = distances_string[1..].join("").parse::<i64>().unwrap();

    let b = -time as f64;
    let c = distance as f64;

    let mut first_win = (-b - ((b*b) - (4.0*c)).sqrt())/(2.0);
    let mut last_win =  (-b + ((b*b) - (4.0*c)).sqrt())/(2.0);

    if first_win.fract() == 0.0
    {
        first_win += 1.0;
    }

    if last_win.fract() == 0.0
    {
        last_win -= 1.0;
    }

    println!("Part 2: {}", (last_win.floor() - first_win.ceil()) as i64 + 1);

}

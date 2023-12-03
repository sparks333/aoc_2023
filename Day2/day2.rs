use std::{fs::read_to_string, f32::consts::E, cmp};

fn main() {

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut total = 0;

    for line in read_to_string("input.txt").unwrap().lines()
    {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let split_line:Vec<String> = line.split(|x| (x == ' ') || (x == ':') || (x == ',') || (x == ';')).filter(|s| !s.is_empty()).map(str::to_string).collect();
        let game_id = split_line.get(1).unwrap().parse::<i32>().unwrap();

        let mut value = 0;

        for a in &split_line[2..]
        {
            if a == "red"
            {
                red = cmp::max(red, value);
            }
            else if a == "green"
            {
                green = cmp::max(green, value);
            }
            else if a == "blue"
            {
                blue = cmp::max(blue, value);
            }
            else
            {
                value = a.parse::<i32>().unwrap();
            }
        }
        if red <= max_red && blue <= max_blue && green <= max_green
        {
            total += game_id;
        }
    }
    println!("Part 1: {}", total);

    total = 0;

    for line in read_to_string("input.txt").unwrap().lines()
    {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let split_line:Vec<String> = line.split(|x| (x == ' ') || (x == ':') || (x == ',') || (x == ';')).filter(|s| !s.is_empty()).map(str::to_string).collect();
        let game_id = split_line.get(1).unwrap().parse::<i32>().unwrap();

        let mut value = 0;

        for a in &split_line[2..]
        {
            if a == "red"
            {
                red = cmp::max(red, value);
            }
            else if a == "green"
            {
                green = cmp::max(green, value);
            }
            else if a == "blue"
            {
                blue = cmp::max(blue, value);
            }
            else
            {
                value = a.parse::<i32>().unwrap();
            }
        }
        total += red * green * blue;
    }
    println!("Part 2: {}", total);
}

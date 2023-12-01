use std::{fs::read_to_string, f32::consts::E};

fn main() {
    // Part 1
    let mut total:i64 = 0;
    for line in read_to_string("input.txt").unwrap().lines()
    {
        let mut i:Result<i32, _>;
        let mut numbers:Vec<i32> = vec![];
        for c in line.chars()
        {
            i = c.to_string().parse::<i32>();
            if i.is_ok() {
                numbers.push(i.unwrap());
            }
        }
        let val = numbers.first().unwrap() * 10 + numbers.last().unwrap();

        total += val as i64;
    }
    println!("Part 1: {}", total);

    
    // Part 2
    total = 0;
    for line in read_to_string("input.txt").unwrap().lines()
    {
        let str1 = line.replace("one", "o1e");
        let str2 = str1.replace("two", "t2o");
        let str3 = str2.replace("three", "t3e");
        let str4 = str3.replace("four", "f4r");
        let str5 = str4.replace("five", "f5e");
        let str6 = str5.replace("six", "s6x");
        let str7 = str6.replace("seven", "s7n");
        let str8 = str7.replace("eight", "e8t");
        let str9 = str8.replace("nine", "n9e");

        let mut i:Result<i32, _>;
        let mut numbers:Vec<i32> = vec![];
        for c in str9.chars()
        {
            i = c.to_string().parse::<i32>();
            if i.is_ok() {
                numbers.push(i.unwrap());
            }
        }
        let val = numbers.first().unwrap() * 10 + numbers.last().unwrap();

        total += val as i64;
    }
    println!("Part 2: {}", total);
}
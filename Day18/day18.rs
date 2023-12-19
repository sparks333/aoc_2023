use std::fs::read_to_string;

fn main() {

    let mut inst:Vec<(char, i64, String, (i64, i64))> = vec![];

    let mut x = 0;
    let mut y = 0;

    let mut integers = 0;

    for line in read_to_string("input.txt").unwrap().lines()
    {
        let split_line:Vec<String> = line.split(" ").map(str::to_string).collect();
        let dir:char = split_line[0].chars().collect::<Vec<_>>()[0];
        let dist = split_line[1].parse::<i64>().unwrap();
        let color = split_line[2][1..split_line[2].len()-1].to_string();

        inst.push((dir, dist, color, (x, y)));

        match dir
        {
            'U'=> y -= dist,
            'D'=> y += dist,
            'L'=> x -= dist,
            'R'=> x += dist,
            _=> println!("Parse Error!")
        }
        integers += dist;
    }

    let mut sum1 = 0;
    let mut sum2 = 0;


    let mut xa;
    let mut ya;
    let mut xb;
    let mut yb;

    for i in 0..inst.len()-1
    {

        xa = inst[i].3.0;
        ya = inst[i].3.1;
        xb = inst[i+1].3.0;
        yb = inst[i+1].3.1;

        sum1 += xa*yb;
        sum2 += ya*xb;
    }

    let area = (sum1-sum2).abs() / 2;

    println!("Part 1: {}", area + integers/2 +1);

    let mut x = 0;
    let mut y = 0;

    let mut integers = 0;

    for i in 0..inst.len()
    {
        let color = &inst[i].2;
        let dist = i64::from_str_radix(&color[1..6], 16).unwrap();
        let mut dir = '?';
        match color.chars().collect::<Vec<_>>()[6]
        {
            '0'=>dir='R',
            '1'=>dir='D',
            '2'=>dir='L',
            '3'=>dir='U',
            _=>println!("Parse Error"),
        }

        inst[i].3.0 = x;
        inst[i].3.1 = y;

        match dir
        {
            'U'=> y -= dist,
            'D'=> y += dist,
            'L'=> x -= dist,
            'R'=> x += dist,
            _=> println!("Parse Error!")
        }
        integers += dist;
    }

    let mut sum1 = 0;
    let mut sum2 = 0;


    let mut xa;
    let mut ya;
    let mut xb;
    let mut yb;

    for i in 0..inst.len()-1
    {

        xa = inst[i].3.0;
        ya = inst[i].3.1;
        xb = inst[i+1].3.0;
        yb = inst[i+1].3.1;

        sum1 += xa*yb;
        sum2 += ya*xb;
    }

    let area = (sum1-sum2).abs() / 2;

    println!("Part 2: {}", area + integers/2 +1);

}

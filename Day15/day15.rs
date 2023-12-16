use std::fs::read_to_string;

fn hash(seq:String)->u32
{
    let mut curr_value = 0;
    for i in seq.chars()
    {
        let ascii_value = i as u32;
        curr_value += ascii_value;
        curr_value *= 17;
        curr_value %= 256;
    }
    return curr_value;
}

fn main() {
    let line:String = read_to_string("input.txt").unwrap().lines().map(str::to_string).collect();
    let seq:Vec<String> = line.replace("\n", "").split(",").map(str::to_string).collect();

    let mut tot = 0;

    let seq1 = seq.clone();

    for i in seq1
    {
        tot += hash(i);
    }

    println!("Part 1: {}", tot);

    tot = 0;

    let mut board:Vec<Vec<(String, u32)>> = vec![vec![]; 256];
    let seq2 = seq.clone();

    for i in seq2
    {
        let command;
        if i.find('=').is_some()
        {
            command = "=".to_string();
        }
        else
        {
            command = "-".to_string();
        }
        
        let label_focal_len:Vec<String> = i.split(['=','-']).map(str::to_string).collect();
        let label = label_focal_len[0].clone();
        let box_num = hash(label.clone()) as usize;

        if command == "="
        {
            let mut inserted = false;
            let focal_len = label_focal_len[1].parse::<u32>().unwrap();
            for j in 0..board[box_num].len()
            {
                if board[box_num][j].0 == label
                {
                    board[box_num][j].1 = focal_len;
                    inserted = true;
                    break;
                }
            }
            if inserted == false
            {
                board[box_num].push((label.to_string(), focal_len));
            }
        }
        else if command == "-"
        {
            for j in 0..board[box_num].len()
            {
                if board[box_num][j].0 == label
                {
                    board[box_num].remove(j);
                    break;
                }
            }
        }
        else
        {
            println!("Parse error");
        }
    }

    for b in 0..board.len()
    {
        for s in 0..board[b].len()
        {
            let focus_power = (b as u32 + 1) * (s as u32 + 1)*board[b][s].1;
            tot += focus_power
        }
    }
    println!("Part 2: {}", tot);
}

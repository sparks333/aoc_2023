use std::fs::read_to_string;
use std::collections::HashMap;
use std::cmp;

fn main() {
    let mut board = HashMap::new();

    for line in read_to_string("input.txt").unwrap().lines() {
        let split_line:Vec<String> = line.split([':', ' ']).filter(|x| !x.is_empty()).map(str::to_string).collect();
        board.insert(split_line[0].clone(), split_line[1..].to_vec());

        for i in 1..split_line.len()
        {
            if board.contains_key(&split_line[i]) {
                board.get_mut(&split_line[i]).unwrap().push(split_line[0].clone());
            } else {
                board.insert(split_line[i].clone(), vec![split_line[0].clone()]);
            }
        }
    }


    let mut node1 = "".to_string();
    let mut node2 = "".to_string();
    let mut tot;

    loop{
        let mut new_board: HashMap<String, Vec<String>> = board.clone();
        let mut min_count = 3;

        loop{
            let mut nnb = HashMap::new();
            let mut nni = "".to_string();
            let mut new_min_count = usize::MAX;
            for j in &new_board{
                if j.0.len() == min_count {
                    node1 = j.0.clone();
                    let node2_index = (rand::random::<u32>() % (j.1.len() as u32)) as usize;
                    node2 = j.1[node2_index].clone();
                    nni = node1.clone();
                    nni.push_str(&node2);
                    let mut nnv:Vec<String> = j.1.clone();
                    let mut node2_vec = new_board.get(&node2).unwrap().clone();
                    nnv.append(&mut node2_vec);
                    nnv.retain(|e| *e != node1 && *e != node2);
                    nnb.insert(nni.clone(), nnv);
                    new_min_count = cmp::min(new_min_count, nni.len());
                    break;
                }
            }
            for j in &new_board {
                if j.0 == &node1 || j.0 == &node2 { continue; }
                let mut nnv:Vec<String> = vec![];
                for i in j.1{
                    if *i == node1 || *i == node2 { 
                        nnv.push(nni.clone()); 
                    } else {
                        nnv.push(i.clone());
                    }
                }
                nnb.insert(j.0.clone(), nnv);
                new_min_count = cmp::min(new_min_count, j.0.len());
            }
            new_board = nnb.clone();
            min_count = new_min_count;
            if new_board.len() == 2
            {
                break;
            }
        }
        let mut done = true;
        tot = 1;
        for i in new_board
        {
            if i.1.len() > 2 { done = false; }
            tot *= i.0.len()/3;
        }
        if done { break; }
    }
    println!("Part 1: {}", tot)
}

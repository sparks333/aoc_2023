use std::{fs::read_to_string};
use std::collections::HashMap;

fn main()
{
    let mut map = HashMap::new();
    let mut inst:Vec<char> = vec![];
    let mut start_nodes:Vec<String> = vec![];

    let mut state = 0;

    for line in read_to_string("input.txt").unwrap().lines()
    {
        if state == 0
        {
            inst = line.chars().collect();
            state += 1;
        }
        else if state == 2
        {
            let split_line:Vec<String> = line.split(|x:char| !x.is_ascii_alphanumeric()).filter(|s| !s.is_empty()).map(str::to_string).collect();
            map.insert(split_line[0].clone(), (split_line[0].clone(), split_line[1].clone(), split_line[2].clone()));
            if split_line[0].chars().last().unwrap() == 'A'
            {
                start_nodes.push(split_line[0].clone());
            }
        }
        else 
        {
            state += 1;    
        }
    }

    let mut steps = 0;
    let mut index = 0;
    
    let mut node = map.get("AAA").unwrap();
    loop 
    {
        let next_node;

        if node.0 == "ZZZ"
        {
            break;
        }

        if inst[index] == 'L'
        {
            next_node = map.get(&node.1).unwrap();
        }
        else
        {
            next_node = map.get(&node.2).unwrap();
        }
        steps += 1;
        node = next_node;

        index += 1;
        if index == inst.len()
        {
            index = 0;
        }
    }

    println!("Part 1: {}", steps);
    
    steps = 0;
    index = 0;
    let mut nodes:Vec<&(String, String, String)> = vec![];
    let mut inst_count:Vec<i32> = vec![0; start_nodes.len()];
    
    for i in start_nodes
    {
        nodes.push(map.get(&i).unwrap());
    }
    
    let mut z_count = 0;

    loop 
    {
        for k in 0..nodes.len()
        {
            if nodes[k].0.chars().last().unwrap() == 'Z'
            {
                if inst_count[k] == 0
                {
                    inst_count[k] = (steps / inst.len()) as i32;
                    z_count += 1;
                }
            }
        }

        if z_count == nodes.len()
        {
            break;
        }

        let mut next_nodes:Vec<&(String, String, String)> = vec![];

        for j in &nodes
        {
            if inst[index] == 'L'
            {
                next_nodes.push(map.get(&j.1).unwrap());
            }
            else
            {
                next_nodes.push(map.get(&j.2).unwrap());
            }
        }
        nodes = next_nodes.clone();
        index += 1;
        if index == inst.len()
        {
            index = 0;
        }
        steps += 1;
    }

    let mut total:i64 = 1;
    for i in inst_count
    {
        total *= i as i64;
    }

    total *= inst.len() as i64;
    println!("Part 2: {}", total);
}
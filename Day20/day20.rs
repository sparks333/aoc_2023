use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let mut network:HashMap<String, (char, HashMap<String, i32>, i32, Vec<String>)> = HashMap::new();
    for line in read_to_string("input.txt").unwrap().lines() {
        let split_line:Vec<String> = line.split([' ', '-', '>', ',']).filter(|&x| !x.is_empty()).map(str::to_string).collect();
        let node_name:String;
        let node_type:char;
        let mut node_dest:Vec<String> = vec![];
        if split_line[0] == "broadcaster" {
            node_type = 'b';
            node_name = split_line[0].clone(); 
        } else {
            node_type = split_line[0].chars().collect::<Vec<_>>()[0];
            node_name = String::from_iter(split_line[0].chars().collect::<Vec<_>>()[1..].to_vec().iter());
        }
        for i in 1..split_line.len() { node_dest.push(split_line[i].clone()); }
        network.insert(node_name, (node_type, HashMap::new(), 0, node_dest));
    }

    let net2 = network.clone();

    for n in &mut network {
        if n.1.0 == '&' {
            for m in &net2 {
                if m.1.3.contains(&n.0) { n.1.1.insert(m.0.clone(), -1); }
            }
        }
    }

    let part_2_net = network.clone();

    let mut pos_pulse = 0;
    let mut neg_pulse = 0;

    let mut pulses = vec![];

    for _i in 0..1000 {
        pulses.push(("broadcaster".to_string(), "".to_string(), -1));
        loop {
            if pulses.len() == 0 { break; }
            let p = pulses.pop().unwrap();
            if p.2 == -1 { neg_pulse += 1; } else { pos_pulse += 1; }
            if network.contains_key(&p.0) {
                let node = network.get_mut(&p.0).unwrap();
                if node.0 == '%' {
                    if p.2 == -1 {
                        if node.2 == 0 {
                            node.2 = 1;
                            for receipient in &node.3 { pulses.push((receipient.clone(), p.0.clone(), 1)); }
                        } else {
                            node.2 = 0;
                            for receipient in &node.3 { pulses.push((receipient.clone(), p.0.clone(), -1)); }
                        }
                    }
                }
                if node.0 == '&' {
                    let sender = node.1.get_mut(&p.1).unwrap();
                    *sender = p.2;

                    let mut all_pos = true;
                    for val in &node.1 { if *val.1 == -1 { all_pos = false; } }
                    if all_pos {
                        for receipient in &node.3 { pulses.push((receipient.clone(), p.0.clone(), -1)); }
                    } else {
                        for receipient in &node.3 { pulses.push((receipient.clone(), p.0.clone(), 1)); }
                    }
                }
                if node.0 == 'b' {
                    for receipient in &node.3 { pulses.push((receipient.clone(), p.0.clone(), p.2)); }
                }
            }
        }
    }
    let tot = neg_pulse * pos_pulse;
    println!("Part 1: {}", tot);

    let mut network = part_2_net.clone();

    let mut pulses = vec![];
    let mut button_presses = 0;

    let mut tg_button_presses = 0;
    let mut hn_button_presses = 0;
    let mut lz_button_presses = 0;
    let mut kh_button_presses = 0;

    loop {
        pulses.push(("broadcaster".to_string(), "".to_string(), -1));
        button_presses += 1;
        
        loop {
            if pulses.len() == 0 { break; }
            let p = pulses.pop().unwrap();

            if p.0 == "cs" && p.1 == "hn" && p.2 == 1 {
                if hn_button_presses > 0 { hn_button_presses = button_presses+1; } else { hn_button_presses = button_presses; }
            }

            if p.0 == "cs" && p.1 == "lz" && p.2 == 1 {
                if lz_button_presses > 0 { lz_button_presses = button_presses+1; } else { lz_button_presses = button_presses; }
            }

            if p.0 == "cs" && p.1 == "kh" && p.2 == 1 {
                if kh_button_presses > 0 { kh_button_presses = button_presses+1; } else { kh_button_presses = button_presses; }                
            }

            if p.0 == "cs" && p.1 == "tg" && p.2 == 1 {
                if tg_button_presses > 0 { tg_button_presses = button_presses+1; } else { tg_button_presses = button_presses; }                
            }

            if hn_button_presses > 0 && lz_button_presses > 0 && kh_button_presses > 0 && tg_button_presses > 0 {
                let tot:i64 = hn_button_presses * lz_button_presses * kh_button_presses * tg_button_presses;
                println!("Part 2: {}", tot);
                return;
            }

            if network.contains_key(&p.0) {
                let node = network.get_mut(&p.0).unwrap();
                if node.0 == '%' {
                    if p.2 == -1 {
                        if node.2 == 0 {
                            node.2 = 1;
                            for receipient in &node.3 { pulses.push((receipient.clone(), p.0.clone(), 1)); }
                        } else {
                            node.2 = 0;
                            for receipient in &node.3 { pulses.push((receipient.clone(), p.0.clone(), -1)); }
                        }
                    }
                }
                if node.0 == '&' {
                    let sender = node.1.get_mut(&p.1).unwrap();
                    *sender = p.2;
                    let mut all_pos = true;
                    for val in &node.1 {
                        if *val.1 == -1 { all_pos = false; }
                    }
                    if all_pos {
                        for receipient in &node.3 { pulses.push((receipient.clone(), p.0.clone(), -1)); }
                    } else {
                        for receipient in &node.3 { pulses.push((receipient.clone(), p.0.clone(), 1)); }
                    }
                }
                if node.0 == 'b' {
                    for receipient in &node.3 { pulses.push((receipient.clone(), p.0.clone(), p.2)); }
                }
            }
        }
    }
}
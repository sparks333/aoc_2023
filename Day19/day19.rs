use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let mut flows = HashMap::new();
    let mut tests:Vec<Vec<i32>> = vec![];
    let mut rules_state = 0;

    for line in read_to_string("input.txt").unwrap().lines()
    {
        if line.len() == 0
        {
            rules_state = 1;
            continue;
        }

        if rules_state == 0
        {
            let split_line:Vec<String> = line.split_terminator(['{', ',', '}']).map(str::to_string).collect();
            let flow_name = split_line[0].clone();
            let mut rules:Vec<(i32, char, i32, String)> = vec![];
            let final_rule;

            for i in 1..split_line.len()-1
            {
                let subline = &split_line[i];
                let mut attrib = 10;
                let mut cond = '?';
                let mut val:Vec<char> = vec![];
                let mut success:Vec<char> = vec![];
                let mut state = 0;

                for c in subline.chars()
                {
                    match state
                    {
                        0=>
                        {
                            match c
                            {
                                'x'=>attrib=0,
                                'm'=>attrib=1,
                                'a'=>attrib=2,
                                's'=>attrib=3,
                                _=>println!("Parse Error"),
                            }
                            state = 1;
                        },
                        1=>
                        {
                            cond = c;
                            state = 2;
                        },
                        2=>
                        {
                            if !c.is_numeric()
                            {
                                state = 3;
                            }
                            else
                            {
                                val.push(c);
                            }
                        },
                        3=>
                        {
                            success.push(c);
                        },
                        _=> println!("Parse Error"),
                    }
                }

                rules.push((attrib, cond, String::from_iter(val).parse::<i32>().unwrap(), String::from_iter(success)));
            }
            final_rule = split_line[split_line.len()-1].clone();
            flows.insert(flow_name, (rules, final_rule));
        }
        else 
        {
            let split_line:Vec<String> = line.split_terminator(['{', ',', '=', '}']).map(str::to_string).collect();
            let mut x = 0;
            let mut m = 0;
            let mut a = 0;
            let mut s = 0;

            for i in (1..split_line.len()).step_by(2)
            {
                match split_line[i].chars().collect::<Vec<_>>()[0]
                {
                    'x'=> x = split_line[i+1].parse::<i32>().unwrap(),
                    'm'=> m = split_line[i+1].parse::<i32>().unwrap(),
                    'a'=> a = split_line[i+1].parse::<i32>().unwrap(),
                    's'=> s = split_line[i+1].parse::<i32>().unwrap(),
                    _=>println!("Parse Error!"),
                }
            }

            tests.push(vec![x, m, a, s]);
        }
    }

    let mut accepted:Vec<Vec<i32>> = vec![];

    for t in tests
    {
        let mut flow_name = "in".to_string();
        let mut done:bool = false;
        loop {
            let rule = flows.get(&flow_name).unwrap();
            let mut rule_found:bool = false;
            for r in &rule.0
            {
                let mut pass = false;
                if r.1 == '>'
                {
                    pass = t[r.0 as usize] > r.2;
                }
                else if r.1 == '<'
                {
                    pass = t[r.0 as usize] < r.2;
                }
                else
                {
                    println!("Unknown operator");
                }
                if pass
                {
                    if r.3 == "A"
                    {
                        accepted.push(t.clone());
                        done = true;
                        break;
                    }
                    else if r.3 == "R"
                    {
                        done = true;
                        break;
                    }
                    else
                    {
                        flow_name = r.3.clone();
                        rule_found = true;
                        break;
                    }
                }
            }
            if done == true
            {
                break;
            }
            if !rule_found
            {
                if rule.1 == "A"
                {
                    accepted.push(t.clone());
                    break;
                }
                else if rule.1 == "R"
                {
                    break;
                }
                else
                {
                    flow_name = rule.1.clone();
                }
            }
        }
    }

    let mut tot = 0;

    for a in accepted
    {
        for b in a
        {
            tot += b;
        }
    }

    println!("Part 1: {}", tot);

    let mut state = vec![(vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)], "in".to_string())];

    let mut accepted:Vec<Vec<(i32, i32)>> = vec![];

    loop {

        if state.len() == 0
        {
            break;
        }

        let s = state.pop().unwrap();

        let mut flow_name = s.1.clone();
        let mut done:bool = false;
        loop {
            let rule = flows.get(&flow_name).unwrap();
            let mut rule_found:bool = false;
            let mut split_found = false;
            for r in &rule.0
            {
                let mut pass = false;
                let attrib = r.0 as usize;

                if s.0[attrib].0 < r.2 && s.0[attrib].1 > r.2
                {
                    let mut new_vec_1 = s.clone();
                    let mut new_vec_2 = s.clone();

                    new_vec_1.1 = flow_name.clone();
                    new_vec_2.1 = flow_name.clone();
                    if r.1 == '<'
                    {
                        new_vec_1.0[attrib].1 = r.2-1;
                        new_vec_2.0[attrib].0 = r.2;
                    }
                    else
                    {
                        new_vec_1.0[attrib].1 = r.2;
                        new_vec_2.0[attrib].0 = r.2+1;
                    }
                    state.push(new_vec_1.clone());
                    state.push(new_vec_2.clone());
                    split_found = true;
                    break;
                }
                if r.1 == '>'
                {
                    pass = s.0[attrib].0 > r.2;
                }
                else if r.1 == '<'
                {
                    pass = s.0[attrib].1 < r.2;
                }
                if pass
                {
                    if r.3 == "A"
                    {
                        accepted.push(s.0.clone());
                        done = true;
                        break;
                    }
                    else if r.3 == "R"
                    {
                        done = true;
                        break;
                    }
                    else
                    {
                        flow_name = r.3.clone();
                        rule_found = true;
                        break;
                    }
                }
            }
            if done == true || split_found == true
            {
                break;
            }
            if !rule_found
            {
                if rule.1 == "A"
                {
                    accepted.push(s.0.clone());
                    break;
                }
                else if rule.1 == "R"
                {
                    break;
                }
                else
                {
                    flow_name = rule.1.clone();
                }
            }
        }
    }

    let mut tot:i64 = 0;

    for a in accepted
    {
        let mut sub_tot:i64 = 1;
        for r in a
        {
            sub_tot *= ((r.1-r.0)+1) as i64;
        }
        tot += sub_tot;
    }

    println!("Part 2: {}", tot);
}
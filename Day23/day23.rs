use std::fs::read_to_string;
use std::cmp;
use std::collections::HashMap;

fn main() {

    let mut board:Vec<Vec<char>> = vec![];
    
    for line in read_to_string("input.txt").unwrap().lines() {
        board.push(line.chars().collect());
    }

    let target = (board[0].len()-2, board[0].len()-1);

    board[0][1] = 'O';

    let mut steps = vec![];
    steps.push(((1, 0), 0, board.clone()));
    let mut cost: Vec<Vec<i32>> = vec![vec![0; board[0].len()]; board.len()];

    let mut max_steps = 0;

    loop {
        if steps.len() == 0 { break; }
        let step = steps.pop().unwrap();
        let x = step.0.0;
        let y = step.0.1;
        let s_count = step.1;
        let map = step.2;

        if cost[y][x] > s_count { continue; }
        
        cost[y][x] = s_count;

        if x == target.0 && y == target.1 {
            max_steps = cmp::max(s_count, max_steps);
            continue;
        }

        if x > 0 && ( map[y][x-1] == '.' || map[y][x-1] == '<' ) {
            let mut new_map = map.clone();
            new_map[y][x-1] = 'O';
            steps.push(((x-1, y), s_count+1, new_map));
        }

        if x < board[0].len()-1 && ( map[y][x+1] == '.' || map[y][x+1] == '>' ) {
            let mut new_map = map.clone();
            new_map[y][x+1] = 'O';
            steps.push(((x+1, y), s_count+1, new_map));
        }

        if y > 0 && ( map[y-1][x] == '.' || map[y-1][x] == '^' ) {
            let mut new_map = map.clone();
            new_map[y-1][x] = 'O';
            steps.push(((x, y-1), s_count+1, new_map));
        }

        if y < board.len()-1 && ( map[y+1][x] == '.' || map[y+1][x] == 'v' ) {
            let mut new_map = map.clone();
            new_map[y+1][x] = 'O';
            steps.push(((x, y+1), s_count+1, new_map));
        }
    }

    println!("Part 1: {}", max_steps);

    let mut graph = HashMap::new();
    graph.insert((1, 0), vec![]);
    graph.insert((target.0, target.1), vec![]);

    let mut paths = vec![((1, 1), 1, (1, 0))];

    let mut map = board.clone();
    map[0][1] = '.';

    loop {
        if paths.len() == 0 {break;}
        let path = paths.pop().unwrap();

        let x = path.0.0;
        let y = path.0.1;
        let s_count = path.1;
        let prev_node = path.2;
        
        if x == prev_node.0 && y == prev_node.1 { continue; }

        let mut dir_count = 0;
        if x > 0 && map[y][x-1] != '#' { dir_count +=  1; }
        if x < board[0].len()-1 && map[y][x+1] != '#' { dir_count +=  1; }
        if y > 0 && map[y-1][x] != '#' { dir_count += 1; }
        if y < board.len()-1 && map[y+1][x] != '#' { dir_count += 1; }

        if dir_count > 2 || graph.contains_key(&(x, y)){
            if graph.contains_key(&(x, y)) {
                graph.get_mut(&(x, y)).unwrap().push((prev_node, s_count));
            } else {
                graph.insert((x, y), vec![(prev_node, s_count)]);
            }
            graph.get_mut(&prev_node).unwrap().push(((x, y), s_count));

            if x > 0 && ( map[y][x-1] != '#' && map[y][x-1] != 'O' ) {
                paths.push(((x-1, y), 1, (x, y)));
            }

            if x < board[0].len()-1 && ( map[y][x+1] != '#' && map[y][x+1] != 'O' ) {
                paths.push(((x+1, y), 1, (x, y)));
            }

            if y > 0 && ( map[y-1][x] != '#' && map[y-1][x] != 'O' ) {
                paths.push(((x, y-1), 1, (x, y)));
            }
    
            if y < board.len()-1 && ( map[y+1][x] != '#' && map[y+1][x] != 'O' ) {
                paths.push(((x, y+1), 1, (x, y)));
            }

        } else {

            map[y][x] = 'O';

            if x > 0 && ( map[y][x-1] != '#' && map[y][x-1] != 'O' ) {
                paths.push(((x-1, y), s_count+1, prev_node));
            }

            if x < board[0].len()-1 && ( map[y][x+1] != '#' && map[y][x+1] != 'O' ) {
                paths.push(((x+1, y), s_count+1, prev_node));
            }

            if y > 0 && ( map[y-1][x] != '#' && map[y-1][x] != 'O' ) {
                paths.push(((x, y-1), s_count+1, prev_node));
            }
    
            if y < board.len()-1 && ( map[y+1][x] != '#' && map[y+1][x] != 'O' ) {
                paths.push(((x, y+1), s_count+1, prev_node));
            }
        }
    }

    let mut max_steps = 0;

    let mut nodes = vec![((1, 0), 0, vec![])];
    loop {
        if nodes.len() == 0 { break; }
        let node:((usize, usize), i32, Vec<(usize, usize)>) = nodes.pop().unwrap();
        let curr_node: (usize, usize) = node.0;
        let cost = node.1;

        if curr_node == target {
            max_steps = cmp::max(max_steps, cost);
            continue;
        }

        let breadcrumbs = node.2;

        let paths = graph.get(&curr_node).unwrap();

        for path in paths {
            if !breadcrumbs.contains(&path.0) {
                let mut new_breadcrumbs = breadcrumbs.clone();
                new_breadcrumbs.push(path.0);
                nodes.push((path.0, cost + path.1, new_breadcrumbs));
            }
        }
    }

    println!("Part 2: {}", max_steps);
}

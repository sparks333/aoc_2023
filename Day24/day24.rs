use std::fs::read_to_string;
use nalgebra::*;

const PART_1_MIN:f64 = 200000000000000.0;
const PART_1_MAX:f64 = 400000000000000.0;

fn main() {

    let mut board = vec![];

    for line in read_to_string("input.txt").unwrap().lines() {
        let split_line:Vec<String> = line.split([' ', ',', '@']).filter(|x| !x.is_empty()).map(str::to_string).collect();
        let px = split_line[0].parse::<f64>().unwrap();
        let py = split_line[1].parse::<f64>().unwrap();
        let pz = split_line[2].parse::<f64>().unwrap();
        let vx = split_line[3].parse::<f64>().unwrap();
        let vy = split_line[4].parse::<f64>().unwrap();
        let vz = split_line[5].parse::<f64>().unwrap();
        board.push(((px, py, pz), (vx, vy, vz)));
    }   

    let mut tot = 0;

    let mut a1;
    let mut b1;
    let mut c1;
    let mut a2;
    let mut b2;
    let mut c2;
    let mut denom;
    let mut x_intersect;
    let mut y_intersect;

    for i in 0..board.len()-1
    {
        a1 = -board[i].1.1;
        b1 = board[i].1.0;
        c1 = a1*board[i].0.0 + b1*board[i].0.1;

        for j in i+1..board.len()
        {
            a2 = -board[j].1.1;
            b2 = board[j].1.0;
            c2 = a2*board[j].0.0 +b2*board[j].0.1;

            denom = a1*b2 - b1*a2;

            if denom == 0.0
            {
                continue;
            }

            x_intersect = (c1*b2 - b1*c2)/denom;
            y_intersect = (a1*c2 - c1*a2)/denom;

            let t1 = (x_intersect - board[i].0.0)/board[i].1.0;
            let t2 = (x_intersect - board[j].0.0)/board[j].1.0;

            if x_intersect >= PART_1_MIN && x_intersect <= PART_1_MAX &&
               y_intersect >= PART_1_MIN && y_intersect <= PART_1_MAX &&
               t1 >= 0.0 && t2 >= 0.0
            {
                tot += 1;
            }
        }
    }

    println!("Part 1: {}", tot);

    let p0 = Vector3::new(board[0].0.0, board[0].0.1, board[0].0.2);
    let v0 = Vector3::new(board[0].1.0, board[0].1.1, board[0].1.2);
    let p1 = Vector3::new(board[1].0.0, board[1].0.1, board[1].0.2);
    let v1 = Vector3::new(board[1].1.0, board[1].1.1, board[1].1.2);
    let p2 = Vector3::new(board[2].0.0, board[2].0.1, board[2].0.2);
    let v2 = Vector3::new(board[2].1.0, board[2].1.1, board[2].1.2);

    let a0 = -p0.cross(&v0) + p1.cross(&v1);
    let a1 = -p0.cross(&v0) + p2.cross(&v2);

    let a = Vector6::new(a0[0], a0[1], a0[2], a1[0], a1[1], a1[2]);

    let b00 = v0.cross_matrix() - v1.cross_matrix();
    let b01 = v0.cross_matrix() - v2.cross_matrix();
    let b10 = -p0.cross_matrix() + p1.cross_matrix();
    let b11 = -p0.cross_matrix() + p2.cross_matrix();

    let mut b = Matrix6::zeros();

    b[0] = b00[0];
    b[1] = b00[1];
    b[2] = b00[2];

    b[3] = b01[0];
    b[4] = b01[1];
    b[5] = b01[2];

    b[6] = b00[3];
    b[7] = b00[4];
    b[8] = b00[5];

    b[9] = b01[3];
    b[10] = b01[4];
    b[11] = b01[5];

    b[12] = b00[6];
    b[13] = b00[7];
    b[14] = b00[8];

    b[15] = b01[6];
    b[16] = b01[7];
    b[17] = b01[8];

    b[18] = b10[0];
    b[19] = b10[1];
    b[20] = b10[2];

    b[21] = b11[0];
    b[22] = b11[1];
    b[23] = b11[2];

    b[24] = b10[3];
    b[25] = b10[4];
    b[26] = b10[5];

    b[27] = b11[3];
    b[28] = b11[4];
    b[29] = b11[5];

    b[30] = b10[6];
    b[31] = b10[7];
    b[32] = b10[8]; 

    b[33] = b11[6];
    b[34] = b11[7];
    b[35] = b11[8]; 

    let res = b.try_inverse().unwrap() * a;

    let mut p = Vector3::new(res[0], res[1], res[2]);
    let v = Vector3::new(res[3].round(), res[4].round(), res[5].round());

    let t1 = ((p0-p).cross(&v0).dot(&(v.cross(&v0))) / (v.cross(&v0)).norm_squared()).round();
    p = (p0 + v0*t1) - (v*t1);

    println!("Part 2: {}", (p[0] + p[1] + p[2]));
}

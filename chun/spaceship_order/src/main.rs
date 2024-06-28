use std::{cmp, io, iter};

fn surrogate_cost(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    // Chevyshev distance

    return cmp::max((p1.0 - p2.0).abs(), (p1.1 - p2.1).abs());
}

fn greedy_initial_solution(v: Vec<(i32, i32)>) -> Vec<usize> {
    let mut curpt: (i32, i32) = (0, 0);
    let mut visited: Vec<bool> = iter::repeat(false).take(v.len()).collect();
    let mut ret = Vec::new();

    for _i in 0..v.len() {
        // add candidates greedily
        let mut best: Option<usize> = None;
        let mut bestdist = i32::MAX;
        for nextp in 0..v.len() {
            if visited[nextp] {
                continue;
            }

            let curcost = surrogate_cost(curpt, v[nextp]);
            if curcost < bestdist {
                best = Some(nextp);
                bestdist = curcost;
            }
        }
        let rbest = best.unwrap();
        visited[rbest] = true;
        ret.push(rbest);
        curpt = v[rbest];
    }
    return ret;
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();

    let mut v: Vec<(i32, i32)> = Vec::new();

    loop {
        match stdin.read_line(&mut buffer) {
            Ok(k) => {
                if k == 0 {
                    break;
                }
                let mut values = buffer.split_whitespace();
                let a = values.next();
                let b = values.next();
                let (a, b) = match a {
                    Some(x) => (x, b.unwrap()),
                    None => break,
                };
                let ai: i32 = a.parse().unwrap();
                let bi: i32 = b.parse().unwrap();

                v.push((ai, bi));
                buffer.clear();
            }
            Err(_) => {
                break;
            }
        }
    }

    //println!("start reorder");
    let initial_order = greedy_initial_solution(v.clone());

    // TODO(chun): run SA
    for pos in initial_order {
        println!("{} {}", v[pos].0, v[pos].1);
    }
}

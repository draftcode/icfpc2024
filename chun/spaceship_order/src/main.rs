use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::iter::repeat;
use std::{cmp, io, iter};

fn surrogate_cost(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    // Chevyshev distance

    return cmp::max((p1.0 - p2.0).abs(), (p1.1 - p2.1).abs());
}

fn greedy_initial_solution(v: Vec<(i32, i32)>) -> Vec<usize> {
    let mut curpt: (i32, i32) = (0, 0);
    let mut visited: Vec<bool> = iter::repeat(false).take(v.len()).collect();
    let mut ret = Vec::new();

    #[allow(unused_mut)]
    let mut specialdist: HashMap<((i32, i32), (i32, i32)), i32> = HashMap::new();
    /* for problem 20
    for (p1, p2) in [((0, 0), (38, 4)), ((3, 50), (13, 423)), ((270, 296), (269, 307)), ((310, 81), (372, 83)), ((0, 674), (1, 553))] {
        specialdist.insert((p1, p2), 0);
    }
    */
    /* for problem 21 */
    /*
    for (p1, p2) in [((0, 2), (2, 247)), ((113, 21), (270, 3)), ((84, 415), (95, 522)), ((724, 266), (669, 261)), ((613, 483), (614, 475)),
    ((995, 187), (994, 51)), ((615, 470), (614, 475)),
    ((917, 6), (462, 0)),
    ((999, 368), (915, 993)),
    ((998, 918), (993, 740))] {
        specialdist.insert((p1, p2), 0);
    }*/

    for _i in 0..v.len() {
        // add candidates greedily
        let mut best: Option<usize> = None;
        let mut bestdist = i32::MAX;
        for nextp in 0..v.len() {
            if visited[nextp] {
                continue;
            }

            let mut curcost = surrogate_cost(curpt, v[nextp]);
            if let Some(_v) = specialdist.get(&(curpt, v[nextp])) {
                curcost = 0;
                eprintln!("{curpt:?} {:?} {bestdist} {:?}", v[nextp], v[best.unwrap()]);
            }
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

fn sa_2opt(v: &Vec<(i32, i32)>) -> Vec<usize> {
    let mut order = Vec::new();
    for i in 0..v.len() {
        order.push(i as usize);
    }
    let mut invorder = Vec::new();
    for i in 0..v.len() {
        invorder.push(i as usize);
    }
    let mut rng = &mut rand::thread_rng();
    //let mut urand = rand::distributions::uniform::Uniform::new(0., 1.);
    let mut temp = 5.;
    let mut accept = 0;
    let mut linearcost: Vec<i32> = (0..order.len() - 1)
        .map(|x| surrogate_cost(v[x], v[x + 1]))
        .collect();
    eprintln!(
        "Initial cost = {}",
        linearcost.iter().fold(0, |acc, x| acc + x)
    );
    use hora::core::ann_index::ANNIndex;
    let mut index_hora = hora::index::hnsw_idx::HNSWIndex::<f32, usize>::new(
        2,
        &hora::index::hnsw_params::HNSWParams::<f32>::default(),
    );
    for (i, p) in v.iter().enumerate() {
        index_hora.add(&[p.0 as f32, p.1 as f32], i).unwrap();
    }
    index_hora
        .build(hora::core::metrics::Metric::Euclidean)
        .unwrap();
    for iter in 0..10000000 {
        // 2-opt
        let dist = WeightedIndex::new(&linearcost).unwrap();
        let mut a = dist.sample(&mut rng);
        let pt = v[order[a]];
        let nearby = index_hora.search(&[pt.0 as f32, pt.1 as f32], 10);
        let cands: Vec<usize> = nearby
            .iter()
            .map(|i| invorder[*i])
            .filter(|x| *x != a && *x != a + 1 && *x != a - 1)
            .collect();
        let mut x = *cands.choose(&mut rng).unwrap();
        if a > x {
            std::mem::swap(&mut a, &mut x);
        }
        let costab = surrogate_cost(v[order[a]], v[order[a + 1]]);
        let costxy = if x == v.len() - 1 {
            0
        } else {
            surrogate_cost(v[order[x]], v[order[x + 1]])
        };
        let costax = surrogate_cost(v[order[a]], v[order[x]]);
        let costby = if x == v.len() - 1 {
            0
        } else {
            surrogate_cost(v[order[a + 1]], v[order[x + 1]])
        };
        let delta = ((costax + costby) - (costab + costxy)) as f64;

        let dovert = (-delta / temp).exp();
        let u: f64 = rng.gen();
        if u < dovert {
            // swap
            let mut neworder: Vec<usize> = (0..a + 1).map(|i| order[i]).collect();
            neworder.push(order[x]);
            neworder.extend((a + 1..x).rev().map(|i| order[i]));
            if x != v.len() - 1 {
                neworder.push(order[x + 1]);
                neworder.extend((x + 2..v.len()).map(|i| order[i]));
            }
            assert_eq!(neworder.len(), order.len());
            order.swap_with_slice(&mut neworder);
            let mut newlinearcost: Vec<i32> = (0..order.len() - 1)
                .map(|x| surrogate_cost(v[neworder[x]], v[neworder[x + 1]]))
                .collect();
            linearcost.swap_with_slice(&mut newlinearcost);
            let mut newinvorder: Vec<usize> = repeat(0).take(order.len()).collect();
            for i in 0..order.len() {
                newinvorder[order[i]] = i;
            }
            invorder.swap_with_slice(&mut newinvorder);
            accept += 1;
        }
        if iter % 1000 == 0 && iter > 0 {
            let score = (0..order.len() - 1).fold(0, |acc, i| {
                acc + surrogate_cost(v[order[i]], v[order[i + 1]])
            });
            eprintln!(
                "Iter {iter}, score = {score}, T = {temp:.2} ac ratio = {:.3}",
                (accept as f64) / (iter as f64)
            );
        }
        temp *= 0.9999999;
    }
    return order;
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
                if (ai, bi) != (0, 0) {
                    v.push((ai, bi)); // remove from the beginning
                }
                buffer.clear();
            }
            Err(_) => {
                break;
            }
        }
    }
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        //println!("start reorder");
        let initial_order = greedy_initial_solution(v.clone());
        let v: Vec<(i32, i32)> = initial_order.iter().map(|i| v[*i].clone()).collect();
        for pos in 0..initial_order.len() {
            println!("{} {}", v[pos].0, v[pos].1);
        }
    } else {
        let initial_order = greedy_initial_solution(v.clone());
        let v: Vec<(i32, i32)> = initial_order.iter().map(|i| v[*i].clone()).collect();
        let final_order = sa_2opt(&v);
        for pos in final_order {
            println!("{} {}", v[pos].0, v[pos].1);
        }
    }
}

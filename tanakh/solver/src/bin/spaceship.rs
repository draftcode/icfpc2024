use std::{cmp::Reverse, collections::HashSet};

use euclid::default::*;

fn main() {
    let ps: Vec<Point2D<i64>> = std::io::stdin()
        .lines()
        .filter_map(|line| {
            let v = line
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            if v.len() != 2 {
                None
            } else {
                Some(Point2D::new(v[0], v[1]))
            }
        })
        .collect();

    // solve_shortest(ps);
    solve_basic(ps);
}

fn solve_basic(mut ps: Vec<Point2D<i64>>) {
    ps.sort_by_key(|p| (p.x, p.y));
    ps.dedup();

    use rand::prelude::SliceRandom;
    ps.shuffle(&mut rand::thread_rng());

    let n = ps.len();

    let mut cur = Point2D::new(0, 0);
    let mut v = Vector2D::new(0, 0);

    let mut moves = vec![];

    let mut done = vec![false; n];

    for turn in 1..=n {
        // let mut best = (0, i64::MAX, i64::MAX);

        // for i in 0..n {
        //     if done[i] {
        //         continue;
        //     }

        //     let dist = calc_dist(cur, v, ps[i], None);
        //     if (dist.0, dist.1.square_length()) < (best.1, best.2) {
        //         best = (i, dist.0, dist.1.square_length());
        //     }
        // }

        let mut nearest = (0, i64::MAX);

        for i in 0..n {
            if done[i] {
                continue;
            }

            // let dist = calc_dist(cur, v, ps[i], None).0;
            // if dist <= nearest.1 {
            //     if dist < nearest.1 {
            //         nearest = (i, dist);
            //     }
            // }

            let diff = ps[i] - cur;
            let dist = diff.square_length();
            if dist < nearest.1 {
                // let d = calc_dist(cur, v, ps[i], None);
                nearest = (i, dist);
            }
        }

        let i = nearest.0;
        let (t, nv) = calc_dist(cur, v, ps[i], Some(&mut moves));
        v = nv;
        cur = ps[i];
        done[i] = true;

        let total = moves.len();
        let est = total * n / turn;

        eprintln!(
            "* {turn} / {n}: total: {}, est: {est}, movs: {t} pos: {:?} vel: {:?}",
            moves.len(),
            cur,
            v
        );
    }

    eprintln!("total: {}", moves.len());
    println!("{}", moves.into_iter().map(to_move).collect::<String>());
}

fn to_move(v: Vector2D<i64>) -> char {
    match (v.x, v.y) {
        (-1, -1) => '1',
        (0, -1) => '2',
        (1, -1) => '3',
        (-1, 0) => '4',
        (0, 0) => '5',
        (1, 0) => '6',
        (-1, 1) => '7',
        (0, 1) => '8',
        (1, 1) => '9',
        _ => unreachable!(),
    }
}

fn calc_dist(
    cur: Point2D<i64>,
    v: Vector2D<i64>,
    to: Point2D<i64>,
    mut moves: Option<&mut Vec<Vector2D<i64>>>,
) -> (i64, Vector2D<i64>) {
    let mut lo = 0;
    let mut hi = 1_000_000;

    while hi - lo > 1 {
        let m = (lo + hi) / 2;

        let d = to - (cur + v * m);

        // 1 + 2 + ... + m
        let maxd = m * (m + 1) / 2;

        if d.x.abs() <= maxd && d.y.abs() <= maxd {
            hi = m;
        } else {
            lo = m;
        }
    }

    let ret = hi;

    let d = to - (cur + v * ret);

    let mut dx = d.x;
    let mut dy = d.y;

    // dbg!(ret, cur, v, to);

    if let Some(ref mut moves) = moves {
        for _ in 0..ret {
            (*moves).push(Vector2D::new(0, 0));
        }
    }

    let mut curm = ret;
    let mut xc = 0;
    while dx != 0 {
        assert!(curm > 0);
        let c = dx.abs().min(curm);
        if dx > 0 {
            if let Some(ref mut moves) = moves {
                let len = moves.len();
                moves[len - c as usize].x = 1;
            }
            xc += 1;
            dx -= c;
            curm = c - 1;
        } else {
            if let Some(ref mut moves) = moves {
                let len = moves.len();
                moves[len - c as usize].x = -1;
            }
            xc -= 1;
            dx += c;
            curm = c - 1;
        }
    }
    // dbg!(ret);

    let mut curm = ret;
    let mut yc = 0;
    while dy != 0 {
        assert!(curm > 0);
        let c = dy.abs().min(curm);
        if dy > 0 {
            if let Some(ref mut moves) = moves {
                let len = moves.len();
                moves[len - c as usize].y = 1;
            }
            yc += 1;
            dy -= c;
            curm = c - 1;
        } else {
            if let Some(ref mut moves) = moves {
                let len = moves.len();
                moves[len - c as usize].y = -1;
            }
            yc -= 1;
            dy += c;
            curm = c - 1;
        }
    }
    // dbg!(ret);

    (ret, v + Vector2D::new(xc, yc))
}

fn solve_shortest(ps: Vec<Point2D<i64>>) {
    let mut ps = ps.iter().copied().collect::<HashSet<_>>();
    ps.remove(&Point2D::new(0, 0));

    const EXTRA_LIMIT: usize = 23;

    eprintln!("len: {}", ps.len());

    let mut cur = vec![(Point2D::new(0, 0), Vector2D::new(0, 0), 0, vec![], ps)];

    while !cur.is_empty() {
        let mut next = vec![];

        for (cur, v, extra, moves, ps) in cur {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let nv = v + Vector2D::new(dx, dy);
                    let npos = cur + nv;

                    if !ps.contains(&npos) && extra + 1 > EXTRA_LIMIT {
                        continue;
                    }

                    let mut moves = moves.clone();
                    moves.push(Vector2D::new(dx, dy));

                    let mut ps = ps.clone();
                    let removed = ps.remove(&npos);

                    if ps.is_empty() {
                        println!("total: {}", moves.len());
                        println!(
                            "moves: {}",
                            moves.into_iter().map(to_move).collect::<String>()
                        );
                        return;
                    }

                    next.push((npos, nv, extra + (!removed) as usize, moves, ps));
                }
            }
        }

        cur = next;
    }

    panic!("no solution");
}

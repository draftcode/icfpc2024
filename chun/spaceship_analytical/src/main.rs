use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::iter::zip;
use std::{cmp, io};

type Point = (i32, i32);
type Velocity = (i32, i32);
type Acceleration = (i32, i32);

#[allow(dead_code)]
fn surrogate_cost(p1: Point, p2: Point) -> i32 {
    // Chevyshev distance

    return cmp::max((p1.0 - p2.0).abs(), (p1.1 - p2.1).abs());
}

fn surrogate_cost_with_velocity(curpt: Point, curv: Velocity, goal: Point) -> i32 {
    let dx = goal.0 - curpt.0;
    let dy = goal.1 - curpt.1;
    let vx = curv.0;
    let (avx, xpenalty) = if (dx.signum() - vx.signum()).abs() <= 1 {
        (cmp::max(1, vx.abs()), 0)
    } else {
        (1, vx.abs() * vx.abs() + 1)
    };
    let vy = curv.1;
    let (avy, ypenalty) = if (dy.signum() - vy.signum()).abs() <= 1 {
        (cmp::max(1, vy.abs()), 0)
    } else {
        (1, vy.abs() * vy.abs() + 1)
    };
    let adx = dx.abs();
    let ady = dy.abs();

    return cmp::max(adx / avx + xpenalty, ady / avy + ypenalty);
    //return surrogate_cost(curpt, goal);
}

fn encode_thrust_into_keypad(thrusts: Vec<Acceleration>) -> String {
    let mut retbuf = String::new();

    for thrust in thrusts {
        let ix: usize = ((thrust.0 + 1) + (thrust.1 + 1) * 3) as usize;
        let c: u8 = "123456789".bytes().nth(ix).unwrap();
        retbuf.extend([c as char]);
    }

    return retbuf;
}

#[allow(dead_code)]
fn decode_thrust_from_keypad(s: &str) -> Vec<Acceleration> {
    let mut ret = Vec::new();

    for c in s.as_bytes() {
        let n = (*c - ('1' as u8)) as i32;
        let dx = n % 3 - 1;
        let dy = n / 3 - 1;
        ret.push((dx, dy))
    }
    return ret;
}

#[cfg(test)]
mod encode_decode_thrust_test {
    use super::*;

    #[test]
    fn test_encode_thrust() {
        assert_eq!(
            encode_thrust_into_keypad(vec![(-1, 1), (0, 0), (1, 0)]),
            "756"
        );
    }

    #[test]
    fn test_decode_thrust() {
        assert_eq!(
            decode_thrust_from_keypad("756"),
            vec![(-1, 1), (0, 0), (1, 0)]
        );
    }
}

fn simulate(inip: Point, iniv: Velocity, order: &Vec<Acceleration>) -> Vec<(Point, Velocity)> {
    let mut ret = Vec::new();
    let mut p = inip;
    let mut v = iniv;
    for acc in order {
        v = (v.0 + acc.0, v.1 + acc.1);
        p = (p.0 + v.0, p.1 + v.1);
        ret.push((p, v))
    }
    return ret;
}

#[cfg(test)]
mod simulate_test {
    use super::*;

    // 236659 visits, in this order, the following squares: (0,0) (0,-1) (1,-3) (3,-5) (6,-7) (9,-9) (13,-10)
    #[test]
    fn test_simulate() {
        let thrusts = decode_thrust_from_keypad("236659");
        let expected_points = [(0, -1), (1, -3), (3, -5), (6, -7), (9, -9), (13, -10)];
        let res = simulate((0, 0), (0, 0), &thrusts);
        let posres: Vec<Point> = res.into_iter().map(|x| x.0).collect();
        assert_eq!(posres, expected_points);
    }
}

// returns the relative coordinate range (inclusive) available at time t
fn get_oneaxis_available_xs(v: i32, t: i32) -> (i32, i32) {
    let delta = t * (t + 1) / 2;
    return (v * t - delta, v * t + delta);
}

// FIXME: we should be able to solve this by not looping but considering the number of solutions in the quadratic equation
fn get_first_available_t(dx: i32, vinit: i32) -> i32 {
    for t in 0.. {
        let r = get_oneaxis_available_xs(vinit, t);
        if dx >= r.0 && dx <= r.1 {
            return t;
        }
    }
    panic!("should never reached")
}

#[cfg(test)]
mod axis_availability_test {
    use super::*;

    #[test]
    fn test_axis_v0() {
        assert_eq!(get_oneaxis_available_xs(0, 0), (0, 0));
        assert_eq!(get_oneaxis_available_xs(0, 1), (-1, 1));
        assert_eq!(get_oneaxis_available_xs(0, 2), (-3, 3));
    }

    #[test]
    fn test_axis_v1() {
        assert_eq!(get_oneaxis_available_xs(1, 0), (0, 0));
        assert_eq!(get_oneaxis_available_xs(1, 1), (0, 2));
        assert_eq!(get_oneaxis_available_xs(1, 2), (-1, 5));
    }

    #[test]
    fn test_first_available() {
        assert_eq!(get_first_available_t(0, 0), 0);
        assert_eq!(get_first_available_t(1, 0), 1);
        assert_eq!(get_first_available_t(-1, 0), 1);
        assert_eq!(get_first_available_t(2, 0), 2);
        assert_eq!(get_first_available_t(3, 0), 2);
        assert_eq!(get_first_available_t(4, 0), 3);
        assert_eq!(get_first_available_t(5, 0), 3);
        assert_eq!(get_first_available_t(6, 0), 3);
        assert_eq!(get_first_available_t(-4, 0), 3);
        assert_eq!(get_first_available_t(-5, 0), 3);
        assert_eq!(get_first_available_t(-6, 0), 3);
        assert_eq!(get_first_available_t(-6, 0), 3);
        assert_eq!(get_first_available_t(0, 1), 0);
        assert_eq!(get_first_available_t(1, 1), 1);
        assert_eq!(get_first_available_t(2, 1), 1);
        assert_eq!(get_first_available_t(3, 1), 2);
        assert_eq!(get_first_available_t(4, 1), 2);
        assert_eq!(get_first_available_t(-1, 1), 2);
    }
}

fn get_pyramid_area(n: i32) -> i32 {
    // OEIS A002620 Quarter-squares: a(n) = floor(n/2)*ceiling(n/2). Equivalently, a(n) = floor(n^2/4).
    // note OEIS A002620 starts from 0, 0, 1, 2...
    return (n + 1) * (n + 1) / 4;
}

fn get_num_completed_pyramid_size(a: i32) -> i32 {
    // a(n) = floor((n+1)^2/4)
    // <=> ((n+1)^2-3)/4 <= a(n) <= (n+1)^2/4
    // <=> sqrt(4 a(n)) - 1 <= n <= sqrt(4 a(n) + 3) - 1
    if a == 1 {
        return 1;
    }
    let nhigh = (((4 * a + 3) as f64).sqrt() as i32) - 1;
    if get_pyramid_area(nhigh) <= a {
        return nhigh;
    } else {
        return nhigh - 1; // for large enough a this should work
    }
}

#[cfg(test)]
mod pyramid_num_test {
    use super::*;

    #[test]
    fn test_pyramid_num() {
        assert_eq!(get_pyramid_area(0), 0);
        assert_eq!(get_pyramid_area(1), 1);
        assert_eq!(get_pyramid_area(2), 2);
        assert_eq!(get_pyramid_area(3), 4);
        assert_eq!(get_pyramid_area(4), 6);
        assert_eq!(get_pyramid_area(5), 9);
        assert_eq!(get_pyramid_area(6), 12);
    }

    #[test]
    fn test_inv_pyramid_num() {
        assert_eq!(get_num_completed_pyramid_size(0), 0);
        assert_eq!(get_num_completed_pyramid_size(1), 1);
        assert_eq!(get_num_completed_pyramid_size(2), 2);
        assert_eq!(get_num_completed_pyramid_size(3), 2);
        assert_eq!(get_num_completed_pyramid_size(4), 3);
        assert_eq!(get_num_completed_pyramid_size(5), 3);
        assert_eq!(get_num_completed_pyramid_size(6), 4);
        assert_eq!(get_num_completed_pyramid_size(7), 4);
        assert_eq!(get_num_completed_pyramid_size(8), 4);
        assert_eq!(get_num_completed_pyramid_size(9), 5);
        assert_eq!(get_num_completed_pyramid_size(10), 5);
        assert_eq!(get_num_completed_pyramid_size(11), 5);
        assert_eq!(get_num_completed_pyramid_size(12), 6);
    }
}

/*
 The speed is controlled by adding / removing the pyramid with a fixed area ("to_decrease" variable).
        o
  ^    oo
  |   koo
 v|  xxxo    To lower the speed while fix the area (assuming v(t=0) = 0) of x to be 8, "o" and "k" must be removed.
  | xxxxx    We first remove "o" and then start removing "k"
  --------> t
*/
fn get_available_vrange(dx: i32, vinit: i32, t: i32) -> Option<(i32, i32)> {
    let xr = get_oneaxis_available_xs(vinit, t);
    if dx < xr.0 || dx > xr.1 {
        return None;
    }
    //eprintln!("dx={dx}, vinit={vinit}, t={t}, xr={xr:?}");
    let to_decrease = xr.1 - dx;
    let removed_pyramid_size = get_num_completed_pyramid_size(to_decrease); // in the upper example, this corresponds to number of "o" at the final t
                                                                            //eprintln!("to_decrease = {to_decrease}, removed_pyramid_size={removed_pyramid_size}");
    let slowest = vinit + t - removed_pyramid_size;
    let to_increase = dx - xr.0;
    let added_pyramid_size = get_num_completed_pyramid_size(to_increase);
    let fastest = vinit - t + added_pyramid_size;
    return Some((slowest, fastest));
}

#[cfg(test)]
mod vrange_test {
    use super::*;

    #[test]
    fn test_vrange() {
        assert_eq!(get_available_vrange(0, 0, 0), Some((0, 0)));
        assert_eq!(get_available_vrange(0, 0, 1), Some((0, 0)));
        assert_eq!(get_available_vrange(0, 0, 2), Some((0, 0)));
        assert_eq!(get_available_vrange(0, 1, 0), Some((1, 1)));
        assert_eq!(get_available_vrange(0, 1, 1), Some((0, 0)));
        assert_eq!(get_available_vrange(0, 1, 2), Some((0, 0)));
    }
}

/*      __A
   __B  o
  ^    oo
  |   aoo         (1) remove 'o' so that vend is correct
 v|  ecbo         (2) count remaining points to remove (to_be_removed)
  | xxfdx         (3) remove in abcdef... order. Note the size may be (L/2, L/2, ...) or (L/2, L/2 + 1, L/2, L/2 + 1 ...)
  |xxxxxx ____\t  (4) reconstruct the sequence
  |xxxxxx     /   A: half_pyramid_size
  | xxxxx         B: n_initial_accel
  |  xxxx
  |   xxx
  |    xx
  |     x
*/
fn generate_accelerate_seqs(dx: i32, vinit: i32, vend: i32, t: i32) -> Vec<i32> {
    #[cfg(debug_assertions)]
    {
        let range = get_available_vrange(dx, vinit, t);
        match range {
            Some(range) => {
                assert!(range.0 <= vend);
                assert!(vend <= range.1);
            }
            None => {
                assert!(false);
            }
        }
    }
    let pyramid_size = vinit + t - vend;
    //eprintln!("pyrsize = {pyramid_size}");
    assert!(pyramid_size >= 0);
    let half_pyramid_size = pyramid_size / 2;
    let x_atmost = get_oneaxis_available_xs(vinit, t).1;
    assert!(dx <= x_atmost);
    let removed_area = get_pyramid_area(pyramid_size);
    let to_be_removed_area = (x_atmost - removed_area) - dx;
    let area_per_twolines = if pyramid_size % 2 == 0 {
        half_pyramid_size * 2
    } else {
        half_pyramid_size * 2 + 1
    };
    let n_twolines = if area_per_twolines != 0 {
        to_be_removed_area / area_per_twolines
    } else {
        0
    };
    let remainder_remove_twolines = if area_per_twolines != 0 {
        to_be_removed_area % area_per_twolines
    } else {
        0
    };
    let deccel_length = half_pyramid_size + pyramid_size % 2;
    let n_initial_accel = t - deccel_length - n_twolines; // actually initial_acceleration may be lower, but it will be fixed when actually filling the number
    let mut retvec: Vec<i32> = Vec::new();
    for _i in 0..n_initial_accel {
        retvec.push(1);
    }
    for _i in 0..deccel_length {
        retvec.push(-1);
    }
    if pyramid_size % 2 == 1 {
        retvec[n_initial_accel as usize] = 0;
    }
    for _i in 0..n_twolines {
        retvec.push(1);
    }
    assert_eq!(retvec.len(), t as usize);
    //eprintln!("retvec initial: {retvec:?}");

    // this can be effectively written by just changing two endpoints, but this routine requires O(t) anyway
    let remove_first_cycle = cmp::min(remainder_remove_twolines, half_pyramid_size);
    let remove_second_cycle = cmp::max(remainder_remove_twolines - half_pyramid_size, 0);
    if pyramid_size % 2 == 0 {
        for i in 0..remove_first_cycle {
            retvec[(n_initial_accel - 1 + i) as usize] -= 1;
            retvec[(n_initial_accel - 1 + i + 1) as usize] += 1;
            assert!(retvec[(n_initial_accel - 1 + i) as usize].abs() <= 1);
            assert!(retvec[(n_initial_accel - 1 + i + 1) as usize].abs() <= 1);
        }
        //eprintln!("retvec after phase 1: {retvec:?}");
        for i in 0..remove_second_cycle {
            retvec[(n_initial_accel - 1 + i) as usize] -= 1;
            retvec[(n_initial_accel - 1 + i + 1) as usize] += 1;
            assert!(retvec[(n_initial_accel - 1 + i) as usize].abs() <= 1);
            assert!(retvec[(n_initial_accel - 1 + i + 1) as usize].abs() <= 1);
        }
    } else {
        for i in 0..remove_first_cycle {
            retvec[(n_initial_accel + i) as usize] -= 1;
            retvec[(n_initial_accel + i + 1) as usize] += 1;
            assert!(retvec[(n_initial_accel + i) as usize].abs() <= 1);
            assert!(retvec[(n_initial_accel + i + 1) as usize].abs() <= 1);
        }
        //eprintln!("retvec after phase 1: {retvec:?}");
        let remove_second_cycle = cmp::max(remainder_remove_twolines - half_pyramid_size, 0);
        for i in 0..remove_second_cycle {
            retvec[(n_initial_accel - 1 + i) as usize] -= 1;
            retvec[(n_initial_accel - 1 + i + 1) as usize] += 1;
            assert!(retvec[(n_initial_accel - 1 + i) as usize].abs() <= 1);
            assert!(retvec[(n_initial_accel - 1 + i + 1) as usize].abs() <= 1);
        }
    }

    return retvec;
}

#[cfg(test)]
mod generate_test {
    use super::*;

    #[test]
    fn test_generate() {
        assert_eq!(generate_accelerate_seqs(1, 0, 2, 5), vec![-1, 0, 1, 1, 1]);
        assert_eq!(generate_accelerate_seqs(2, 0, 2, 5), vec![0, -1, 1, 1, 1]);
        assert_eq!(generate_accelerate_seqs(3, 0, 2, 5), vec![0, 0, 0, 1, 1]);
        assert_eq!(generate_accelerate_seqs(4, 0, 2, 5), vec![1, -1, 0, 1, 1]);
        assert_eq!(generate_accelerate_seqs(15, 0, 5, 5), vec![1, 1, 1, 1, 1]);
        assert_eq!(generate_accelerate_seqs(14, 0, 4, 5), vec![1, 1, 1, 1, 0]);
        assert_eq!(
            generate_accelerate_seqs(-15, 0, -5, 5),
            vec![-1, -1, -1, -1, -1]
        );
        assert_eq!(
            generate_accelerate_seqs(0, 0, 2, 6),
            vec![-1, 0, 0, 1, 1, 1]
        );
        assert_eq!(
            generate_accelerate_seqs(-1, 0, 2, 6),
            vec![-1, -1, 1, 1, 1, 1]
        );
        assert_eq!(
            generate_accelerate_seqs(1, 0, 2, 6),
            vec![0, -1, 0, 1, 1, 1]
        );
        assert_eq!(
            generate_accelerate_seqs(2, 0, 2, 6),
            vec![0, 0, -1, 1, 1, 1]
        );
        assert_eq!(
            generate_accelerate_seqs(3, 0, 2, 6),
            vec![1, -1, -1, 1, 1, 1]
        );
    }
}

fn generate_visit_plan(
    curp: Point,
    curv: Velocity,
    targetp: Point,
    numplan: usize,
) -> Vec<(i32, Velocity)> {
    let mut ret = Vec::new();
    let dx = targetp.0 - curp.0;
    let dy = targetp.1 - curp.1;
    let mut t = -1;
    while ret.len() < numplan {
        t += 1;
        let xvr = get_available_vrange(dx, curv.0, t);
        let yvr = get_available_vrange(dy, curv.1, t);
        match (xvr, yvr) {
            (Some(xvr), Some(yvr)) => {
                for vx in xvr.0..xvr.1 + 1 {
                    for vy in yvr.0..yvr.1 + 1 {
                        ret.push((t, (vx, vy)));
                    }
                }
            }
            _ => {
                continue;
            }
        }
    }
    return ret;
}

fn actualize_visit_plan(
    curp: Point,
    curv: Velocity,
    targetp: Point,
    targetv: Velocity,
    t: i32,
) -> Vec<Acceleration> {
    let xaccl = generate_accelerate_seqs(targetp.0 - curp.0, curv.0, targetv.0, t);
    let yaccl = generate_accelerate_seqs(targetp.1 - curp.1, curv.1, targetv.1, t);
    return zip(xaccl, yaccl).collect();
}

mod generate_plan_test {
    use super::*;

    #[test]
    fn test_generate_plan() {
        let curp = (1, 2);
        let curv = (-2, 1);
        let targetpt = (12, 10);
        let plan = generate_visit_plan(curp, curv, targetpt, 10);
        assert!(plan.len() >= 10);
        eprintln!("{plan:?}");
        for (t, targetv) in plan {
            let accl = actualize_visit_plan(curp, curv, targetpt, targetv, t);
            let rres = simulate(curp, curv, &accl);
            assert!(rres.len() == t as usize);
            assert_eq!(rres.last().unwrap().0, targetpt);
            assert_eq!(rres.last().unwrap().1, targetv);
        }
    }
}

fn visit_point(curpt: Point, curvel: Velocity, targetpt: Point) -> (Vec<Acceleration>, Velocity) {
    let p = generate_visit_plan(curpt, curvel, targetpt, 1);
    let p = p[0];
    let t = p.0;
    let vend = p.1;
    let accl = actualize_visit_plan(curpt, curvel, targetpt, p.1, p.0);
    return (accl, vend);
}

fn solve_greedy(v: Vec<(i32, i32)>) {
    let mut curpt = (0, 0);
    let mut curvel = (0, 0);
    let mut buffer = String::new();
    for i in 0..v.len() {
        let targetpt = v[i];
        let (accl, vend) = visit_point(curpt, curvel, targetpt);
        let encoded = encode_thrust_into_keypad(accl);
        buffer += encoded.as_str();
        eprintln!(
            "{i}/{} {curpt:?} {curvel:?} to {targetpt:?} {vend:?} res=\"{encoded}\"",
            v.len()
        );
        curpt = v[i];
        curvel = vend;
    }
    println!("{}", buffer);
}

fn solve_la1(v: Vec<(i32, i32)>) {
    let mut curpt = (0, 0);
    let mut curvel = (0, 0);
    let mut buffer = String::new();
    let nplan1 = 100;
    let nplan2 = 100;
    for i in 0..v.len() - 1 {
        let mut fastest_time = i32::MAX;
        let mut fastest = None;
        let plan1 = generate_visit_plan(curpt, curvel, v[i], nplan1);
        for (t1, v1) in plan1 {
            let plan2 = generate_visit_plan(v[i], v1, v[i + 1], nplan2);
            for (t2, v2) in plan2 {
                let t = t1 + t2;
                if t < fastest_time {
                    fastest = Some((t1, v1, t2, v2));
                    fastest_time = t;
                }
            }
        }
        let (t1, v1, _t2, _v2) = fastest.unwrap();
        let accl = actualize_visit_plan(curpt, curvel, v[i], v1, t1);
        let encoded = encode_thrust_into_keypad(accl);
        buffer += encoded.as_str();
        eprintln!(
            "{i}/{} {curpt:?} {curvel:?} to {:?} {v1:?} lookahead {:?}  res=\"{encoded}\"",
            v.len(),
            v[i],
            v[i + 1]
        );
        curpt = v[i];
        curvel = v1;
    }
    let targetpt = *v.last().unwrap();
    let (accl, vend) = visit_point(curpt, curvel, targetpt);
    let encoded = encode_thrust_into_keypad(accl);
    buffer += encoded.as_str();
    eprintln!(
        "{}/{} {curpt:?} {curvel:?} to {targetpt:?} {vend:?} res=\"{encoded}\"",
        v.len() - 1,
        v.len()
    );
    curpt = *v.last().unwrap();
    curvel = vend;
    println!("{}", buffer);
    eprintln!("Solution length = {}", buffer.len());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = &args[0];
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

    if args[1] == "greedy" {
        solve_greedy(v);
    } else if args[1] == "la1" {
        solve_la1(v);
    } else {
        eprintln!("Usage: (cargo run) greedy < foo.txt");
    }

    //println!("start reorder");
    //let solution: String = solve(v.clone());

    // TODO(chun): run SA
    //println!("{}", solution);
}

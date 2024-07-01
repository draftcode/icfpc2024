use std::cmp;
use std::collections::HashMap;
use std::io::BufWriter;

type Point = (i32, i32);
type Velocity = (i32, i32);
type Acceleration = (i32, i32);

#[allow(dead_code)]
fn surrogate_cost(p1: Point, p2: Point) -> i32 {
    // Chevyshev distance

    return cmp::max((p1.0 - p2.0).abs(), (p1.1 - p2.1).abs());
}

#[allow(dead_code)]
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
#[allow(dead_code)]
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

/*     __A
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

#[allow(dead_code)]
fn generate_pow2_visit_plan(
    curp: Point,
    curv: Velocity,
    targetp: Point,
    numplan: usize,
) -> Vec<(i32, Velocity)> {
    let mut ret = Vec::new();
    let dx = targetp.0 - curp.0;
    let dy = targetp.1 - curp.1;
    let mut tbase: i32 = 1;
    let mut tfirstfound = -1;
    while ret.len() < numplan {
        let t = if tfirstfound == -1 {
            tbase
        } else {
            tfirstfound + 1 << (tbase - tfirstfound)
        };
        tbase += 1;
        let xvr = get_available_vrange(dx, curv.0, t);
        let yvr = get_available_vrange(dy, curv.1, t);
        match (xvr, yvr) {
            (Some(xvr), Some(yvr)) => {
                if tfirstfound == -1 {
                    tfirstfound = tbase
                };
                for vx in (xvr.0..xvr.1 + 1).step_by((tbase - tfirstfound + 1) as usize) {
                    for vy in (yvr.0..yvr.1 + 1).step_by((tbase - tfirstfound + 1) as usize) {
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
    return std::iter::zip(xaccl, yaccl).collect();
}

#[cfg(test)]
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
    let accl = actualize_visit_plan(curpt, curvel, targetpt, vend, t);
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

fn solve_la1(v: Vec<(i32, i32)>, nplan: usize) {
    let mut curpt = (0, 0);
    let mut curvel = (0, 0);
    let mut buffer = String::new();
    let nplan1 = nplan;
    let nplan2 = nplan;
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
    // currently unused
    // curpt = *v.last().unwrap();
    // curvel = vend;
    println!("{}", buffer);
    eprintln!("Solution length = {}", buffer.len());
}

fn solve_la2(v: Vec<(i32, i32)>, nplan: usize) {
    let mut curpt = (0, 0);
    let mut curvel = (0, 0);
    let mut buffer = String::new();
    let nplan1 = nplan * 2;
    let nplan2 = nplan;
    let nplan3 = nplan / 2;
    for i in 0..v.len() - 1 {
        let mut fastest_time = i32::MAX;
        let mut fastest = None;
        let plan1 = generate_visit_plan(curpt, curvel, v[i], nplan1);
        for (t1, v1) in plan1 {
            let plan2 = generate_visit_plan(v[i], v1, v[i + 1], nplan2);
            for (t2, v2) in plan2 {
                let plan3 = if i != v.len() - 2 {
                    generate_visit_plan(v[i + 1], v2, v[i + 2], nplan3)
                } else {
                    vec![(0, (0, 0))]
                };
                for (t3, _v3) in plan3 {
                    let t = t1 + t2 + t3;
                    if t < fastest_time {
                        fastest = Some((t1, v1));
                        fastest_time = t;
                    }
                }
            }
        }
        let (t1, v1) = fastest.unwrap();
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
    // currently unused
    //curpt = *v.last().unwrap();
    //curvel = vend;
    println!("{}", buffer);
    eprintln!("Solution length = {}", buffer.len());
}

fn load_problem(fname: String) -> Vec<(i32, i32)> {
    use std::io::BufRead;
    let file = std::fs::File::open(fname.as_str()).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut v: Vec<(i32, i32)> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut values = line.split_whitespace();
                let a = values.next();
                let b = values.next();
                let (a, b) = match a {
                    Some(x) => (x, b.unwrap()),
                    None => break,
                };
                let ai: i32 = a.parse().unwrap();
                let bi: i32 = b.parse().unwrap();

                v.push((ai, bi));
            }
            Err(_) => {
                break;
            }
        }
    }
    return v;
}

fn load_keypads(fname: String) -> String {
    use std::io::BufRead;
    let file = std::fs::File::open(fname.as_str()).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut ret = String::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                for c in line.chars() {
                    if c.is_numeric() {
                        ret.push(c);
                    }
                }
            }
            Err(_) => {
                break;
            }
        }
    }
    return ret;
}

type Plan = Vec<(Point, Velocity, usize)>;

fn load_plan(plan_fname: String) -> Plan {
    use std::io::BufRead;
    let file = std::fs::File::open(plan_fname.as_str()).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut ret = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut values = line.split_whitespace();
                let Some(px) = values.next() else {
                    break;
                };
                let px: i32 = px.parse::<i32>().unwrap();
                let py: i32 = values.next().unwrap().parse::<i32>().unwrap();
                let vx: i32 = values.next().unwrap().parse::<i32>().unwrap();
                let vy: i32 = values.next().unwrap().parse::<i32>().unwrap();
                let keylen: usize = values.next().unwrap().parse::<usize>().unwrap();
                ret.push(((px, py), (vx, vy), keylen));
            }
            Err(_) => {
                break;
            }
        }
    }
    return ret;
}

fn save_plan(plan_fname: String, plan: &Plan) {
    use std::io::Write;
    let f = std::fs::File::create(plan_fname.as_str()).unwrap();
    let mut f = BufWriter::new(f);
    for (p, v, keylen) in plan {
        writeln!(&mut f, "{} {} {} {} {}", p.0, p.1, v.0, v.1, keylen).expect("write failed");
    }
}

fn make_plan(v: Vec<(i32, i32)>, seq: String, plan_fname: String) {
    let accl = decode_thrust_from_keypad(seq.as_str());
    let traj = simulate((0, 0), (0, 0), &accl);
    let mut visited: Vec<bool> = std::iter::repeat(false).take(v.len()).collect();
    let mut pt2idx: HashMap<(i32, i32), usize> = HashMap::new();
    let mut plan = Vec::new();
    for i in 0..v.len() {
        pt2idx.insert(v[i], i);
    }
    let mut prevptidx = -1;
    for (t, (p, v)) in traj.into_iter().enumerate() {
        if pt2idx.contains_key(&p) {
            let idx = pt2idx.get(&p).unwrap();
            if !visited[*idx] {
                let keylen = (t as i32 - prevptidx) as usize;
                plan.push((p, v, keylen));
                prevptidx = t as i32;
                visited[*idx] = true;
            }
        }
    }
    save_plan(plan_fname, &plan);
}

#[allow(dead_code)]
fn in_range(u: i32, rangeval: (i32, i32)) -> bool {
    return rangeval.0 <= u && u <= rangeval.1;
}

fn local_opt_resovle3pt(
    p1: Point,
    v1: Velocity,
    p2: Point,
    p3: Point,
    v3: Velocity,
    curtotlen: usize,
) -> Option<(Velocity, usize, usize)> {
    let mut ret_time = curtotlen;
    let mut ret = None;
    for t1 in 0..curtotlen {
        let Some(xvr) = get_available_vrange(p2.0 - p1.0, v1.0, t1 as i32) else {
            continue;
        };
        let Some(yvr) = get_available_vrange(p2.1 - p1.1, v1.1, t1 as i32) else {
            continue;
        };
        for vx in xvr.0..xvr.1 + 1 {
            for vy in yvr.0..yvr.1 + 1 {
                for t2 in 0..curtotlen - t1 {
                    if t1 + t2 >= ret_time {
                        break;
                    }
                    let Some(xvr) = get_available_vrange(p3.0 - p2.0, vx, t2 as i32) else {
                        continue;
                    };
                    if !in_range(v3.0, xvr) {
                        continue;
                    }
                    let Some(yvr) = get_available_vrange(p3.1 - p2.1, vy, t2 as i32) else {
                        continue;
                    };
                    if !in_range(v3.1, yvr) {
                        continue;
                    }
                    if t1 + t2 < ret_time {
                        ret_time = t1 + t2;
                        ret = Some(((vx, vy), t1, t2));
                    }
                }
            }
        }
    }
    return ret;
}

fn greedy_local_opt(plan: Plan) -> Plan {
    let mut curp: Point = (0, 0);
    let mut curv: Velocity = (0, 0);
    let mut t1 = plan[0].2;
    let mut t2;
    let mut retplan: Plan = Vec::new();
    for i in 0..plan.len() - 1 {
        t2 = plan[i + 1].2;
        match local_opt_resovle3pt(curp, curv, plan[i].0, plan[i + 1].0, plan[i + 1].1, t1 + t2) {
            Some((newv, newt1, newt2)) => {
                assert!(newt1 + newt2 < t1 + t2);
                retplan.push((plan[i].0, newv, newt1));
                eprintln!("step {i} improved from v={:?} {t1} & v={:?} {t2} -> v={:?} {newt1} & v={:?} {newt2}",
                    plan[i].1, plan[i+1].1, newv, plan[i+1].1);
                t1 = newt2;
                curp = plan[i].0;
                curv = newv;
            }
            None => {
                retplan.push((plan[i].0, plan[i].1, t1));
                t1 = t2;
                curp = plan[i].0;
                curv = plan[i].1;
            }
        }
    }
    let (p, v, _t) = plan.last().unwrap();
    retplan.push((*p, *v, t1));
    return retplan;
}

fn optimize_plan(planfile: String, outplanfile: String) {
    let initialplan = load_plan(planfile);

    let newplan = greedy_local_opt(initialplan);

    println!(
        "Final plan length = {}",
        newplan.iter().fold(0usize, |acc, (_, _, t)| acc + t)
    );
    save_plan(outplanfile, &newplan);
}

fn actuailze_all_plan(planfile: String, outputfile: String) {
    let plan = load_plan(planfile);
    let mut curp = (0, 0);
    let mut curv = (0, 0);
    let mut keys = String::new();
    for (i, (p, v, t)) in plan.iter().enumerate() {
        eprintln!("Step {i}/{} {:?} {:?} {:?}", plan.len(), p, v, t);
        let accl = actualize_visit_plan(curp, curv, *p, *v, *t as i32);
        let encoded = encode_thrust_into_keypad(accl);
        keys += encoded.as_str();
        curp = *p;
        curv = *v;
    }

    use std::io::Write;
    let f = std::fs::File::create(outputfile.as_str()).unwrap();
    let mut f = BufWriter::new(f);
    writeln!(&mut f, "{}", keys).unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = &args[0];
    if args.len() < 3 {
        eprintln!(
            "Usage:
{0} la1 problemfile.txt [look-ahead-size]
{0} la2 problemfile.txt [look-ahead-size]
{0} make_plan problemfile.txt output_seq.txt plan.txt
{0} optimize_plan planfile.txt [niter]
",
            cmd
        );
        return;
    }

    if args[1] == "greedy" {
        if args.len() < 2 {
            panic!("too few args");
        }
        let v = load_problem(args[2].clone());
        solve_greedy(v);
    } else if args[1] == "la1" {
        if args.len() < 3 {
            panic!("too few args");
        }
        let v = load_problem(args[2].clone());
        let mut nplan = 1000;
        if args.len() >= 4 {
            nplan = args[3].parse::<usize>().unwrap();
        }

        solve_la1(v, nplan);
    } else if args[1] == "la2" {
        if args.len() < 3 {
            panic!("too few args");
        }
        let v = load_problem(args[2].clone());
        let mut nplan = 100;
        if args.len() >= 4 {
            nplan = args[3].parse::<usize>().unwrap();
        }
        solve_la2(v, nplan);
    } else if args[1] == "make_plan" {
        let v = load_problem(args[2].clone());
        let seq = load_keypads(args[3].clone());
        let plan_fname = args[4].clone();
        make_plan(v, seq, plan_fname);
    } else if args[1] == "optimize_plan" {
        optimize_plan(args[2].clone(), args[3].clone());
    } else if args[1] == "actualize" {
        actuailze_all_plan(args[2].clone(), args[3].clone());
    }

    //println!("start reorder");
    //let solution: String = solve(v.clone());

    // TODO(chun): run SA
    //println!("{}", solution);
}

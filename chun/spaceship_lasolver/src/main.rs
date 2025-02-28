use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
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

#[derive(Copy, Clone, Eq, PartialEq)]
struct SearchState {
    cost_with_potential: i32,
    passed_mid: bool,
    position: Point,
    velocity: Velocity,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost_with_potential
            .cmp(&self.cost_with_potential)
            .then_with(|| self.passed_mid.cmp(&other.passed_mid))
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.velocity.cmp(&other.velocity))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn follow_backtrack(
    backtrackinfo: HashMap<(Point, Velocity, bool), (Point, Velocity, bool)>,
    pos: Point,
    vel: Velocity,
    midpt: Point,
    backto: Point,
    vbackto: Velocity,
) -> Vec<Acceleration> {
    let mut ret: Vec<Acceleration> = Vec::new();
    let mut pos = pos;
    let mut vel = vel;
    let mut passed_mid = true;
    loop {
        if false {
            eprintln!(
                " backtracking cur = {:?} {:?} {:?}, mid = {:?}, to = {:?} {:?}",
                pos, vel, passed_mid, midpt, backto, vbackto
            );
        }
        if pos == backto && vel == vbackto && !passed_mid {
            ret.reverse();
            return ret;
        }
        let (prevpos, prevvel, prevpassmid) =
            *(backtrackinfo.get(&(pos, vel, passed_mid)).unwrap());
        let prevacc = (vel.0 - prevvel.0, vel.1 - prevvel.1);
        pos = prevpos;
        vel = prevvel;
        passed_mid = prevpassmid;
        ret.push(prevacc);
    }
}

fn calc_additional_estimate(
    curpt: Point,
    curv: Velocity,
    passed_midpt: bool,
    midpt: Point,
    endpt: Point,
) -> i32 {
    if passed_midpt {
        return surrogate_cost_with_velocity(curpt, curv, endpt);
    } else {
        return surrogate_cost_with_velocity(curpt, curv, midpt)
            + surrogate_cost_with_velocity(midpt, curv, endpt);
    }
}

fn solve_lookahead_impl(
    inip: Point,
    iniv: Velocity,
    midpt: Point,
    endpt: Point,
    force_run: bool,
) -> Result<(Vec<Acceleration>, Vec<Acceleration>, Velocity), ()> {
    let mut heap = BinaryHeap::new();
    type State = (Point, Velocity, bool);
    let mut backtrack: HashMap<State, State> = HashMap::new();
    let mut combinedscore: HashMap<State, i32> = HashMap::new();
    let mut truescore: HashMap<State, i32> = HashMap::new();
    let ini_passed_mid = inip == midpt;

    heap.push(SearchState {
        cost_with_potential: 0,
        passed_mid: ini_passed_mid,
        position: inip,
        velocity: iniv,
    });

    truescore.insert((inip, iniv, ini_passed_mid), 0);
    combinedscore.insert(
        (inip, iniv, ini_passed_mid),
        0 + calc_additional_estimate(inip, iniv, ini_passed_mid, midpt, endpt),
    );

    while let Some(SearchState {
        cost_with_potential: _,
        passed_mid,
        position,
        velocity,
    }) = heap.pop()
    {
        let passed_mid = passed_mid || position == midpt;
        if passed_mid && position == endpt {
            let backtrack_res = follow_backtrack(backtrack, position, velocity, midpt, inip, iniv);
            let trajectory = simulate(inip, iniv, &backtrack_res);
            let mut move_before_mid = Vec::new();
            let mut move_after_mid = Vec::new();
            let mut aftermid = false;
            let mut midv = None;
            if inip == midpt {
                aftermid = true;
                midv = Some(iniv);
            }
            for i in 0..trajectory.len() {
                let (curp, curv) = trajectory[i];
                if aftermid {
                    move_after_mid.push(backtrack_res[i]);
                } else {
                    move_before_mid.push(backtrack_res[i]);
                }
                if curp == midpt && !aftermid {
                    aftermid = true;
                    midv = Some(curv);
                }
            }
            return Ok((move_before_mid, move_after_mid, midv.unwrap()));
        }
        let base_truescore = *truescore.get(&(position, velocity, passed_mid)).unwrap();
        //let base_combinedscore = base_truescore + calc_additional_estimate(position, velocity, passed_mid, midpt, endpt);
        for ax in [-1, 0, 1] {
            for ay in [-1, 0, 1] {
                let newv = (velocity.0 + ax, velocity.1 + ay);
                let newx = (position.0 + newv.0, position.1 + newv.1);
                let new_passed_mid = passed_mid || newx == midpt;
                let new_truescore = base_truescore + 1;
                let cur_saved_score = *truescore
                    .get(&(newx, newv, new_passed_mid))
                    .or(Some(&i32::MAX))
                    .unwrap();
                if new_truescore < cur_saved_score {
                    let new_combinedscore = new_truescore
                        + calc_additional_estimate(newx, newv, passed_mid, midpt, endpt);
                    truescore.insert((newx, newv, new_passed_mid), new_truescore);
                    combinedscore.insert((newx, newv, new_passed_mid), new_combinedscore);
                    backtrack.insert(
                        (newx, newv, new_passed_mid),
                        (position, velocity, passed_mid),
                    );

                    heap.push(SearchState {
                        cost_with_potential: new_combinedscore,
                        passed_mid: new_passed_mid,
                        position: newx,
                        velocity: newv,
                    })
                }
            }
        }
        if heap.len() > 1_000_000 && !force_run {
            return Err(());
        }
    }
    panic!("Could not find solution");
}

fn solve_onept(curp: Point, curv: Velocity, endpt: Point) -> Vec<Acceleration> {
    let (va, _ve, _midv) = solve_lookahead_impl(curp, curv, endpt, endpt, true).unwrap();
    return va;
}

fn solve_fallback_singleaxis(px: i32, nx: i32) -> Vec<i32> {
    let s = (nx - px).signum();
    let d = (nx - px).abs();
    if d == 0 {
        return Vec::new();
    }

    let mut k = 1;
    let mut x;
    loop {
        x = if k % 2 != 0 {
            (k + 1) / 2 * (k + 1) / 2
        } else {
            (k / 2) * (k / 2 + 1)
        };
        if d <= x {
            break;
        }
        k += 1;
    }

    let mut p = Vec::new();
    if k % 2 != 0 {
        for _i in 0..((k + 1) / 2) {
            p.push(1);
        }
        for _i in 0..((k + 1) / 2) {
            p.push(-1);
        }
    } else {
        for _i in 0..(k / 2) {
            p.push(1);
        }
        p.push(0);
        for _i in 0..(k / 2) {
            p.push(-1);
        }
    }
    //let offset = if k % 2 != 0 { 0 } else { 1 };
    for i in 0..(x - d) {
        if k % 2 != 0 {
            p[((k + 1) / 2 - 1 - i) as usize] -= 1;
            p[((k + 1) / 2 - i) as usize] += 1;
        } else {
            p[(k / 2 - 1 - i) as usize] -= 1;
            p[(k / 2 - i) as usize] += 1;
        }
    }
    if s < 0 {
        return p.into_iter().map(|x| -x).collect();
    } else {
        return p;
    }
}

fn solve_fallback(curp: Point, endpt: Point) -> Vec<Acceleration> {
    // for very long jump
    let mut ax0plan = solve_fallback_singleaxis(curp.0, endpt.0);
    let mut ax1plan = solve_fallback_singleaxis(curp.1, endpt.1);
    let targetlen = cmp::max(ax0plan.len(), ax1plan.len());
    while ax0plan.len() < targetlen {
        ax0plan.push(0);
    }
    while ax1plan.len() < targetlen {
        ax1plan.push(0);
    }
    return std::iter::zip(ax0plan, ax1plan).collect();
}

fn delta(p2: Point, p1: Point) -> Point {
    return (p2.0 - p1.0, p2.1 - p1.1);
}

fn solve_lookahead(
    inip: Point,
    iniv: Velocity,
    midpt: Point,
    endpt: Point,
    memo: &mut HashMap<(Point, Point, Velocity), (Vec<Acceleration>, Vec<Acceleration>, Velocity)>,
) -> (Vec<Acceleration>, Vec<Acceleration>, Velocity) {
    let memo_key = (delta(midpt, inip), delta(endpt, inip), iniv);
    let maybe_memo = memo.get(&memo_key);
    match maybe_memo {
        Some(x) => {
            return x.clone();
        }
        None => {
            let res = solve_lookahead_impl(inip, iniv, midpt, endpt, false);
            match res {
                Ok(x) => {
                    memo.insert(memo_key, x.clone());
                    return x;
                }
                Err(_) => {
                    eprintln!("Exceeded search limit, shrinking the search space...");
                    let x = solve_lookahead_impl(inip, iniv, midpt, midpt, true).unwrap();
                    memo.insert(memo_key, x.clone());
                    return x;
                }
            }
        }
    }
}

fn solve(points: Vec<(i32, i32)>) -> String {
    let mut retbuf = String::new();
    let mut curpt: Point = (0, 0);
    let mut curv: Velocity = (0, 0);
    let mut memo: HashMap<
        (Point, Point, Velocity),
        (Vec<Acceleration>, Vec<Acceleration>, Velocity),
    > = HashMap::new();
    for i in 0..points.len() - 1 {
        let nextmid = points[i];
        let nextend = points[i + 1];
        if nextmid == nextend {
            continue;
        }
        eprintln!(
            "Solving {:?}, {:?} to {:?}, looking ahead: {:?}",
            curpt, curv, nextmid, nextend
        );
        if surrogate_cost(curpt, nextmid) > 1000 {
            // to heavy to search, falling back
            // stop particle
            while curv != (0, 0) {
                let acc = (-curv.0.signum(), -curv.1.signum());
                retbuf.push_str(&encode_thrust_into_keypad(vec![acc]));
                curv = (curv.0 + acc.0, curv.1 + acc.1);
                curpt = (curpt.0 + curv.0, curpt.1 + curv.1);
            }
            let accs = solve_fallback(curpt, nextmid);
            let encoded = encode_thrust_into_keypad(accs.clone());
            eprintln!(
                "Solved step {}/{} by jumping to {:?}, \"{}\"",
                i,
                points.len(),
                nextmid,
                encoded
            );
            retbuf.push_str(encoded.as_str());
            let check_by_simulate = simulate(curpt, curv, &accs);
            assert_eq!(check_by_simulate.last().unwrap().0, nextmid);
            assert_eq!(check_by_simulate.last().unwrap().1, (0, 0));
            curv = (0, 0);
            curpt = nextmid;
            continue;
        }
        let (accs, _ve, midv) = solve_lookahead(curpt, curv, nextmid, nextend, &mut memo);
        {
            let check_by_simulate = simulate(curpt, curv, &accs);
            if curpt != nextmid {
                assert_eq!(check_by_simulate.last().unwrap().0, nextmid);
                assert_eq!(check_by_simulate.last().unwrap().1, midv);
            }
        }
        let encoded = encode_thrust_into_keypad(accs);
        eprintln!(
            "Solved step {}/{}, arrived at {:?}, {:?}, \"{}\"",
            i,
            points.len(),
            nextmid,
            midv,
            encoded
        );
        retbuf.push_str(encoded.as_str());
        curpt = nextmid;
        curv = midv;
    }
    let finalpt = points[points.len() - 1];
    eprintln!(
        "Solving {:?}, {:?} to {:?} (final pt)",
        curpt, curv, finalpt
    );
    let accs = if surrogate_cost(curpt, finalpt) > 1000 {
        // to heavy to search, falling back
        // stop particle
        while curv != (0, 0) {
            let acc = (-curv.0.signum(), -curv.1.signum());
            retbuf.push_str(&encode_thrust_into_keypad(vec![acc]));
            curv = (curv.0 + acc.0, curv.1 + acc.1);
            curpt = (curpt.0 + curv.0, curpt.1 + curv.1);
        }
        solve_fallback(curpt, finalpt)
    } else {
        solve_onept(curpt, curv, points[points.len() - 1])
    };
    eprintln!("accs returned, len={}", accs.len());
    let vel = {
        let check_by_simulate = simulate(curpt, curv, &accs);
        check_by_simulate.last().unwrap().1
    };
    let encoded = encode_thrust_into_keypad(accs);
    eprintln!(
        "Solved final step, arrived at {:?}, {:?}, \"{}\"",
        points.last().unwrap(),
        vel,
        encoded
    );
    retbuf.push_str(encoded.as_str());
    return retbuf;
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
    let solution: String = solve(v.clone());

    // TODO(chun): run SA
    println!("{}", solution);
}

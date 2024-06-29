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
    let avx = if dx.signum() == vx.signum() {
        cmp::max(1, vx.abs())
    } else {
        1
    };
    let vy = curv.1;
    let avy = if dy.signum() == vy.signum() {
        cmp::max(1, vy.abs())
    } else {
        1
    };
    let adx = dx.abs();
    let ady = dy.abs();

    return cmp::max(adx / avx, ady / avy);
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
    backtrackinfo: HashMap<(Point, Velocity), Acceleration>,
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
        eprintln!(
            " backtracking cur = {:?} {:?}, mid = {:?}, to = {:?} {:?}",
            pos, vel, midpt, backto, vbackto
        );
        if pos == backto && vel == vbackto {
            ret.reverse();
            return ret;
        }
        let prevacc = *(backtrackinfo.get(&(pos, vel)).unwrap());
        if pos == midpt {
            passed_mid = false;
        }
        let prevpos = (pos.0 - vel.0, pos.1 - vel.1);
        let prevvel = (vel.0 - prevacc.0, vel.1 - prevacc.1);
        pos = prevpos;
        vel = prevvel;
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
    let mut backtrack: HashMap<(Point, Velocity), Acceleration> = HashMap::new();
    let mut combinedscore: HashMap<(Point, Velocity), i32> = HashMap::new();
    let mut truescore: HashMap<(Point, Velocity), i32> = HashMap::new();

    heap.push(SearchState {
        cost_with_potential: 0,
        passed_mid: false,
        position: inip,
        velocity: iniv,
    });

    truescore.insert((inip, iniv), 0);
    combinedscore.insert(
        (inip, iniv),
        0 + calc_additional_estimate(inip, iniv, inip == midpt, midpt, endpt),
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
        let base_truescore = *truescore.get(&(position, velocity)).unwrap();
        //let base_combinedscore = base_truescore + calc_additional_estimate(position, velocity, passed_mid, midpt, endpt);
        for ax in [-1, 0, 1] {
            for ay in [-1, 0, 1] {
                let newv = (velocity.0 + ax, velocity.1 + ay);
                let newx = (position.0 + newv.0, position.1 + newv.1);
                let new_passed_mid = passed_mid || newx == midpt;
                let new_truescore = base_truescore + 1;
                let cur_saved_score = *truescore.get(&(newx, newv)).or(Some(&i32::MAX)).unwrap();
                if new_truescore < cur_saved_score {
                    let new_combinedscore = new_truescore
                        + calc_additional_estimate(newx, newv, passed_mid, midpt, endpt);
                    truescore.insert((newx, newv), new_truescore);
                    combinedscore.insert((newx, newv), new_combinedscore);
                    backtrack.insert((newx, newv), (ax, ay));

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

fn solve_lookahead(
    inip: Point,
    iniv: Velocity,
    midpt: Point,
    endpt: Point,
) -> (Vec<Acceleration>, Vec<Acceleration>, Velocity) {
    let res = solve_lookahead_impl(inip, iniv, midpt, endpt, false);
    match res {
        Ok(x) => return x,
        Err(_) => {
            eprintln!("Exceeded search limit, shrinking the search space...");
            return solve_lookahead_impl(inip, iniv, midpt, midpt, true).unwrap();
        }
    }
}

fn solve(points: Vec<(i32, i32)>) -> String {
    let mut retbuf = String::new();
    let mut curpt: Point = (0, 0);
    let mut curv: Velocity = (0, 0);
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
        let (accs, _ve, midv) = solve_lookahead(curpt, curv, nextmid, nextend);
        {
            let check_by_simulate = simulate(curpt, curv, &accs);
            if curpt != nextmid {
                assert_eq!(check_by_simulate.last().unwrap().0, nextmid);
                assert_eq!(check_by_simulate.last().unwrap().1, midv);
            }
        }
        retbuf.push_str(encode_thrust_into_keypad(accs).as_str());
        curpt = nextmid;
        curv = midv;
    }
    let accs = solve_onept(curpt, curv, points[points.len() - 1]);
    retbuf.push_str(encode_thrust_into_keypad(accs).as_str());
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

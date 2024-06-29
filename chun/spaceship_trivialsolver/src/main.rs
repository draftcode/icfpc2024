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
    let (avx, xpenalty) = if dx.signum() == vx.signum() {
        (cmp::max(1, vx.abs()), 0)
    } else {
        (1, vx.abs() * vx.abs() + 1)
    };
    let vy = curv.1;
    let (avy, ypenalty) = if dy.signum() == vy.signum() {
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
    position: Point,
    velocity: Velocity,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost_with_potential
            .cmp(&self.cost_with_potential)
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
    backtrackinfo: HashMap<(Point, Velocity), (Point, Velocity)>,
    pos: Point,
    vel: Velocity,
    backto: Point,
    vbackto: Velocity,
) -> Vec<Acceleration> {
    let mut ret: Vec<Acceleration> = Vec::new();
    let mut pos = pos;
    let mut vel = vel;
    loop {
        if false {
            eprintln!(
                " backtracking cur = {:?} {:?}, to = {:?} {:?}",
                pos, vel, backto, vbackto
            );
        }
        if pos == backto && vel == vbackto {
            ret.reverse();
            return ret;
        }
        let (prevpos, prevvel) = *(backtrackinfo.get(&(pos, vel)).unwrap());
        let prevacc = (vel.0 - prevvel.0, vel.1 - prevvel.1);
        pos = prevpos;
        vel = prevvel;
        ret.push(prevacc);
    }
}

fn velloss(v1: Velocity, v2: Velocity) -> i32 {
    return cmp::max((v1.0 - v2.0).abs(), (v1.1 - v2.1).abs());
}

fn calc_additional_estimate(curpt: Point, curv: Velocity, endpt: Point) -> i32 {
    return surrogate_cost_with_velocity(curpt, curv, endpt);
}

fn solve_lookahead(
    inip: Point,
    iniv: Velocity,
    endpt: Point,
    endtargvel: Velocity,
) -> (Vec<Acceleration>, Velocity) {
    let mut heap = BinaryHeap::new();
    type State = (Point, Velocity);
    let mut backtrack: HashMap<State, State> = HashMap::new();
    let mut combinedscore: HashMap<State, i32> = HashMap::new();
    let mut truescore: HashMap<State, i32> = HashMap::new();

    heap.push(SearchState {
        cost_with_potential: 0,
        position: inip,
        velocity: iniv,
    });

    let iniloss = velloss(iniv, endtargvel);
    truescore.insert((inip, iniv), iniloss);
    combinedscore.insert(
        (inip, iniv),
        iniloss + calc_additional_estimate(inip, iniv, endpt),
    );

    let mut bestans = None;
    let mut bestscore = i32::MAX;
    let mut bestvel = None;
    while let Some(SearchState {
        cost_with_potential: _,
        position,
        velocity,
    }) = heap.pop()
    {
        let base_truescore = *truescore.get(&(position, velocity)).unwrap();
        let truescore_val = base_truescore + velloss(endtargvel, velocity);
        if position == endpt {
            if truescore_val < bestscore {
                bestans = Some((position, velocity));
                bestscore = truescore_val;
                bestvel = Some(velocity);
            }
            break;
        }
        //let base_combinedscore = base_truescore + calc_additional_estimate(position, velocity, passed_mid, midpt, endpt);
        for ax in [-1, 0, 1] {
            for ay in [-1, 0, 1] {
                let newv = (velocity.0 + ax, velocity.1 + ay);
                let newx = (position.0 + newv.0, position.1 + newv.1);
                let new_truescore = base_truescore + 1 + velloss(endtargvel, newv);
                let cur_saved_score = *truescore.get(&(newx, newv)).or(Some(&i32::MAX)).unwrap();
                if new_truescore < cur_saved_score {
                    let new_combinedscore =
                        new_truescore + calc_additional_estimate(newx, newv, endpt);
                    truescore.insert((newx, newv), new_truescore);
                    combinedscore.insert((newx, newv), new_combinedscore);
                    backtrack.insert((newx, newv), (position, velocity));

                    heap.push(SearchState {
                        cost_with_potential: new_combinedscore,
                        position: newx,
                        velocity: newv,
                    })
                }
            }
        }
    }

    let bestans = bestans.unwrap();
    let backtrack_res = follow_backtrack(backtrack, bestans.0, bestans.1, inip, iniv);
    //let trajectory = simulate(inip, iniv, &backtrack_res);
    return (backtrack_res, bestvel.unwrap());
}

fn solve_fallback_singleaxis(px: i32, nx: i32) -> Vec<i32> {
    let s = (nx - px).signum();
    let d = (nx - px).abs();
    if d == 0 {
        return Vec::new();
    }

    let mut k = 1;
    let mut x = 0;
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
    let offset = if k % 2 != 0 { 0 } else { 1 };
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
        let target_vel = {
            fn signedroot(x: i32) -> i32 {
                x.signum() * ((x.abs() as f64).sqrt() as i32)
            }
            (
                signedroot(nextend.0 - nextmid.0),
                signedroot(nextend.1 - nextmid.0),
            )
        };
        let (accs, midv) = solve_lookahead(curpt, curv, nextmid, target_vel);
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
    let (accs, _) = solve_lookahead(curpt, curv, points[points.len() - 1], (0, 0));
    let encoded = encode_thrust_into_keypad(accs);
    eprintln!("Solved final step, \"{}\"", encoded);
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

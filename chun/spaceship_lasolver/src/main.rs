use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::{cmp, io};

type Point = (i32, i32);
type Velocity = (i32, i32);
type Acceleration = (i32, i32);

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

fn simulate(inip: Point, iniv: Velocity, order: Vec<Acceleration>) -> Vec<(Point, Velocity)> {
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
        let res = simulate((0, 0), (0, 0), thrusts);
        let posres: Vec<Point> = res.into_iter().map(|x| x.0).collect();
        assert_eq!(posres, expected_points);
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct SearchState {
    cost: i32,
    cost_with_potential: usize,
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
    backtrackinfo: HashMap<(Point, Velocity), (Acceleration, i32)>,
    pos: Point,
    vel: Velocity,
    backto: Point,
    vbackto: Velocity,
) -> Vec<Acceleration> {
    let mut ret: Vec<Acceleration> = Vec::new();
    let mut pos = pos;
    let mut vel = vel;
    loop {
        if pos == backto && vel == vbackto {
            ret.reverse();
            return ret;
        }
        let prevacc = backtrackinfo.get(&(pos, vel)).unwrap().0;
        let prevpos = (pos.0 - vel.0, pos.1 - vel.1);
        let prevvel = (vel.0 - prevacc.0, vel.1 - prevacc.1);
        pos = prevpos;
        vel = prevvel;
        ret.push(prevacc);
    }
}

fn solve_lookahead(
    inip: Point,
    iniv: Velocity,
    midpt: Point,
    endpt: Point,
) -> (Vec<Acceleration>, Velocity) {
    let midvel: Velocity = (0, 0);
    let mut heap = BinaryHeap::new();
    let mut backtrack: HashMap<(Point, Velocity), (Acceleration, i32)> = HashMap::new();

    heap.push(SearchState {
        cost: 0,
        cost_with_potential: 0,
        passed_mid: false,
        position: inip,
        velocity: iniv,
    });

    while let Some(SearchState {
        cost,
        cost_with_potential,
        passed_mid,
        position,
        velocity,
    }) = heap.pop()
    {
        let mut passed_mid = passed_mid;
        if passed_mid && position == endpt {

            /*
            return get_backtrack_and_midvec(backgrack, position, velocity, inip, iniv)
            return follow_backtrack(backtrack, position, velocity);
            */
        }
    }
    return (Vec::new(), midvel);
}

fn solve_onept(curp: Point, curv: Velocity, endpt: Point) -> Vec<Acceleration> {
    return Vec::new();
}

fn solve(points: Vec<(i32, i32)>) -> String {
    let mut retbuf = String::new();
    let mut curpt: (i32, i32) = (0, 0);
    for i in 0..points.len() - 1 {}
    return retbuf;
}

fn main() {
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
}

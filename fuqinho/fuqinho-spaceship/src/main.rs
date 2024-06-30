use anyhow::Result;
use std::cmp::max;
use std::collections::VecDeque;
use std::io::stdin;

const MIN_V: i32 = -40;
const MAX_V: i32 = 40;
const NUM_V: usize = (MAX_V - MIN_V + 1) as usize;
const MIN_D: i32 = -4000;
const MAX_D: i32 = 4000;
const NUM_D: usize = (MAX_D - MIN_D + 1) as usize;
const INF: i32 = 1e9 as i32;
const BEAM_WIDTH: usize = 100;

struct Problem {
    v: Vec<(i32, i32)>,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum CoolingSchedule {
    Linear,
    Quadratic,
    Exponential,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum AcceptFunction {
    Linear,
    Exponential,
}

#[derive(Debug, Clone)]
struct BeamState {
    num_steps: i32,
    vx: i32,
    vy: i32,
    prev_index: usize,
}
impl PartialEq for BeamState {
    fn eq(&self, other: &Self) -> bool {
        self.num_steps == other.num_steps && self.vx == other.vx && self.vy == other.vy
    }
}
impl Eq for BeamState {}

#[derive(Debug, Clone)]
struct MoveInfo {
    steps: i32,
    terminal_velocity: i32,
}

fn main() -> Result<()> {
    // Read problem text from stdin.
    let problem = read_problem()?;

    // Solve the problem.
    let solution = solve(&problem);
    println!("{}", solution);

    Ok(())
}

fn solve(problem: &Problem) -> String {
    // Solve the problem.

    // Precalculate minimum steps.
    // min_steps[v0][v][pos] = minimum steps to move from 0 to pos with initial velocity v0 and terminal velocity v.
    let mut min_steps = vec![vec![vec![INF; NUM_D]; NUM_V]; NUM_V];
    let mut q = VecDeque::new();
    for v in MIN_V..=MAX_V {
        min_steps[(v - MIN_V) as usize][(v - MIN_V) as usize][(0 - MIN_D) as usize] = 0;
        q.push_back((v, v, 0, 0));
    }
    while let Some((v0, v, pos, steps)) = q.pop_front() {
        if min_steps[(v0 - MIN_V) as usize][(v - MIN_V) as usize][(pos - MIN_D) as usize] < steps {
            continue;
        }
        for a in -1..=1 {
            let nv = v + a;
            let npos = pos + nv;
            if nv < MIN_V || nv > MAX_V {
                continue;
            }
            if npos < MIN_D || npos > MAX_D {
                continue;
            }
            if min_steps[(v0 - MIN_V) as usize][(nv - MIN_V) as usize][(npos - MIN_D) as usize]
                <= steps + 1
            {
                continue;
            }

            min_steps[(v0 - MIN_V) as usize][(nv - MIN_V) as usize][(npos - MIN_D) as usize] =
                steps + 1;
            q.push_back((v0, nv, npos, steps + 1));
        }
    }

    // Beam search
    let mut x = 0;
    let mut y = 0;
    let mut beams = vec![vec![]; problem.v.len() + 1];
    beams[0].push(BeamState {
        num_steps: 0,
        vx: 0,
        vy: 0,
        prev_index: 0,
    });
    for i in 0..problem.v.len() {
        let dx = problem.v[i].0 - x;
        let dy = problem.v[i].1 - y;
        for j in 0..beams[i].len() {
            let BeamState {
                num_steps,
                vx,
                vy,
                prev_index: _prev_index,
            } = beams[i][j];
            let moves_x = get_moves(vx, dx, 10, &min_steps);
            let moves_y = get_moves(vy, dy, 10, &min_steps);
            if moves_x.is_empty() || moves_y.is_empty() {
                continue;
            }

            let lb_steps = max(moves_x[0].steps, moves_y[0].steps);
            for s in lb_steps..=(lb_steps + 10) {
                let moves_x = get_moves_with_steps(vx, dx, s, 10, &min_steps);
                let moves_y = get_moves_with_steps(vy, dy, s, 10, &min_steps);
                if moves_x.is_empty() || moves_y.is_empty() {
                    continue;
                }
                for k in 0..moves_x.len() {
                    for l in 0..moves_y.len() {
                        beams[i + 1].push(BeamState {
                            num_steps: num_steps + s,
                            vx: moves_x[k].terminal_velocity,
                            vy: moves_y[l].terminal_velocity,
                            prev_index: j,
                        });
                    }
                }
            }
        }
        beams[i + 1].sort_by_key(|x| (x.num_steps, x.vx.abs() + x.vy.abs(), x.vx, x.vy));
        beams[i + 1].dedup();
        beams[i + 1].truncate(BEAM_WIDTH);

        x = problem.v[i].0;
        y = problem.v[i].1;
    }

    eprintln!("Minimum steps: {}", beams[problem.v.len()][0].num_steps);

    // Reconstruct the steps.
    let mut vxs = vec![];
    let mut vys = vec![];
    let mut steps_history = vec![];
    let mut beam_idx = 0;
    for i in (1..=problem.v.len()).rev() {
        let BeamState {
            num_steps,
            vx,
            vy,
            prev_index,
        } = beams[i][beam_idx];
        vxs.push(vx);
        vys.push(vy);
        steps_history.push(num_steps - beams[i - 1][prev_index].num_steps);
        beam_idx = prev_index;
    }
    vxs.push(0);
    vys.push(0);
    vxs.reverse();
    vys.reverse();
    steps_history.reverse();

    let mut ans = String::new();
    let mut cx = 0;
    let mut cy = 0;
    let mut cvx = 0;
    let mut cvy = 0;
    for i in 0..problem.v.len() {
        let nx = problem.v[i].0;
        let ny = problem.v[i].1;
        let nvx = vxs[i + 1];
        let nvy = vys[i + 1];
        let steps_x = reconstruct_steps(nx - cx, cvx, nvx, steps_history[i], &min_steps);
        let steps_y = reconstruct_steps(ny - cy, cvy, nvy, steps_history[i], &min_steps);
        for j in 0..steps_x.len() {
            if steps_x[j] == -1 && steps_y[j] == -1 {
                ans.push_str("1");
            } else if steps_x[j] == -1 && steps_y[j] == 0 {
                ans.push_str("4");
            } else if steps_x[j] == -1 && steps_y[j] == 1 {
                ans.push_str("7");
            } else if steps_x[j] == 0 && steps_y[j] == -1 {
                ans.push_str("2");
            } else if steps_x[j] == 0 && steps_y[j] == 0 {
                ans.push_str("5");
            } else if steps_x[j] == 0 && steps_y[j] == 1 {
                ans.push_str("8");
            } else if steps_x[j] == 1 && steps_y[j] == -1 {
                ans.push_str("3");
            } else if steps_x[j] == 1 && steps_y[j] == 0 {
                ans.push_str("6");
            } else if steps_x[j] == 1 && steps_y[j] == 1 {
                ans.push_str("9");
            } else {
                unreachable!();
            }
        }
        cx = nx;
        cy = ny;
        cvx = nvx;
        cvy = nvy;
    }
    ans
}

fn get_moves(v: i32, dx: i32, max_results: usize, min_steps: &Vec<Vec<Vec<i32>>>) -> Vec<MoveInfo> {
    let mut results = vec![];
    for ve in MIN_V..=MAX_V {
        let steps = min_steps[(v - MIN_V) as usize][(ve - MIN_V) as usize][(dx - MIN_D) as usize];
        if steps == INF {
            continue;
        }
        results.push(MoveInfo {
            steps,
            terminal_velocity: ve,
        });
    }
    results.sort_by_key(|x| (x.steps, x.terminal_velocity.abs()));
    results.truncate(max_results);
    results
}

fn get_moves_with_steps(
    v: i32,
    dx: i32,
    steps: i32,
    max_results: usize,
    min_steps: &Vec<Vec<Vec<i32>>>,
) -> Vec<MoveInfo> {
    let mut results = vec![];
    for ve in MIN_V..=MAX_V {
        let steps1 = min_steps[(v - MIN_V) as usize][(ve - MIN_V) as usize][(dx - MIN_D) as usize];
        if steps1 != steps {
            continue;
        }
        results.push(MoveInfo {
            steps: steps1,
            terminal_velocity: ve,
        });
    }
    if steps >= 1 {
        for a in -1..=1 {
            let nv = v + a;
            let ndx = dx - nv;
            if nv < MIN_V || nv > MAX_V || ndx < MIN_D || ndx > MAX_D {
                continue;
            }
            for ve in MIN_V..=MAX_V {
                let steps1 =
                    min_steps[(nv - MIN_V) as usize][(ve - MIN_V) as usize][(ndx - MIN_D) as usize];
                if steps1 == steps - 1 {
                    results.push(MoveInfo {
                        steps,
                        terminal_velocity: ve,
                    });
                }
            }
        }
    }
    if steps >= 2 {
        for a0 in -1..=1 {
            for a1 in -1..=1 {
                let nv0 = v + a0;
                let ndx0 = dx - nv0;
                let nv1 = nv0 + a1;
                let ndx1 = ndx0 - nv1;
                if nv1 < MIN_V || nv1 > MAX_V || ndx1 < MIN_D || ndx1 > MAX_D {
                    continue;
                }
                for ve in MIN_V..=MAX_V {
                    let steps1 = min_steps[(nv1 - MIN_V) as usize][(ve - MIN_V) as usize]
                        [(ndx1 - MIN_D) as usize];
                    if steps1 == steps - 2 {
                        results.push(MoveInfo {
                            steps,
                            terminal_velocity: ve,
                        });
                    }
                }
            }
        }
    }

    results.sort_by_key(|x| x.terminal_velocity.abs());
    results.truncate(max_results);
    results
}

fn can_reach(vs: i32, ve: i32, dx: i32, steps: i32, min_steps: &Vec<Vec<Vec<i32>>>) -> bool {
    if min_steps[(vs - MIN_V) as usize][(ve - MIN_V) as usize][(dx - MIN_D) as usize] == steps {
        return true;
    }
    if steps >= 1 {
        for a in -1..=1 {
            let nv = vs + a;
            let ndx = dx - nv;
            if nv < MIN_V || nv > MAX_V || ndx < MIN_D || ndx > MAX_D {
                continue;
            }
            if min_steps[(nv - MIN_V) as usize][(ve - MIN_V) as usize][(ndx - MIN_D) as usize]
                == steps - 1
            {
                return true;
            }
        }
    }
    if steps >= 2 {
        for a0 in -1..=1 {
            for a1 in -1..=1 {
                let nv0 = vs + a0;
                let ndx0 = dx - nv0;
                let nv1 = nv0 + a1;
                let ndx1 = ndx0 - nv1;
                if nv0 < MIN_V
                    || nv0 > MAX_V
                    || nv1 < MIN_V
                    || nv1 > MAX_V
                    || ndx0 < MIN_D
                    || ndx0 > MAX_D
                    || ndx1 < MIN_D
                    || ndx1 > MAX_D
                {
                    continue;
                }
                if min_steps[(nv1 - MIN_V) as usize][(ve - MIN_V) as usize][(ndx1 - MIN_D) as usize]
                    == steps - 2
                {
                    return true;
                }
            }
        }
    }

    false
}

fn reconstruct_steps(
    dx: i32,
    vs: i32,
    ve: i32,
    steps: i32,
    min_steps: &Vec<Vec<Vec<i32>>>,
) -> Vec<i32> {
    let mut results = vec![];
    let mut v = vs;
    let mut x = dx;
    for i in 0..steps {
        for a in -1..=1 {
            let nv = v + a;
            let nx = x - nv;
            if nv < MIN_V || nv > MAX_V || nx < MIN_D || nx > MAX_D {
                continue;
            }
            if can_reach(nv, ve, nx, steps - i - 1, min_steps) {
                v = nv;
                x = nx;
                results.push(a);
                break;
            }
        }
    }
    results
}

fn read_problem() -> Result<Problem> {
    let mut buffer = String::new();
    let mut v = vec![];

    // Read problem text from stdin.
    loop {
        match stdin().read_line(&mut buffer) {
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

    Ok(Problem { v })
}

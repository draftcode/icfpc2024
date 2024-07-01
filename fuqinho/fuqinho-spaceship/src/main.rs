use anyhow::Result;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::cmp::max;
use std::collections::VecDeque;
use std::io::stdin;
use std::path::PathBuf;

const MIN_V: i32 = -100;
const MAX_V: i32 = 100;
const NUM_V: usize = (MAX_V - MIN_V + 1) as usize;
const MIN_D: i32 = -10000;
const MAX_D: i32 = 10000;
const NUM_D: usize = (MAX_D - MIN_D + 1) as usize;
const INF: i32 = 1e9 as i32;
const GET_MOVES_MAX_RESULTS: usize = 10;
const BEAM_WIDTH: usize = 1000;

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

pub struct SAConfig {
    pub num_iterations: usize,
    pub initial_temperature: f64,
    pub final_temperature: f64,
    pub solutions_dir: PathBuf,
    pub cooling_schedule: CoolingSchedule,
    pub accept_function: AcceptFunction,
}

fn current_temperature(progress: f64, config: &SAConfig) -> f64 {
    match config.cooling_schedule {
        CoolingSchedule::Linear => (1. - progress) * config.initial_temperature,
        CoolingSchedule::Quadratic => (1. - progress).powi(2) * config.initial_temperature,
        CoolingSchedule::Exponential => {
            config.initial_temperature.powf(1. - progress) * config.final_temperature.powf(progress)
        }
    }
}

fn should_accept(
    cur_score: f64,
    next_score: f64,
    temperature: f64,
    rng: &mut ThreadRng,
    config: &SAConfig,
) -> bool {
    if next_score >= cur_score {
        return true;
    }
    match config.accept_function {
        AcceptFunction::Linear => rng.gen_range(0.0..1.0) * temperature > -(next_score - cur_score),
        AcceptFunction::Exponential => {
            rng.gen_bool(f64::exp((next_score - cur_score) / (temperature + 1e-9)))
        }
    }
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
impl PartialEq for MoveInfo {
    fn eq(&self, other: &Self) -> bool {
        self.steps == other.steps && self.terminal_velocity == other.terminal_velocity
    }
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
    // min_steps[v0][v][pos] = minimum steps to move from 0 to pos with initial velocity v0 and terminal velocity v.
    let min_steps = precompute_min_steps();

    let mut checkpoints = problem.v.clone();

    // TODO: Run solve_one in multi threads for trying multiple configurations.

    reorder_checkpoints(&mut checkpoints);

    // Solve the problem on the give visiting order |checkpoints|.
    let mut best_answer = solve_one(&checkpoints, &min_steps);

    eprintln!("Best score: {}", best_answer.len());

    best_answer
}

fn reorder_checkpoints(checkpoints: &mut Vec<(i32, i32)>) {
    let config = SAConfig {
        num_iterations: 500000000,
        initial_temperature: 500.0,
        final_temperature: 1.0,
        solutions_dir: PathBuf::from("results"),
        cooling_schedule: CoolingSchedule::Quadratic,
        accept_function: AcceptFunction::Linear,
    };

    let mut iteration = 1;
    let mut iteration_percent = 0;
    let mut rng = rand::thread_rng();
    let mut best_score = checkpoints_score(&checkpoints);
    loop {
        iteration += 1;
        let temperature =
            current_temperature(iteration as f64 / config.num_iterations as f64, &config);

        // swap random two checkpoints
        let c1 = rng.gen_range(0..checkpoints.len());
        let c2 = rng.gen_range(0..checkpoints.len());

        //checkpoints.swap(c1, c2);
        let removed = checkpoints.remove(c1);
        checkpoints.insert(c2, removed);
        //        let insert_pos = if c2 < c1 { c2 } else { c2 - 1 };
        //        checkpoints.insert(insert_pos, removed);

        let score = checkpoints_score(&checkpoints);
        if should_accept(best_score, score, temperature, &mut rng, &config) {
            //eprintln!("Update best score: {} -> {}", best_score, score);
            best_score = score;
        } else {
            //checkpoints.swap(c1, c2);
            let removed2 = checkpoints.remove(c2);
            //let insert_pos2 = if c1 < c2 { c1 } else { c1 - 1 };
            checkpoints.insert(c1, removed2);
        }

        let current_iteration_percent =
            (iteration as f64 / config.num_iterations as f64 * 100.0) as i32;
        if current_iteration_percent != iteration_percent {
            eprintln!(
                "Iteration: {} ({}%), Temperature: {}, Best score: {}",
                iteration, current_iteration_percent, temperature, best_score
            );
            iteration_percent = current_iteration_percent;
        }
        if iteration >= config.num_iterations {
            break;
        }
    }
}

fn checkpoints_score(checkpoints: &Vec<(i32, i32)>) -> f64 {
    let mut score = 0.0;
    let mut x = 0;
    let mut y = 0;
    for i in 0..checkpoints.len() {
        let nx = checkpoints[i].0;
        let ny = checkpoints[i].1;
        let dx = (nx - x) as f64;
        let dy = (ny - y) as f64;
        let length = (dx * dx + dy * dy).sqrt();
        score -= length;

        x = nx;
        y = ny;
    }
    score
}

fn solve_one(checkpoints: &Vec<(i32, i32)>, min_steps: &Vec<Vec<Vec<i32>>>) -> String {
    // Beam search
    let mut x = 0;
    let mut y = 0;
    let mut beams = vec![vec![]; checkpoints.len() + 1];
    beams[0].push(BeamState {
        num_steps: 0,
        vx: 0,
        vy: 0,
        prev_index: 0,
    });
    let mut current_progress_percent = 0;
    for i in 0..checkpoints.len() {
        let dx = checkpoints[i].0 - x;
        let dy = checkpoints[i].1 - y;
        for j in 0..beams[i].len() {
            let BeamState {
                num_steps,
                vx,
                vy,
                prev_index: _prev_index,
            } = beams[i][j];
            let moves_x = get_moves(vx, dx, GET_MOVES_MAX_RESULTS, &min_steps);
            let moves_y = get_moves(vy, dy, GET_MOVES_MAX_RESULTS, &min_steps);
            if moves_x.is_empty() || moves_y.is_empty() {
                continue;
            }

            let lb_steps = max(moves_x[0].steps, moves_y[0].steps);
            for s in lb_steps..=(lb_steps + 10) {
                let moves_x = get_moves_with_steps(vx, dx, s, GET_MOVES_MAX_RESULTS, &min_steps);
                let moves_y = get_moves_with_steps(vy, dy, s, GET_MOVES_MAX_RESULTS, &min_steps);
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
        if beams[i + 1].is_empty() {
            eprintln!("Expand s and re-search beam.");
            for j in 0..beams[i].len() {
                let BeamState {
                    num_steps,
                    vx,
                    vy,
                    prev_index: _prev_index,
                } = beams[i][j];
                let moves_x = get_moves(vx, dx, GET_MOVES_MAX_RESULTS * 2, &min_steps);
                let moves_y = get_moves(vy, dy, GET_MOVES_MAX_RESULTS * 2, &min_steps);
                if moves_x.is_empty() || moves_y.is_empty() {
                    continue;
                }

                let lb_steps = max(moves_x[0].steps, moves_y[0].steps);
                for s in lb_steps..=(lb_steps + 50) {
                    let moves_x =
                        get_moves_with_steps(vx, dx, s, GET_MOVES_MAX_RESULTS * 2, &min_steps);
                    let moves_y =
                        get_moves_with_steps(vy, dy, s, GET_MOVES_MAX_RESULTS * 2, &min_steps);
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
        }
        beams[i + 1].sort_by_key(|x| (x.num_steps, max(x.vx.abs(), x.vy.abs()), x.vx, x.vy));
        beams[i + 1].dedup();
        beams[i + 1].truncate(BEAM_WIDTH);
        beams[i + 1].shrink_to_fit();

        x = checkpoints[i].0;
        y = checkpoints[i].1;

        let progress_percent = ((i + 1) as f64 / checkpoints.len() as f64 * 100.0) as i32;
        if progress_percent != current_progress_percent {
            eprintln!(
                "Progress: {}%  steps:{}",
                progress_percent,
                beams[i + 1][0].num_steps
            );
            current_progress_percent = progress_percent;
        }
    }

    eprintln!("Minimum steps: {}", beams[checkpoints.len()][0].num_steps);

    // Reconstruct the steps.
    let mut vxs = vec![];
    let mut vys = vec![];
    let mut steps_history = vec![];
    let mut beam_idx = 0;
    for i in (1..=checkpoints.len()).rev() {
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
    for i in 0..checkpoints.len() {
        let nx = checkpoints[i].0;
        let ny = checkpoints[i].1;
        let nvx = vxs[i + 1];
        let nvy = vys[i + 1];
        let steps_x = reconstruct_steps(nx - cx, cvx, nvx, steps_history[i], &min_steps);
        //eprintln!("[{}] reconstruct_steps_x({}, {}, {}, {}) steps_x.len: {}", i, nx-cx, cvx, nvx, steps_history[i], steps_x.len());
        let steps_y = reconstruct_steps(ny - cy, cvy, nvy, steps_history[i], &min_steps);
        //eprintln!("[{}] reconstruct_steps_y({}, {}, {}, {}) steps_x.len: {}", i, ny-cy, cvy, nvy, steps_history[i], steps_y.len());
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

fn precompute_min_steps() -> Vec<Vec<Vec<i32>>> {
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
    min_steps
}

fn get_moves(v: i32, dx: i32, max_results: usize, min_steps: &Vec<Vec<Vec<i32>>>) -> Vec<MoveInfo> {
    let mut results = vec![];
    if (dx < MIN_D || dx > MAX_D) {
        return results;
    }
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

    if steps >= 3 {
        for a0 in -1..=1 {
            for a1 in -1..=1 {
                for a2 in -1..=1 {
                    let nv0 = v + a0;
                    let ndx0 = dx - nv0;
                    let nv1 = nv0 + a1;
                    let ndx1 = ndx0 - nv1;
                    let nv2 = nv1 + a2;
                    let ndx2 = ndx1 - nv2;
                    if nv0 < MIN_V
                        || nv0 > MAX_V
                        || nv1 < MIN_V
                        || nv1 > MAX_V
                        || nv2 < MIN_V
                        || nv2 > MAX_V
                        || ndx0 < MIN_D
                        || ndx0 > MAX_D
                        || ndx1 < MIN_D
                        || ndx1 > MAX_D
                        || ndx2 < MIN_D
                        || ndx2 > MAX_D
                    {
                        continue;
                    }
                    for ve in MIN_V..=MAX_V {
                        let steps1 = min_steps[(nv2 - MIN_V) as usize][(ve - MIN_V) as usize]
                            [(ndx2 - MIN_D) as usize];
                        if steps1 == steps - 3 {
                            results.push(MoveInfo {
                                steps,
                                terminal_velocity: ve,
                            });
                        }
                    }
                }
            }
        }
    }
    if steps >= 4 {
        for a0 in -1..=1 {
            for a1 in -1..=1 {
                for a2 in -1..=1 {
                    for a3 in -1..=1 {
                        let nv0 = v + a0;
                        let ndx0 = dx - nv0;
                        let nv1 = nv0 + a1;
                        let ndx1 = ndx0 - nv1;
                        let nv2 = nv1 + a2;
                        let ndx2 = ndx1 - nv2;
                        let nv3 = nv2 + a3;
                        let ndx3 = ndx2 - nv3;
                        if nv0 < MIN_V
                            || nv0 > MAX_V
                            || nv1 < MIN_V
                            || nv1 > MAX_V
                            || nv2 < MIN_V
                            || nv2 > MAX_V
                            || nv3 < MIN_V
                            || nv3 > MAX_V
                            || ndx0 < MIN_D
                            || ndx0 > MAX_D
                            || ndx1 < MIN_D
                            || ndx1 > MAX_D
                            || ndx2 < MIN_D
                            || ndx2 > MAX_D
                            || ndx3 < MIN_D
                            || ndx3 > MAX_D
                        {
                            continue;
                        }
                        for ve in MIN_V..=MAX_V {
                            let steps1 = min_steps[(nv3 - MIN_V) as usize][(ve - MIN_V) as usize]
                                [(ndx3 - MIN_D) as usize];
                            if steps1 == steps - 4 {
                                results.push(MoveInfo {
                                    steps,
                                    terminal_velocity: ve,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    results.sort_by_key(|x| (x.terminal_velocity.abs(), x.terminal_velocity));
    results.dedup();
    results.truncate(max_results);
    results
}

fn can_reach(vs: i32, ve: i32, dx: i32, steps: i32, min_steps: &Vec<Vec<Vec<i32>>>) -> bool {
    /*
        let moves = get_moves_with_steps(vs, dx, steps, GET_MOVES_MAX_RESULTS, min_steps);
        for m in moves {
            if m.terminal_velocity == ve {
                return true;
            }
        }
        return false;
    */

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

    if steps >= 3 {
        for a0 in -1..=1 {
            for a1 in -1..=1 {
                for a2 in -1..=1 {
                    let nv0 = vs + a0;
                    let ndx0 = dx - nv0;
                    let nv1 = nv0 + a1;
                    let ndx1 = ndx0 - nv1;
                    let nv2 = nv1 + a2;
                    let ndx2 = ndx1 - nv2;
                    if nv0 < MIN_V
                        || nv0 > MAX_V
                        || nv1 < MIN_V
                        || nv1 > MAX_V
                        || nv2 < MIN_V
                        || nv2 > MAX_V
                        || ndx0 < MIN_D
                        || ndx0 > MAX_D
                        || ndx1 < MIN_D
                        || ndx1 > MAX_D
                        || ndx2 < MIN_D
                        || ndx2 > MAX_D
                    {
                        continue;
                    }
                    if min_steps[(nv2 - MIN_V) as usize][(ve - MIN_V) as usize]
                        [(ndx2 - MIN_D) as usize]
                        == steps - 3
                    {
                        return true;
                    }
                }
            }
        }
    }
    if steps >= 4 {
        for a0 in -1..=1 {
            for a1 in -1..=1 {
                for a2 in -1..=1 {
                    for a3 in -1..=1 {
                        let nv0 = vs + a0;
                        let ndx0 = dx - nv0;
                        let nv1 = nv0 + a1;
                        let ndx1 = ndx0 - nv1;
                        let nv2 = nv1 + a2;
                        let ndx2 = ndx1 - nv2;
                        let nv3 = nv2 + a3;
                        let ndx3 = ndx2 - nv3;
                        if nv0 < MIN_V
                            || nv0 > MAX_V
                            || nv1 < MIN_V
                            || nv1 > MAX_V
                            || nv2 < MIN_V
                            || nv2 > MAX_V
                            || nv3 < MIN_V
                            || nv3 > MAX_V
                            || ndx0 < MIN_D
                            || ndx0 > MAX_D
                            || ndx1 < MIN_D
                            || ndx1 > MAX_D
                            || ndx2 < MIN_D
                            || ndx2 > MAX_D
                            || ndx3 < MIN_D
                            || ndx3 > MAX_D
                        {
                            continue;
                        }
                        if min_steps[(nv3 - MIN_V) as usize][(ve - MIN_V) as usize]
                            [(ndx3 - MIN_D) as usize]
                            == steps - 4
                        {
                            return true;
                        }
                    }
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

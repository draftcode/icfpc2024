use std::collections::{BinaryHeap, HashMap, HashSet};

use euclid::default::*;

fn main() {
    let mut ps: Vec<Point2D<i64>> = std::io::stdin()
        .lines()
        .filter_map(|line| {
            let v = line
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            if v.len() != 2 || v[0] == 0 && v[1] == 0 {
                None
            } else {
                Some(Point2D::new(v[0], v[1]))
            }
        })
        .collect();

    ps.sort_by_key(|p| (p.x, p.y));
    ps.dedup();

    // solve_shortest(ps);
    solve_basic(ps);
    // solve15(ps);
}

struct TSP {
    ps: Vec<Point2D<i64>>,
}

struct TSPSTate {
    order: Vec<usize>,
    dist: i64,
}

impl saru::Annealer for TSP {
    type State = TSPState;

    type Move;

    fn start_temp(&self, init_score: f64) -> f64 {
        todo!()
    }

    fn eval(
        &self,
        state: &Self::State,
        progress_ratio: f64,
        best_score: f64,
        valid_best_score: f64,
    ) -> (f64, Option<f64>) {
        todo!()
    }

    fn neighbour(
        &self,
        state: &mut Self::State,
        rng: &mut impl rand::Rng,
        progress_ratio: f64,
    ) -> Self::Move {
        todo!()
    }

    fn apply(&self, state: &mut Self::State, mov: &Self::Move) {
        todo!()
    }

    fn unapply(&self, state: &mut Self::State, mov: &Self::Move) {
        todo!()
    }
}

fn tsp(ps: &Vec<Point2D<i64>>) {}

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

fn solve15(ps: Vec<Point2D<i64>>) {
    let n = ps.len();

    #[derive(Clone, PartialEq, Eq, Debug)]
    struct State {
        pos: Point2D<i16>,
        vel: Vector2D<i16>,
        rem: u32,
        dep: u8,
    }

    #[derive(Hash, PartialEq, Eq, Debug)]
    struct Key(Point2D<i16>, Vector2D<i16>, u32);

    impl State {
        fn new(n: usize) -> Self {
            Self {
                pos: Point2D::new(0, 0),
                vel: Vector2D::new(0, 0),
                rem: (1 << n) - 1,
                dep: 0,
            }
        }

        fn key(&self) -> Key {
            Key(self.pos, self.vel, self.rem)
        }
    }

    #[derive(PartialEq, Eq)]
    struct Priority(State);

    impl PartialOrd for Priority {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            let l = self.0.rem.count_ones() + self.0.dep as u32;
            let r = other.0.rem.count_ones() + other.0.dep as u32;
            r.partial_cmp(&l)
        }
    }

    impl Ord for Priority {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    // const UPPER_BOUND: u32 = 39;

    let init_state = State::new(n);
    let mut final_state = init_state.clone();

    let mut prev = HashMap::<Key, State>::new();
    prev.insert(init_state.key(), init_state.clone());

    // let mut q = VecDeque::new();
    // q.push_back(init_state.clone());

    let mut q = BinaryHeap::new();
    q.push(Priority(init_state.clone()));

    let mut prev_rem = 20;

    while let Some(Priority(s)) = q.pop() {
        if s.rem == 0 {
            eprintln!("done: {}", s.dep);
            final_state = s;
            break;
        }

        if prev_rem > s.rem.count_ones() {
            eprintln!("dep: {} rem: {}, q: {}", s.dep, s.rem.count_ones(), q.len());
            prev_rem = s.rem.count_ones();
        }

        for dx in -1..=1 {
            for dy in -1..=1 {
                let nv = s.vel + Vector2D::new(dx, dy);
                let npos = s.pos + nv;

                if npos.x.abs() > 12 || npos.y.abs() > 12 {
                    continue;
                }

                let mut nrem = s.rem;
                for i in 0..n {
                    if ps[i] == npos.cast() {
                        nrem &= !(1 << i);
                        break;
                    }
                }

                let ndep = s.dep + 1;

                // if ndep as u32 + nrem.count_ones() > UPPER_BOUND {
                //     continue;
                // }

                // let rest_time = UPPER_BOUND - ndep as u32;
                // if nrem.count_ones() * 2 > 3 * rest_time {
                //     continue;
                // }

                let nstate = State {
                    pos: npos,
                    vel: nv,
                    rem: nrem,
                    dep: ndep,
                };

                if let Some(prev) = prev.get(&nstate.key()) {
                    if prev.dep + 1 <= nstate.dep {
                        continue;
                    }
                }

                prev.insert(nstate.key(), s.clone());
                q.push(Priority(nstate));
            }
        }
    }

    assert_ne!(init_state, final_state);

    let mut cur = final_state;

    let mut moves = vec![];

    while cur != init_state {
        let prev = prev[&cur.key()].clone();
        moves.push(cur.vel - prev.vel);
        cur = prev;
    }

    moves.reverse();
    println!(
        "{}",
        moves
            .into_iter()
            .map(|a| to_move(a.cast()))
            .collect::<String>()
    );
}

pub mod saru {
    mod time {
        use std::time::Duration;
        #[cfg(target_arch = "wasm32")]
        use wasm_bindgen::prelude::*;

        #[cfg(not(target_arch = "wasm32"))]
        pub struct Instant(std::time::Instant);

        #[cfg(not(target_arch = "wasm32"))]
        impl Instant {
            pub fn now() -> Self {
                Self(std::time::Instant::now())
            }

            pub fn elapsed(&self) -> Duration {
                self.0.elapsed()
            }
        }

        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen(inline_js = r#"
        export function performance_now() {
          return performance.now();
        }"#)]
        extern "C" {
            fn performance_now() -> f64;
        }

        #[cfg(target_arch = "wasm32")]
        pub struct Instant(u64);

        #[cfg(target_arch = "wasm32")]
        impl Instant {
            pub fn now() -> Self {
                Self((performance_now() * 1000.0) as u64)
            }

            pub fn elapsed(&self) -> Duration {
                Duration::from_micros(Self::now().0 - self.0)
            }
        }
    }

    use rand::prelude::*;
    use std::thread;
    use thousands::Separable;
    use time::Instant;

    pub struct AnnealingOptions<A: Annealer> {
        pub time_limit: f64,
        pub limit_temp: f64,
        pub cooling_function: CoolingFunction,
        pub adaptive_cooling: bool,
        pub restart: usize,
        pub silent: bool,
        pub header: String,
        pub reporter: Option<Box<dyn Fn(&A::State) + Send + Sync + 'static>>,
        pub report_interval: f64,
    }

    impl<A: Annealer> AnnealingOptions<A> {
        pub fn new(time_limit: f64, limit_temp: f64) -> Self {
            Self {
                time_limit,
                limit_temp,
                cooling_function: CoolingFunction::Exponential,
                adaptive_cooling: false,
                restart: 1,
                silent: false,
                header: "".to_string(),
                reporter: None,
                report_interval: 1.0,
            }
        }

        pub fn set_cooling_function(mut self, cooling_function: CoolingFunction) -> Self {
            self.cooling_function = cooling_function;
            self
        }

        pub fn set_adaptive_cooling(mut self, adaptive_cooling: bool) -> Self {
            self.adaptive_cooling = adaptive_cooling;
            self
        }

        pub fn set_restart(mut self, restart: usize) -> Self {
            self.restart = restart;
            self
        }

        pub fn set_silent(mut self, silent: bool) -> Self {
            self.silent = silent;
            self
        }

        pub fn set_header(mut self, header: String) -> Self {
            self.header = header;
            self
        }

        pub fn set_reporter(
            mut self,
            reporter: impl Fn(&A::State) + Send + Sync + 'static,
        ) -> Self {
            self.reporter = Some(Box::new(reporter));
            self
        }

        pub fn set_report_interval(mut self, interval: f64) -> Self {
            self.report_interval = interval;
            self
        }
    }

    pub enum CoolingFunction {
        Exponential,
        Linear,
        Quadratic,
        Cubic,
        Quartic,
    }

    impl CoolingFunction {
        pub fn eval(&self, t0: f64, tn: f64, progress_ratio: f64) -> f64 {
            match self {
                CoolingFunction::Exponential => t0 * (tn / t0).powf(progress_ratio),
                CoolingFunction::Linear => tn + (t0 - tn) * (1.0 - progress_ratio),
                CoolingFunction::Quadratic => tn + (t0 - tn) * (1.0 - progress_ratio).powi(2),
                CoolingFunction::Cubic => tn + (t0 - tn) * (1.0 - progress_ratio).powi(3),
                CoolingFunction::Quartic => tn + (t0 - tn) * (1.0 - progress_ratio).powi(4),
            }
        }
    }

    pub struct AnnealingResult<A: Annealer> {
        pub score: f64,
        pub iterations: usize,
        pub solution: Option<<A::State as State>::Solution>,
        pub state: A::State,
    }

    pub trait State: Send + Sync {
        type Solution: Clone + Send + Sync;
        fn solution(&self) -> Self::Solution;
    }

    pub trait StateInitializer {
        type State: State;

        fn init_state(&self, rng: &mut impl Rng) -> Self::State;
    }

    pub trait Annealer {
        type State: State;
        type Move;

        fn start_temp(&self, init_score: f64) -> f64;

        fn is_done(&self, _score: f64) -> bool {
            false
        }

        fn eval(
            &self,
            state: &Self::State,
            progress_ratio: f64,
            best_score: f64,
            valid_best_score: f64,
        ) -> (f64, Option<f64>);

        fn neighbour(
            &self,
            state: &mut Self::State,
            rng: &mut impl Rng,
            progress_ratio: f64,
        ) -> Self::Move;

        fn apply(&self, state: &mut Self::State, mov: &Self::Move);
        fn unapply(&self, state: &mut Self::State, mov: &Self::Move);

        fn apply_and_eval(
            &self,
            state: &mut Self::State,
            mov: &Self::Move,
            progress_ratio: f64,
            best_score: f64,
            valid_best_score: f64,
            _prev_score: f64,
        ) -> (f64, Option<f64>) {
            self.apply(state, mov);
            self.eval(state, progress_ratio, best_score, valid_best_score)
        }
    }

    pub fn annealing<S, A>(
        annealer: &A,
        opt: &AnnealingOptions<A>,
        seed: u64,
        threads: usize,
    ) -> AnnealingResult<A>
    where
        S: State + Send + Sync,
        A: Annealer<State = S> + StateInitializer<State = S> + Send + Sync,
    {
        assert!(threads > 0);

        if threads == 1 {
            let mut rng = SmallRng::seed_from_u64(seed);
            let state = annealer.init_state(&mut rng);
            annealing_single_thread(None, annealer, opt, seed, state)
        } else {
            let mut rng = StdRng::seed_from_u64(seed);

            let res = thread::scope(|s| {
                let mut ths = vec![];

                for i in 0..threads {
                    let tl_seed = rng.gen();
                    ths.push(s.spawn(move || {
                        let mut rng = SmallRng::seed_from_u64(tl_seed);
                        let state = annealer.init_state(&mut rng);
                        annealing_single_thread(Some(i), annealer, opt, tl_seed, state)
                    }));
                }

                ths.into_iter()
                    .map(|th| th.join().unwrap())
                    .collect::<Vec<_>>()
            });

            if !opt.silent {
                eprintln!("===== results =====");
                for (i, r) in res.iter().enumerate() {
                    eprintln!("[{}]: score: {}", i, r.score);
                }
            }

            let mut iterations = 0;
            let mut best_score = f64::MAX;
            let mut best_solution = None;
            let mut best_state = None;

            for r in res {
                iterations += r.iterations;

                if let Some(s) = r.solution {
                    if r.score < best_score {
                        best_score = r.score;
                        best_solution = Some(s);
                        best_state = Some(r.state);
                    }
                } else if best_state.is_none() {
                    best_state = Some(r.state);
                }
            }
            AnnealingResult {
                iterations,
                score: best_score,
                solution: best_solution,
                state: best_state.unwrap(),
            }
        }
    }

    pub fn annealing_single_thread<A: Annealer + Send + Sync>(
        thread_id: Option<usize>,
        annealer: &A,
        opt: &AnnealingOptions<A>,
        seed: u64,
        mut state: A::State,
    ) -> AnnealingResult<A> {
        let mut rng = SmallRng::seed_from_u64(seed);

        let (mut cur_score, init_correct_score) =
            annealer.eval(&state, 0.0, f64::INFINITY, f64::INFINITY);

        let mut best_score = cur_score;

        let mut valid_best_score = f64::INFINITY;
        let mut valid_best_ans = if let Some(score) = init_correct_score {
            valid_best_score = score;
            Some(state.solution())
        } else {
            None
        };

        macro_rules! progress {
                ($($arg:expr),*) => {
                    if !opt.silent {
                        if let Some(tid) = thread_id {
                            eprint!("[{:02}] ", tid);
                        }
                        eprint!("{}", opt.header);
                        eprintln!($($arg),*);
                    }
                };
            }

        progress!("Initial score: {}", cur_score);

        let mut restart_cnt = 0;

        let t_max = annealer.start_temp(cur_score);
        let t_min = opt.limit_temp;

        let mut timer = Instant::now();
        let time_limit = opt.time_limit;

        let mut temp = t_max;
        let mut progress_ratio = 0.0;
        let mut prev_heart_beat = timer.elapsed();
        let mut prev_updated = timer.elapsed();
        let mut best_valid_updated = false;
        let mut best_updated = false;

        let mut iters = 0;

        for i in 0.. {
            if i % 100 == 0 {
                progress_ratio = timer.elapsed().as_secs_f64() / time_limit;
                if progress_ratio >= 1.0 {
                    restart_cnt += 1;
                    if restart_cnt >= opt.restart {
                        progress!(
                            "Final score: {}, {i} iteration processed, {:.2} iter/s",
                            best_score,
                            i as f64 / time_limit
                        );
                        break;
                    }
                    progress!("Restarting... {}/{}", restart_cnt, opt.restart);

                    timer = Instant::now(); // - Duration::from_secs_f64(time_limit / 2.0);
                }

                temp = opt.cooling_function.eval(t_max, t_min, progress_ratio);

                if (timer.elapsed() - prev_heart_beat).as_secs_f64() >= opt.report_interval {
                    if best_updated
                        || best_valid_updated
                        || (timer.elapsed() - prev_updated).as_secs_f64() >= opt.report_interval
                    {
                        let mark = if best_valid_updated {
                            "✅"
                        } else if best_updated {
                            "🐴"
                        } else {
                            "⛔"
                        };
                        progress!(
                            "{mark} best = {:>17}, best valid = {:>17}, cur = {:>17}, temp = {:>16}, progress: {:6.2}%",
                            format!("{:.1}", best_score).separate_with_commas(),
                            format!("{:.1}", valid_best_score).separate_with_commas(),
                            format!("{:.1}", cur_score).separate_with_commas(),
                            format!("{:.1}", temp).separate_with_commas(),
                            progress_ratio * 100.0
                        );

                        if let Some(reporter) = opt.reporter.as_ref() {
                            if thread_id.is_none() || thread_id == Some(0) {
                                reporter(&state);
                            }
                        }
                        best_updated = false;
                        best_valid_updated = false;
                        prev_updated = timer.elapsed();
                    }
                    prev_heart_beat = timer.elapsed();
                }
            }

            iters += 1;

            let mov = annealer.neighbour(&mut state, &mut rng, progress_ratio);

            let (new_score, new_correct_score) = annealer.apply_and_eval(
                &mut state,
                &mov,
                progress_ratio,
                best_score,
                valid_best_score,
                cur_score,
            );

            if let Some(new_correct_score) = new_correct_score {
                if new_correct_score < valid_best_score {
                    if valid_best_score - new_correct_score > 1e-6 {
                        best_valid_updated = true;
                    }
                    valid_best_score = new_correct_score;
                    valid_best_ans = Some(state.solution());
                }
            }

            // let temp = if opt.adaptive_cooling && new_score > cur_score {
            //     temp * (1.0 + (new_score - best_score) / new_score.abs())
            // } else {
            //     temp
            // };

            if new_score <= cur_score
                || rng.gen::<f64>() <= ((cur_score - new_score) as f64 / temp).exp()
            {
                cur_score = new_score;

                if cur_score < best_score {
                    if best_score - cur_score > 1e-6 {
                        best_updated = true;
                    }

                    best_score = cur_score;
                }

                if annealer.is_done(cur_score) {
                    break;
                }
            } else {
                annealer.unapply(&mut state, &mov);
            }
        }

        AnnealingResult {
            iterations: iters,
            score: valid_best_score,
            solution: valid_best_ans,
            state,
        }
    }
}

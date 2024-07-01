use anyhow;

use common::lambdaman::{self, map::LMap};

fn get_sequence(s: i32, l: usize) -> String {
    let mut res = String::new();
    let mut s = s;
    for _ in 0..l {
        res += match s % 4 {
            0 => "U",
            1 => "D",
            2 => "L",
            3 => "R",
            _ => "_",
        };
        s = s / 4;
    }
    res
}

fn check(seq: &str, seq2: &str) -> bool {
    for r2 in 1..81 {
        let mut map4 = LMap::from_id(4).unwrap();
        for repeat in 1..16 {
            for _ in 0..r2 {
                for i in 0..seq.len() {
                    let _ = map4.do_move(&seq[i..i + 1]).unwrap();
                }
            }
            for _ in 0..r2 {
                for i in 0..seq.len() {
                    let _ = map4.do_move(&seq2[i..i + 1]).unwrap();
                }
            }
            if map4.remaining_pills() == 0 {
                println!("{} {} {} {}", seq, seq2, repeat, r2);
                return true;
            }
        }
    }

    false
}

fn generate(l: usize) {
    let mx = 1 << (2 * l);
    for seed in 0..mx {
        let seq = get_sequence(seed, l);
        for seed2 in 0..mx {
            let seq2 = get_sequence(seed2, l);
            println!("try {} {}", seq, seq2);
            if check(seq.as_str(), seq2.as_str()) {
                println!("Found {}", seq);
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    for l in 1..10 {
        generate(l);
    }

    Ok(())
}

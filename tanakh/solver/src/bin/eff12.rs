fn main() {
    for i in 1..=100 {
        println!("{i}: {}", f(i));
        assert_eq!(f(i), fast(i));
    }

    println!("{}", fast(0x100));
    println!("{}", fast(0x1000));
    println!("{}", fast(0x10000));
    println!("{}", fast(0x12d687));
}

fn fast(x: i64) -> i64 {
    let mut ps = vec![];
    let mut sieve = vec![true; x as usize + 1];

    for p in 2..=x {
        if sieve[p as usize] {
            ps.push(p);
            for q in (p * p..=x).step_by(p as usize) {
                sieve[q as usize] = false;
            }
        }
    }

    let mut z = x;
    for p in ps {
        if x % p == 0 {
            z = z / p * (p - 1);
        }
    }
    (z + 1).min(x)
}

#[memoise::memoise(x)]
fn f(x: i64) -> i64 {
    (1 + if x > 2 { g(x, x) } else { x }).min(x)
}

fn g(x: i64, mut z: i64) -> i64 {
    for y in 2..x {
        let fy = f(y);
        if fy > y - 1 && x % y == 0 {
            assert_eq!(fy, y);
            z = z / y * (y - 1)
        }
    }
    z
}

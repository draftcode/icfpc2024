const SOLUTION: &str = "1 -2 -3 -4 -5 6 -7 -8 -9 10 -11 12 -13 -14 15 -16 -17 -18 -19 -20 -21 22 -23 -24 25 26 -27 28 -29 -30 -31 -32 -33 -34 -35 36 -37 -38 -39 40";

fn main() {
    let w = SOLUTION
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut ans = 0_i64;

    for b in w {
        if b > 0 {
            ans += 1 << (b - 1);
        }
    }

    println!("{ans}");
}

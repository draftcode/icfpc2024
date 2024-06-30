fn main() {
    let mut max = 0;
    for x in -99..=99 {
        for y in -99..=99 {
            if y == 0 {
                continue;
            }

            let mut cnt = 0;
            for a in -100..=100_i32 {
                let m = (a + x) % y;
                if m == a.signum() {
                    cnt += 1;
                }
            }

            if cnt >= max {
                max = cnt;
                println!("{x} {y}: {cnt}");
            }
        }
    }
}

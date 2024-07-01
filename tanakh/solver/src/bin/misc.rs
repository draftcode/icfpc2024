fn main() {
    for a in 2..=10_000 {
        for x in 2.. {
            let mut s = vec![];
            let mut t = a;
            while t > 0 {
                s.push(t % x);
                t /= x;
            }

            let r = s.iter().rev().cloned().collect::<Vec<_>>();
            if s == r {
                println!("{a}: {x}");
                break;
            }
        }
    }
}

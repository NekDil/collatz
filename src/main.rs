fn collatz(num: u64, v: &mut Vec<u32>) -> u32 {
    let mut n = num as usize;
    let mut computed = 0u32;

    while n != 1 {
        match v.get(n) {
            Some(c) => {
                computed = computed + c;
                break;
            }
            _ => {
                computed += 1;
                if n % 2 == 1 {
                    n = n * 3 + 1;
                } else {
                    n = n / 2;
                }
            }
        }
    }
    v.push(computed);
    *(v.get(num as usize).unwrap())
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let mut max = 0;
    let mut max_num = 0;
    let mut cache = vec![1u32];
    for n in 1..=100_000_000 {
        let c = collatz(n, &mut cache);
        if c > max {
            max = c;
            max_num = n;
        }

        // println!("{} => {}", &n, &c);
    }
    let elapsed = now.elapsed();

    println!(
        "Maximum: collatz({}) {}, Cache Size: {}. Elapsed: {:.2?}",
        max_num,
        max,
        cache.len(),
        elapsed
    );
}

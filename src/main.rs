#[inline]
fn collatz(num: usize, cache: &mut Vec<u32>) -> u32 {
    let mut n = num;
    let mut computed = 0u32;

    if n % 2 == 0 {
        computed = *(cache.get(n >> 1).unwrap()) + 1;
        cache.push(computed);
        computed
    } else {
        while n != 1 {
            match cache.get(n) {
                Some(c) => {
                    computed = computed + c;
                    break;
                }
                _ => {
                    computed += 1;
                    if n % 2 == 1 {
                        n = n * 3 + 1;
                    } else {
                        n >>= 1;
                    }
                }
            }
        }
        cache.push(computed);
        computed
    }
}

fn main() {
    use std::time::Instant;
    let limit = 100_000_000;
    let now = Instant::now();
    let mut max = 0;
    let mut max_num = 0;
    let mut cache = vec![1u32];

    for n in 1..=limit {
        let c = collatz(n, &mut cache);
        if c > max {
            max = c;
            max_num = n;
        }
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

use std::collections::HashMap;

fn collatz(orig: u64, num: u64, cache: &mut HashMap<u64, u32>) -> u32 {
    if cache.contains_key(&num) {
        *(cache.get(&num).expect("not computed yet"))
    } else {
        let computed = 1 + match num % 2 {
            1 => collatz(orig, num * 3 + 1, cache),
            _ => collatz(orig, num / 2, cache),
        };
        // if num <= orig {
        cache.insert(num, computed);
        //}
        computed
    }
}

fn collatz_length(n: u64, cache: &mut HashMap<u64, u32>) -> u32 {
    collatz(n, n, cache)
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let mut max = 0;
    let mut max_num = 0;
    let mut cache = HashMap::from([(1u64, 1u32)]);
    for n in 1..100000000 {
        let c = collatz_length(n, &mut cache);
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

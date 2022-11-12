use std::collections::HashMap;

fn collatz_helper(orig: u64, num: u64, cache: &mut HashMap<u64, u32>) -> u32 {
    if cache.contains_key(&num) {
        *(cache.get(&num).expect("not computed yet"))
    } else {
        let computed = 1 + match num % 2 {
            1 => collatz_helper(orig, num * 3 + 1, cache),
            _ => collatz_helper(orig, num / 2, cache),
        };
        if num <= orig {
            cache.insert(num, computed);
        }
        computed
    }
}

fn collatz_length(n: u64, cache: &mut HashMap<u64, u32>) -> u32 {
    collatz_helper(n, n, cache)
}

fn main() {
    let mut max = 0;
    let mut cache = HashMap::from([(1u64, 1u32)]);
    for n in 1..1000000 {
        let c = collatz_length(n, &mut cache);
        if c > max {
            max = c
        }

        // println!("{} => {}", &n, &c);
    }

    println!("Maximum: {}, Cache Size: {}", max, cache.len());
}

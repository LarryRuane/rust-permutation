extern crate rand;
use rand::Rng;

use std::collections::HashMap;
use std::env;

// if key i exists in the hash table, return the content, else i
fn get(swaps: &HashMap<usize, usize>, i: usize) -> usize {
    match swaps.get(&i) {
        None => i,
        Some(j) => *j,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: usize = args[1].parse().expect("not an integer");
    let mut m = n;
    if args.len() >= 3 {
        m = args[2].parse().expect("not an integer");
        if n > m {
            panic!("n({}) must be less than or equal to m({})", n, m);
        }
    }
    let mut result: Vec<usize> = vec![0; n];
    let mut swaps: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        let swap_index: usize = rand::thread_rng().gen_range(0, m - i) + i;
        result[i] = get(&swaps, swap_index);
        let v = get(&swaps, i);
        swaps.insert(swap_index, v);
    }
    println!("{:?}", result);
}

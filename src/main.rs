#![feature(test)]
extern crate test;

use std::ops::Range;

fn sift_table(max: usize, table: &mut Vec<usize>, range: Range<usize>) {
    for i in range {
        let mut j = i;
        while max > j + i {
            table[j + i] = 0;
            j += i;
        }
    }
}

fn eratosthenes(max: usize) -> usize {
    let limit = (max as f32).sqrt().floor() as usize;
    let mut table = vec![0; max];
    for i in 1..(max-1) { table[i] = i }
    
    sift_table(max, &mut table, 2..limit);

    let results = table.iter().filter(|i| **i != 0 as usize);
    return results.count();
}

fn main() {
    println!("primes: {} 個", eratosthenes(10000000) );
   
    // for i in results {print!("{} ", i);}
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn eratosthenes_works() {
        assert_eq!(170,  eratosthenes(1000));
    }

    #[bench]
    fn bench_eratosthenes(b: &mut Bencher) {
        b.iter(|| eratosthenes(10000000));
    }
}

#![feature(test)]
extern crate test;
extern crate rayon;

use rayon::prelude::*;

fn sift_table(max: usize, start: usize, end: usize) -> Vec<usize> {
    let len = end - start;
    let mut table = vec![0; len+1];

    for i in 1..len { table[i] = i + start; }
    // 1をtableから削除
    if start == 0 { table[1] = 0; }

    let limit = (max as f32).sqrt().floor() as usize;
    for i in 2..limit {
        let diff = start % i;
        let mut j = i;
        if start == 0 { j += i; }
        while len >= j - diff {
            table[j - diff] = 0;
            j += i;
        }
    }
    return table;
}

fn eratosthenes(max: usize) -> usize {
    // 分割サイズ。分割は偶数にする前提
    let core = 8;
    let limit = (max as f32 / core as f32).floor() as usize;

    let mut cores = vec![0; core];
    for i in 0..core { cores[i]=i }

    let table: Vec<usize> = cores
        .into_par_iter()
        .map(|i| sift_table(max, i * limit, (i+1) * limit) )
        .flatten().collect();

    let results = table.iter().filter(|i| **i != 0 as usize);
    //for i in results { print!("{} ", i); }
    //return 0;
    return results.count();
}

fn main() {
    // println!("primes: {} 個", eratosthenes(1000) );
    // println!("primes: {} 個", eratosthenes(1000) );
    println!("primes: {} 個", eratosthenes(10000000) );
   
    // for i in results {print!("{} ", i);}
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn eratosthenes_works() {
        assert_eq!(169,  eratosthenes(1000));
    }

    #[bench]
    fn bench_eratosthenes(b: &mut Bencher) {
        b.iter(|| eratosthenes(10000000));
    }
}

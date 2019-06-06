#![feature(test)]

extern crate test;
extern crate rayon;

use rayon::prelude::*;

fn separate_eratosthenes(limit: u32, start: u32, end: u32) -> Vec<u32> {
    let len = end - start;
    let mut table: Vec<u32> = vec![1; (len + 1) as usize];
    table[0] = 0;

    //for i in 1..len { table[i] = i + start; }
    // 1をtableから削除
    if start == 0 { table[1] = 0; }

    for i in 2..limit {
        let mut j = i;
        let diff = start % i;
        if start == 0 { j += i; }
        j -= diff;
        while len >= j {
            table[j as usize] = 0;
            j += i;
        }
    }
    return table;
}

fn eratosthenes(max: u32) -> u32 {
    // 分割サイズ。分割は偶数にする前提
    let core = 128;
    let limit = (max as f32 / core as f32).floor() as u32;

    let mut cores: Vec<u32> = vec![0; core as usize];
    for i in 0..core { cores[i as usize] = i as u32 }

    let table: Vec<u32> = cores
        .into_par_iter()
        .map(|i| separate_eratosthenes(limit, i * limit, (i+1) * limit) )
        .flatten().collect();

    let results = table.iter().filter(|i| **i != 0);
    //for i in results { print!("{} ", i); }
    // return 0;
    return results.count() as u32;
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

#![feature(test)]

extern crate test;
extern crate rayon;

use rayon::prelude::*;
use std::{mem};

#[cfg(target_arch = "x86")]
use std::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

fn separate_eratosthenes(limits: &Vec<u32>, start: u32, end: u32) -> Vec<u32> {
    let len = end - start;
    let mut table: Vec<u32> = vec![0; (len + 1) as usize];
    for i in 1..len { table[i as usize]=i; }

    // 1をtableから削除
    if start == 0 { table[1] = 0; }

    for i in limits {
        let mut j = *i;
        let diff = start % i;
        if start == 0 { j += i * 8; }
        j -= diff;
        unsafe {
            let i32i = *i as i32;
            let i32j = j as i32;
            let mut points = _mm256_set_epi32(i32j + i32i, i32j + i32i * 2, i32j + i32i * 3, i32j + i32i * 4, i32j + i32i * 5, i32j + i32i * 6, i32j + i32i * 7, i32j + i32i * 8);
            let skips = _mm256_set_epi32(i32i * 8, i32i * 8, i32i * 8, i32i * 8, i32i * 8, i32i * 8, i32i * 8, i32i * 8);
            while len >= j + i * 8 {
                let points_map: (i32, i32, i32, i32, i32, i32, i32, i32) = mem::transmute(points);
                table[points_map.0 as usize] = 0;
                table[points_map.1 as usize] = 0;
                table[points_map.2 as usize] = 0;
                table[points_map.3 as usize] = 0;
                table[points_map.4 as usize] = 0;
                table[points_map.5 as usize] = 0;
                table[points_map.6 as usize] = 0;
                table[points_map.7 as usize] = 0;
                points = _mm256_add_epi32(points, skips);
                j += i * 8;
            }
        }
        j -= i * 8;
        // 8倍すると余ってしまう末尾の処理
        while len >= j {
            table[j as usize] = 0;
            j += i;
        }
    }
    return table;
}

fn eratosthenes(max: u32) -> u32 {
    // 分割サイズ。分割は偶数にする前提
    let core = 64;
    let limit = (max as f32).sqrt().floor() as u32;
    let separated_size = (max as f32 / core as f32).floor() as u32;

    let mut limits = vec![0; (limit-2) as usize];
    for i in 2..limit { limits[(i-2) as usize] = i as u32 }
    let shifted_limits: Vec<u32> = separate_eratosthenes(&limits, 0, limit).iter().filter(|i| **i != 0).map(|i| *i ).collect();

    let mut cores: Vec<u32> = vec![0; core as usize];
    for i in 0..core { cores[i as usize] = i as u32 }
    let table: Vec<u32> = cores
        .into_par_iter()
        .map(|i| separate_eratosthenes(&shifted_limits, i * separated_size, (i+1) * separated_size) )
        .flatten().collect();

    let results = table.iter().filter(|i| **i != 0);
    //for i in results { print!("{} ", i); }
    // return 0;
    return results.count() as u32;
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
        assert_eq!(169,  eratosthenes(1000));
    }

    #[bench]
    fn bench_eratosthenes(b: &mut Bencher) {
        b.iter(|| eratosthenes(10000000));
    }
}

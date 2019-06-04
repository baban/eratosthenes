#![feature(test)]
extern crate test;

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
    let start = 0;
    let end = max;
    
    // 分割は偶数にする前提
    let limit = (max as f32 / 4.0).floor() as usize;
    let mut table1 = sift_table(max, 0, limit);
    let mut table2 = sift_table(max, limit, limit * 2);
    let mut table3 = sift_table(max, limit * 2, limit * 3);
    let mut table4 = sift_table(max, limit * 3, max);

    let mut table = table1;
    table.append(&mut table2);
    table.append(&mut table3);
    table.append(&mut table4);

    let results = table.iter().filter(|i| **i != 0 as usize);
    // for i in results { print!("{} ", i); }
    // return 0;
    return results.count();
}

fn main() {
    //println!("primes: {} 個", eratosthenes(1000) );
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

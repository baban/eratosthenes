fn eratosthenes(max: usize) -> usize {
    let limit = (max as f32).sqrt().floor() as usize;
    let mut table = vec![0; max];
    for i in 1..(max-1) { table[i] = i }
    for i in 2..limit {
        let mut j = i;
        while max > j + i {
            table[j + i] = 0;
            j += i;
        }
    }
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

    #[test]
    fn eratosthenes_works() {
        assert_eq!(170,  eratosthenes(1000));
    }
}

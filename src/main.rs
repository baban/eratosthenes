fn main() {
    let max = 1000;
    let limit = (max as f32).sqrt() as usize + 1;
    let mut table = vec![0; max];
    for i in 1..(max-1) {
        table[i] = i;
    }
    for i in 2..limit {
        let mut j = i;
        while max > j + i {
            table[j + i] = 0;
            j += i;
        }
    }
    let results = table.iter().filter(|i| **i != 0 as usize);
    for i in results {
        print!("{} ", i);
    }
}

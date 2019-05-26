fn main() {
    let max = 1000;
    // let limit = (max as f32).sqrt() as usize;
    let mut table = vec![0; max];
    for i in 1..(max-1) {
        table[i] = i;
    }
    let mut table2 = table.clone();
    for i in 2..(max-1) {
        let mut j = i;
        while max > j + i {
            table2[j + i] = 0;
            j += i;
        }
    }
    
    for i in 1..max {
        if table2[i] != 0 {
            print!("{} ", i);
        }
    }
    
}

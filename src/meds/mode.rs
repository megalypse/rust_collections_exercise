use std::collections::HashMap;

pub fn mode_calc(vector: &Vec<i32>) -> i32 {
    let mut count_map: HashMap<i32, i32> = HashMap::new();
    let mut winner: (i32, i32) = (0, 0);
    
    for &i in vector.iter() {
        let count = count_map.entry(i).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", count_map);

    for (key, val) in count_map {
        if val > winner.1 {
            winner.0 = key;
            winner.1 = val;
        }
    }

    winner.0
}
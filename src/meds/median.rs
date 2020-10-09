pub fn median_calc(vector: &Vec<i32>) -> i32 {
    let length: usize = vector.iter().len();
    let middle = length as f64 / 2.0;
    let module = length as f64 % 2.0;
    let mut res = 0;

    if module == 0.0 {
        let middle: usize = middle as usize;
        res = (vector[middle] + vector[middle - 1]) / 2;
    } else {
        res = vector[(middle.round() as usize) - 1];
    }
    res
}
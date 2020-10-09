pub fn mean_calc(numbers: &Vec<i32>) -> i32 {
    let length = numbers.len();
    let sum: i32 = numbers.iter().sum();
    let mean: i32 = sum / length as i32;

    mean
}
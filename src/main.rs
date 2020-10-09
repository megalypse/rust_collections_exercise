mod meds;

use meds::mean::mean_calc;
use meds::median::median_calc;
use meds::mode::mode_calc;

fn main() {
    let my_vec: Vec<i32> = vec![13, 89, 72, 28, 51, 200, 28, 31, 31, 33, 31];
    let mean = mean_calc(&my_vec);
    let median = median_calc(&my_vec);
    let mode = mode_calc(&my_vec);

    println!("Mean: {}", mean);
    println!("Median: {}", median);
    println!("Mode: {}", mode);
}

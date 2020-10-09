mod meds;
mod pig_latim;

use meds::mean::mean_calc;
use meds::median::median_calc;
use meds::mode::mode_calc;
use pig_latim::pig_latimner::pig_latifier;

fn main() {
    let my_vec: Vec<i32> = vec![13, 89, 72, 28, 51, 200, 28, 31, 31, 33, 31];
    let my_name = String::from("Aruno");
    let mean = mean_calc(&my_vec);
    let median = median_calc(&my_vec);
    let mode = mode_calc(&my_vec);
    let my_name = pig_latifier(my_name);

    println!("Mean: {}", mean);
    println!("Median: {}", median);
    println!("Mode: {}", mode);
    println!("Latified: {}", my_name);
}

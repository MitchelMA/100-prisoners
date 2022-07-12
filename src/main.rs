use rand::Rng;
use std::env;
use std::sync::mpsc;
use std::thread;

fn main() {
    let mut args = env::args().skip(1);
    let rounds = 500;
    let size = 100;
    let first = match args.next() {
        Some(v) => match v.parse::<i32>() {
            Ok(num) => num,
            Err(_) => rounds,
        },
        None => rounds,
    };
    let second = match args.next() {
        Some(v) => match v.parse::<i32>() {
            Ok(num) => num,
            Err(_) => size,
        },
        None => size,
    };
    println!("rounds: {}", first);
    println!("group-size: {}", second);
    let outcome = prisoners_slip_mult(first as usize, second as usize, 1);
    println!("overal chance: {}%", outcome * 100.0);
}

fn random_vector(length: usize) -> Vec<usize> {
    let mut thread_rng = rand::thread_rng();
    let mut v = Vec::<usize>::new();
    let mut index: usize = 0;
    while index < length {
        let r = thread_rng.gen_range(1..length + 1);
        if v.contains(&r) {
            continue;
        }

        v.push(r);

        index += 1;
    }
    return v;
}

fn prisoners_slip(times: usize, length: usize) -> f64 {
    // vector containing the shuffled prisoner's slips
    let mut boxes;
    // vector containing all the values of passed groups
    let mut v = Vec::<bool>::new();
    // amount of passed groups
    let mut count = 0.0;
    // loop for how many times the riddle should play to get a better
    // representation of the outcome precentage
    for i in 0..times {
        boxes = random_vector(length);
        // boolean that determines if this group of prisoners passed or not
        let mut passed = true;
        // loop to loop through all the prisoners
        for j in 1..boxes.len() + 1 {
            // let the prisoner choose 50 times
            // if it is able to find its number in the boxes, it succeeds
            // but if it doesn't, everyone will die
            // get the current prisoner
            let current_prisoner = j;
            // begin with the box that has the number of the prisoner
            let mut current_box_value = boxes[current_prisoner as usize - 1];
            // check if this prisoner passed or not
            let mut current_pass = false;
            for _k in 0..(length / 2) {
                // if the value in the current box and that of the current prisoner are the same,
                // that means that the prisoner has passed
                if current_box_value == current_prisoner {
                    current_pass = true;
                    break;
                }

                // else it should go to the box with the labelnumber that the previous box had a slip of
                current_box_value = boxes[current_box_value as usize - 1];
            }
            if !current_pass {
                passed = false;
                break;
            }
        }
        v.push(passed);
        if passed {
            count += 1.0;
        }
        // calc the percentage thusfar
        println!(
            "chance at round {}: {}%",
            i,
            count / (v.len() as f64) * 100.0,
        );
    }

    return count / (v.len() as f64);
}

fn prisoners_slip_mult(times: usize, length: usize, threads: u8) -> f64 {
    let mut v = Vec::<bool>::new();

    let mut count = 0.0;

    for i in 0..times {
        let batch_outcome = || {
            let boxes = random_vector(length);

            let mut passed = true;
        };
    }

    todo!("determine outcome!");
}

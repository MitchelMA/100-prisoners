use rand::{thread_rng, Rng};

fn main() {
    println!("Hello, world!");
    let boxes = random_vector(100);
    for (i, v) in boxes.iter().enumerate() {
        // println!("{}: {}", i, *v);
    }
    let outcome = prisoners_slip(boxes, 1);
    println!("overal chance: {}%", outcome * 100.0);
}

fn random_vector(length: u8) -> Vec<u8> {
    let mut thread_rng = rand::thread_rng();
    let mut v = Vec::<u8>::new();
    let mut index: usize = 0;
    loop {
        if !(index < length.into()) {
            break;
        }
        let r = thread_rng.gen_range(1..length + 1);
        if v.contains(&r) {
            continue;
        }

        v.push(r);

        index += 1;
    }
    return v;
}

fn prisoners_slip(boxes: Vec<u8>, times: usize) -> f64 {
    let mut v = Vec::<bool>::new();
    // loop for how many times the riddle should play to get a better
    // representation of the outcome precentage
    for i in 0..times {
        // boolean that determines if this group of prisoners passed or not
        let mut passed = true;
        // loop to loop through all the prisoners
        for j in 1..boxes.len() + 1 {
            // let the prisoner choose 50 times
            // if it is able to find its number in the boxes, it succeeds
            // but if it doesn't, everyone will die
            // get a random initial choice
            let mut initial_choise = thread_rng().gen_range(0..boxes.len());
            // get the value in the initial choice
            let mut current_box_value = boxes[initial_choise];
            // get the current prisoner
            let current_prisoner = j as u8;
            // check if this prisoner passed or not
            let mut current_pass = false;
            for k in 0..50 {
                // if the value in the current box and that of the current prisoner are the same,
                // that means that the prisoner has passed
                if current_box_value == current_prisoner {
                    println!(
                        "prisoner {} found its box on choice {}",
                        current_prisoner,
                        k + 1
                    );
                    current_pass = true;
                    break;
                }

                // else it should get the next box
                println!("previous value: {}", current_box_value);
                current_box_value = boxes[current_box_value as usize - 1];
                println!("current-value: {}", current_box_value);
                println!("searching for: {}, on choice {}", current_prisoner, k + 1);

                // make sure that the prisoner doesn't get stuck in a loop
                println!("{} : {}", current_box_value, boxes[initial_choise]);
                if current_box_value == boxes[initial_choise] {
                    println!("PRISONER {} FOUND A LOOP!!!!!", current_prisoner);
                    initial_choise = thread_rng().gen_range(0..boxes.len());
                    current_box_value = boxes[initial_choise];
                }
            }
            if !current_pass {
                println!("prisoner {} didn\'t pass", current_prisoner);
                passed = false;
                break;
            }
        }
        v.push(passed);
    }

    let mut count = 0 as f64;
    for i in v.iter() {
        if *i == true {
            count += 1.0;
        }
    }

    return count / (v.len() as f64);
}

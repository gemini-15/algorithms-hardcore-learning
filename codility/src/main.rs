use std::vec;




/**
 * BinaryGap
 */
fn binarygap_solution(N: i32) -> Vec<i32> {
    // interating through the bits of N
    let mut vec_gaps : Vec<i32> = Vec::new();
    let res = (0..32).rev().map (|n| (N >> n) & 1);
    let mut counter = 0;
    let mut bounded = false;
    for bit in res {
        if (bit & 1) == 0 {
            counter +=1; 
        }
        else {
            if bounded == true {
                vec_gaps.push(counter);
            }
            bounded = true;
            counter = 0; 
        }
    }
    if vec_gaps.len() == 0 {
        vec_gaps.push(0);
        vec_gaps
    } else {
        vec_gaps
    }
}

fn rotation_by_one(vector: Vec<i32>) -> Vec<i32> {
    let vector_length = vector.len();
    let mut new_vector: Vec<i32> = Vec::new();
    new_vector.push(vector[vector_length-1]);
    for index_element in 0..vector_length-1 {
        if index_element+1 < vector_length {
            new_vector.push(vector[index_element])
        }
    }
    new_vector
}

fn cyclic_rotation_solution(vector: Vec<i32>, rotations: i32) -> Vec<i32> {
    let mut cyclic_rot = vector;
    for rotation in 0..rotations {
        cyclic_rot = rotation_by_one(cyclic_rot);
        println!("rotation {:?}, is : {:?}", rotation, cyclic_rot);
    }
    cyclic_rot
}

fn main() {
    /*
     * binary gap test
     */
    // let gaps = binarygap_solution(32);
    // println!("{:?}", gaps);
    // let max_gap = gaps.iter().max();
    // println!("{:?}", max_gap.unwrap());

    /* 
        Cyclic rotation test
     */
    // let test_vector = vec![0, 0, 0];
    // let test_rotation = 3;
    // let cyclic_rot = cyclic_rotation_solution(test_vector, test_rotation);
    // println!("{:?}", cyclic_rot);
}

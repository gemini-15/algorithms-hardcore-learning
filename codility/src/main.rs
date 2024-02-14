use std;
use num_integer;

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


fn odd_occurence_in_array_solution(array: Vec<u32>) -> u32 {
    let mut array_of_occurences:Vec<u32> = Vec::new();
    let mut max_value = *array.iter().max().unwrap();
    for i in 1..max_value+2{
        array_of_occurences.push(0);
    }

    for element in array {
        array_of_occurences[element as usize] += 1;
    }

    for i in 1..max_value+2 {
        if array_of_occurences[i as usize] == 1{
            println!("{:?}", i);
            return i
        }    
    }
    0
}

/**
 * Frog_jmp
 */
fn frog_jmp_solution(X: i32, Y: i32, D: i32) -> i32 {
    let (quotient, rest) =  num_integer::div_rem(X-Y, D);
    if rest > 0{
        quotient+1
    }
    else {
        quotient
    }
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

    /*
        Odd Occurence in array
     */
    // let test_vector_odd = vec![9, 3, 9, 3, 9, 7, 9];
    // println!("{:?}", odd_occurence_in_array_solution(test_vector_odd));

    /*
        Frog jump

     */
    println!("{:?}", frog_jmp_solution(120, 10, 30));
}



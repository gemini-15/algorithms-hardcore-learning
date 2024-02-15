
use std::{self, vec};
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

/**
 * perm_missing_Elem
 */
fn perm_missing_elem_solution(vector: Vec<u16>) -> u16 {
    let max_element: u16 = (vector.len() +1) as u16;
    let combination = max_element * (max_element+1)/2;
    let sum_all_elements:u16 = vector.iter().sum();
    let missing = combination - sum_all_elements;
    missing
}


/**
 * tape_equilibrium
 */
fn tape_equilibrium(vector: Vec<u16>) -> u16 {
    let mut min_absolute_diff: u16 = vector.len() as u16; 
    for p in 0..(vector.len() as u16) {
        let mut vector_clone = vector.clone();
        let sec_vec = vector_clone.split_off(p.into()); 
        let sum1:u16 = sec_vec.iter().sum();
        let sum2:u16 = vector_clone.iter().sum();
        let absolut_diff = sum1.abs_diff(sum2);
        if absolut_diff < min_absolute_diff {
            min_absolute_diff = absolut_diff;
        }

   }
   min_absolute_diff
}

/**
 * frogRiverOne
 */
fn frog_river_one(X: u16, vector: Vec<u16>) -> i16 {
    use std::collections::HashSet;
    let time_u16 = vector.len() as u16;
    println!("{:?}", time_u16);
    let mut t: u16 = 0;
    let mut positions_known:HashSet<u16> = HashSet::new();
    while positions_known.len() == X.into() || t < time_u16-2 {
        positions_known.insert(vector[t as usize]);
        t +=1;
    }   
    println!("{positions_known:?}");
    if t >= time_u16-1 {
        -1
    }
    else {
        t.try_into().unwrap()
    }
}

/**
 * PermCheck
 */
fn permcheck_solution(vector: Vec<u16>) -> u8 {
    let length_vector = vector.len();
    let mut combination = (length_vector+1)*length_vector/2;
    let mut sum_vector = 0;
    for element in vector.clone() {
        sum_vector += element;
    }
    if (combination as u16) == sum_vector {
        1
    }
    else {
        0
    }
}

/**
 * maxcounters
 */
fn increase_one_index(index: i32, vector: Vec<i32>) -> Vec<i32> {
    let mut vector_increased = vector.clone();
    vector_increased[index as usize] +=1; 
    vector_increased
}

fn max_counters(vector: Vec<i32>) -> Vec<i32> {
    let max_counter = vector.iter().max().unwrap();
    vec![*max_counter; vector.len()]
}

fn max_counters_solution(N: i32, vector: Vec<i32>) -> Vec<i32> {
    let mut counters: Vec<i32> = vec![0; N as usize]; //init N counters to 0 
    for element in vector {
        if element > N {
            counters = max_counters(counters.clone());
        }
        else {
            counters = increase_one_index(element-1, counters.clone())
        }
        println!("{:?}", counters);
    }
    counters
} 


/**
 * Missing integer
 */
fn missing_integer(vector: &Vec<i32>)  -> i32 {
    use std::collections::HashSet;

    let mut numbers = HashSet::new();

    // Removing all the negative numbers and 0
    for &number in vector.iter() {
        if number > 0 {
            numbers.insert(number);
        }
    }

    let mut missing_number = 1;

    while numbers.contains(&missing_number)  {
        missing_number +=1;
    }

    missing_number
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
    // println!("{:?}", frog_jmp_solution(120, 10, 30));


    /*
        PermMissingElem
     */
    // let A = vec![2, 3, 1, 5];
    // println!("{:?}", perm_missing_elem_solution(A));


    /*
        TapeEquilibrum
     */

    //  let A = vec![3, 1, 2, 4, 3];
    //  println!("{:?}", tape_equilibrium(A));


     /*
        Frog river one
      */

    // let A = vec![1, 3, 1, 4, 2, 3, 5, 4];
    // println!("{:?}",frog_river_one(5, A));

    /*
        Permcheck
     */
    // let A = vec![4, 1, 3, 2];
    // println!("{:?}",permcheck_solution(A));

    /*
        Max counters
     */
    // let A = vec![3, 4, 4, 6, 1, 4, 4];
    // println!("{:?}", max_counters_solution(5, A));

    /*
        Smallest missing 
     */
    let A = vec![-1, -3];
    println!("{:?}", missing_integer(&A));

}



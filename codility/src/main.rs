
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

// pub fn solution(A: &mut Vec<i32>) -> i32 {
//     use std::collections::HashSet;

//     let mut smallest_pos:i32 = 1;
//     let slice = A.as_slice();
//     let mut set_vec : HashSet<i32> = HashSet::new();
//     for element in slice {
//         let diff = 0;
//         if element > &mut 0 {
//             set_vec.insert(*element);
//         }
//     }

//     while set_vec.contains(&smallest_pos) {
//         smallest_pos +=1;
//     }

//     smallest_pos

// }
/**
 * countdiv
 */
fn count_div_solution(A: u32, B: u32, K:u32) -> u32 {
    let range: Vec<u32> = (A..B).collect();
    let mut dividables = 0u32;  
    for element in range {
        if element%K == 0 {
            dividables +=1;
        }
    }
    dividables
}

/**
 * Distinct 
 */
fn Distinct_solution(vector: &Vec<i16>) -> i16 {
    use std::collections::HashSet;
    let mut distinct = HashSet::new();
    for element in vector {
        distinct.insert(*element);
    }
    distinct.len().try_into().unwrap()
}   


/**
 * MaxProductOfThree
 * 
 */
fn max_product_of_three_solution(vector: &Vec<i16>) -> i16 {
    use std::collections::HashSet;

    // getting the hashset directly
    let mut vector_set: HashSet<i16> = HashSet::from_iter(vector[..].iter().cloned());

    // sorting the biggest 3 elements
    let mut max_three = 1;

    for _ in 0..3 {
        if let Some(max_value) = vector_set.iter().cloned().max() {
            max_three = max_three*max_value;
            vector_set.remove(&max_value);
        }
    }
    
    max_three

}

fn number_of_disc_intersections(vector: &Vec<i32>) -> i32 {
    let number_intersections = 0;
    
    let array_start: Vec<i32> = Vec::new();
    let array_end:Vec<i32> = Vec::new();

    let mut intervals : Vec<(i32, i32)> = Vec::new();
    // let t: i32 = (vector.len()-1).try_into().unwrap(); 


    // for i in 0..vector.len().try_into().unwrap() {
    //     let start: i32 = 
    // }
    for (i, &radius) in vector.iter().enumerate() {
        intervals.push(
            ((i as i32-radius), i as i32+radius)
        );
    }

    // sorting the intervals vector by the first entry of each tuple 


    0
}

/**
 * Brackets solution 
 */
fn brackets_solution(S: &str) -> u8 {
    let mut brackets_stack = String::new();

    for c in S.chars() {
        match c {
            '{' | '(' | '[' => brackets_stack.push(c),
            '}' => {
                let popped_value = brackets_stack.pop().unwrap(); 
                if popped_value != '{' {
                    return 0
                }
            },             
            ')' => {
                let popped_value = brackets_stack.pop().unwrap(); 
                if popped_value != '(' {
                    return 0
                }
            }, 
            ']' => {
                let popped_value = brackets_stack.pop().unwrap(); 
                if popped_value != '[' {
                    return 0
                }
            }, 
            _ => println!("{:?}", c)
        }
    }

    1
}


/**
 * Fish
 */
fn fish_solution(arrayA: &Vec<u32>, arrayB: &Vec<u32>) -> usize {
    let mut number_of_fish = arrayA.len();

    let mut fish_stack: Vec<(u32, u32)> = Vec::new();
    fish_stack.push((arrayA[0], arrayB[0]));
    println!("{:?}", fish_stack);
    for fish in 1..arrayA.len() {
        // fish_stack.push((A[fish], B[fish]))
        println!("{:?}", fish_stack);

        let (prec_fishA, prec_fishB) = fish_stack.pop().unwrap(); 
        println!("prec fish is : {:?}, current is :{:?}", prec_fishB, arrayB[fish]);
        // Opposit direction to be eaten
        if prec_fishB > arrayB[fish] {
            number_of_fish -=1;
            println!("number of fish is {:?}", number_of_fish); 
            if prec_fishA > arrayA[fish] {
                fish_stack.push((prec_fishA, prec_fishB));
            }
            else {
                fish_stack.push((arrayA[fish], arrayB[fish]));
            }
        }
        else {
            fish_stack.push((arrayA[fish], arrayB[fish]));
        }


    }

    number_of_fish
}


/**
 * Nesting
 * 
 */
fn nesting_solution(S: &str) -> u8 {
    let mut stack_nested = String::new();

    for c in S.chars() {
        match c {
            '(' => stack_nested.push(c),
            ')' => {
                let last_element = stack_nested.pop();
                if last_element.unwrap() != '(' || last_element == None {
                    return 0
                }
            },
            _ => println!("{:?}", stack_nested)
        }
    }

    if stack_nested.len() == 0 {
        0
    }
    else {
        1
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
    // let A = vec![-1, -3];
    // println!("{:?}", missing_integer(&A));

    /*
        Count div
     */
    // let a = 6;
    // let b = 11;
    // let k = 2; 
    // println!("{:?}", count_div_solution(a, b, k));
    
    /*
        Distinct
     */
    // let A = vec![2, 1, 1, 2, 3, 1];
    // println!("{:?}", Distinct_solution(&A));

    /*
        Maxproductofthree
     */
    // let A = vec![-3, 1, 2, -2, 5, 6];
    // println!("{:?}", max_product_of_three_solution(&A));

    /*
        Brackets
     */
    // let S =  "{[()()]}"; 
    // // let mut ss = String::from(S);
    // println!("{:?}", brackets_solution(S));


    /*
        Fish
     */
    let A = vec![4, 3, 2, 1, 5];
    let B = vec![0, 1, 0, 0, 0];

    println!("{:?}", fish_solution(A.as_ref(), B.as_ref()));
}



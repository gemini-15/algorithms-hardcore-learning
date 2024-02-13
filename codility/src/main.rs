


/**
 * BinaryGap
 */
fn binarygap_solution(N: i32) -> Vec<i32> {
    // interating through the bits of N
    let mut vec_gaps : Vec<i32> = Vec::new();
    let mut res = (0..32).map (|n| (N >> n) & 1);
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


fn main() {
    /**
     * binary gap test
     */
    let gaps = binarygap_solution(32);
    println!("{:?}", gaps);
    let maxGap = gaps.iter().max();
    println!("{:?}", maxGap.unwrap());
}

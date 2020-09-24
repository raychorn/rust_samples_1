#![allow(non_snake_case)]

/*
    Find the sum of all multiples of 3 or 5 below 1000
*/


fn isMultiple(num: i32) -> bool {
    num % 5 == 0 || num % 3 == 0
}

fn main() {
    let mut sum_of_multiples = 0; 

    //loop from 0..999
    for i in (std::ops::Range { start: 0, end: 1000 }) {
        sum_of_multiples += 
            if isMultiple(i) {
                i
            }else{
                0
            };
    }
    println!("Sum is {}", sum_of_multiples);

}

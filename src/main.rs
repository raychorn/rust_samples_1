use std::time::{Instant};

/*
    Find the sum of all multiples of 3 or 5 below 1000
*/

type NumberType = i32;

fn sqrt(n: NumberType) -> f64 {

  let a: f64 = n as f64;// = float::from_str(int::str(n));
  let mut x: f64 = 1.0;
  let mut i = 0;

  while i < n {
    x = 0.5 * ( x + a / x );
    i += 1;
  }

  x
}


fn is_prime(n: NumberType) -> bool {

  let sroot: NumberType = sqrt(n) as NumberType;
  let mut i = 3;
  let mut rbool = true;

  if n == 1 || (n > 2 && n % 2 == 0) {
    false
  }
  else {

    while  i < sroot  {
      if n % i == 0 {
        rbool = false;
        break;
      }
      i += 2;
    }

    rbool
  }
}

fn is_multiple(num: NumberType) -> bool {
    // num % 5 == 0 || num % 3 == 0
    is_prime(num)
}

fn sum_of_vector(input: Vec<i32>) -> i32 {
    input.iter()
         .map(|&i| i)
         .sum()
}

fn main() {
    let start = Instant::now();
    let mut xs = vec![1i32];

    
    //loop from 0..999
    for i in (std::ops::Range { start: 0, end: 100000 }) {
        if is_multiple(i) {
            xs.push(i)
       }else{
            xs.push(0)
        };
    }
    let sum_of_multiples = sum_of_vector(xs);
    println!("Sum is {}", sum_of_multiples);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

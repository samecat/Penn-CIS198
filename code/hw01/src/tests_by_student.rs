#![cfg(test)]   //similar to include guards in C; only compile when testing

use crate::problem1::{sum, dedup, filter};
use crate::problem2::{mat_mult, Matrix};
use crate::problem3::sieve;
use crate::problem4::{hanoi, Peg};


// Start of Testing Problem 1
#[test]
fn test_prob1_sum(){
    assert!(sum(&[1,9]) == 10);
    assert!(sum(&[1,5,9]) == 15);
    assert!(sum(&[]) == 0);
    assert!(sum(&[1,-5,9]) == 5);
}

#[test]
fn test_prob1_dedup(){
	let v : Vec<i32> = vec![1,2,2,3,1,3,2];
	let w : Vec<i32> = vec![1,2,3];
	assert_eq!(w, dedup(&v));
}


fn number_greater_than_ten(x : i32) -> bool{
	x > 10
}

#[test]
fn test_prob1_filter(){
	let v : Vec<i32> = vec![11,9,12,8,10];
	assert_eq!(vec![11,12],filter(&v, &number_greater_than_ten));
}
// End of Testing Problem 1

// Start of Testing Problem 2
#[test]
fn test_prob2_mat_mult(){
	let m : Matrix = vec![vec![1.,2.,3.],vec![4.,5.,6.]];
	let n : Matrix = vec![vec![5.,0.],vec![6.,1.],vec![7.5,0.]];
	let p : Matrix = vec![vec![79./2.,2.],vec![95.,5.]];
	assert_eq!(p, mat_mult(&m,&n));
}
// End of Testing Problem 2

// Start of Testing Problem 3
#[test]
fn test_prob3_sieve(){
	assert_eq!(sieve(16),vec![2,3,5,7,11,13]);
}
// End of Testing Problem 3

// Start of Testing Problem 4
#[test]
fn test_prob4_hanoi(){
	assert_eq!(hanoi(3,Peg::A,Peg::B,Peg::C),vec![(Peg::A,Peg::C),(Peg::A,Peg::B),(Peg::C,Peg::B),(Peg::A,Peg::C),(Peg::B,Peg::A),(Peg::B,Peg::C),(Peg::A,Peg::C)]);
}
// End of Testing Problem 4
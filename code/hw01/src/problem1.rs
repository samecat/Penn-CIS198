//pub mod problem1 {

/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    // TODO
    //unimplemented!();
    let mut res : i32 = 0;
    for x in slice{
    	res += x;
    }
    res
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    // TODO
    //unimplemented!();
    let mut uv = Vec::new();
    for x in vs {
    	if !uv.contains(x) {uv.push(*x);} //contains expects &T, and push expects T; since vs is sent by reference, only push needs x to be dereferenced
    }
    uv
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    // TODO
    //unimplemented!();
    let mut uv = Vec::new();
    for x in vs {
    	if pred(*x) {uv.push(*x);}
    }
    uv
}


//}

//pub mod problem3 {
/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    // TODO
    //unimplemented!();
    let mut pv : Vec<Vec<u32>> = Vec::new();
    pv.push(vec![0,0]);
    pv.push(vec![1,0]);
    for i in 2..n{
    	pv.push(vec![i,1]);
    }

    
    for j in 2..pv.len(){
    	if pv[j][1] != 0 {
    		let mut k : usize = 2*j;
    		while k < n as usize {
    			pv[k][1] = 0;
				k += j;
    		}
    	}
    }

    let mut res : Vec<u32> = Vec::new();
    for z in 2..pv.len(){
    	if pv[z][1] == 1 {res.push(pv[z][0])};
    }
    res

}

//}

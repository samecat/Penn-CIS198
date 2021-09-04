//pub mod problem2 {

/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    // TODO
    //unimplemented!();
    let mut mat3 : Matrix = Vec::new();
    assert!(mat1[0].len() == mat2.len());
    for r in 0..mat1.len(){
    	let mut r_of_cees = Vec::new();
    	for c in 0..mat2[0].len(){
    		let mut c_ij : f32 = 0.0;
    		//println!("Computing c_ij {} {}", r,c);
    		for i in 0..mat1[r].len(){
    				//println!("mat1 {} {}",r,i);
    				//println!("mat2 {} {}",i,c);
    				c_ij += mat1[r][i]*mat2[i][c];
    		}
    		r_of_cees.push(c_ij);
    	}
    	mat3.push(r_of_cees);
    }

    //println!("{:?}",mat1);
    //println!("{:?}",mat2);
    //println!("{:?}",mat3);

    mat3
}


//}

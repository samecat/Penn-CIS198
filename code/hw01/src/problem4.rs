//pub mod problem4 {

/// #[derive(...)] statements define certain properties on the enum for you for
/// free (printing, equality testing, the ability to copy values). More on this
/// when we cover Enums in detail.

/// You can use any of the variants of the `Peg` enum by writing `Peg::B`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

/// Solves for the sequence of moves required to move all discs from `src` to
/// `dst`.
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    // TODO
    //unimplemented!();
    let mut moves : Vec<Move> = Vec::new();
        
    if num_discs >= 1 {
	    //move n-1 discs from source to spare (aux)
	    moves = hanoi(num_discs-1,src,dst,aux);
	    
	    //move last disc to target (dst)
	    let a_move : Move = (src,dst);
	    moves.push(a_move);    
	    
	    //move n-1 discs from spare to target
	    let mut remaining_moves : Vec<Move> = hanoi(num_discs-1,aux,src,dst);

	    //Merge moves vectors before return
	    moves.append(&mut remaining_moves);

	    moves
	} else {
		moves
	}
}

//}

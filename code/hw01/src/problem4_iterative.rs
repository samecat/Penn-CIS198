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
    let mut pegs = Vec::new();
    
	for t in 0..3 {
	    let mut a_peg = Vec::new();
    	pegs.push(a_peg);
    }

    println!("num_disc is {}", num_discs);

    for x in (1..num_discs+1).rev(){
		pegs[0].push(x);
    }

    println!("Tower state before: {:?}",pegs);

	let mut debug_moves : Vec<Move> = Vec::new();

    let mut moves : Vec<Move> = Vec::new();
    let mut first_move : Move = (Peg::A, Peg::A);
    let a_disc : u32 = pegs[0].pop().expect("Oh oh: No discs to pop!");
    if num_discs % 2 == 0 {  //if even, first move is peg A to peg B
    	pegs[1].push(a_disc);
    	first_move = (Peg::A, Peg::B);
    } else {  //if odd, first move is peg A to peg C
    	pegs[2].push(a_disc);
    	first_move = (Peg::A, Peg::C);
    }
    moves.push(first_move);

    println!("Tower state after: {:?}",pegs);
    println!("Moves: {:?}", moves);

    let mut last_moved = 1;
    while pegs[2].len() != num_discs {
    	if pegs[0][pegs[0].len()-1] != last_moved{
    		
    		last_moved = pegs[0][pegs[0].len()-1];
    	}
    	if pegs[1][pegs[0].len()-1] != last_moved{
    		last_moved = pegs[1][pegs[0].len()-1];
    	}
    	if pegs[2][pegs[0].len()-1] != last_moved{
    		last_moved = pegs[2][pegs[0].len()-1];
    	}
    }

    //moves
    debug_moves

}

//}

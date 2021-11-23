mod othello_board;
mod othello_state;
mod othello_hash;

use crate::othello_state::OthelloState;

fn main() {
	
	let initial_state = OthelloState::starting_state();
	
	let moves = initial_state.available_moves();
	
	println!("{}", initial_state.apply_move(moves[0]));
	
}

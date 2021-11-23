mod othello_board;
mod othello_state;
mod othello_hash;
mod heuristic;

use crate::heuristic::heuristic_fast;
use crate::othello_state::OthelloState;

fn main() {
	
	let mut initial_state = OthelloState::starting_state();
	
	let moves = initial_state.available_moves();
	
	println!("{}", initial_state.apply_move(moves[0]));
	
	initial_state = initial_state.apply_move(moves[0]);
	
	println!("eval={}", heuristic_fast(initial_state.ply(), initial_state.black(), initial_state.white()));
	
}

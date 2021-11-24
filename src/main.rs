mod othello_board;
mod othello_state;
mod othello_hash;
mod heuristic;
mod endgame;

use crate::endgame::{solve_endgame_mo, solve_endgame_nomo};
use crate::heuristic::heuristic_minimax;
use crate::othello_state::OthelloState;

fn main() {
	
	let mut initial_state = OthelloState::starting_state();
	
	let moves = initial_state.available_moves();
	
	println!("{}", initial_state.apply_move(moves[0]));
	
	initial_state = initial_state.apply_move(moves[0]);
	
	println!("eval={}", heuristic_minimax(initial_state.ply(), initial_state.black(), initial_state.white()));
	
	let state = OthelloState::new(42, 9241634274023193656u64, 4485055359014469632u64, 0);
	println!("{}", state);
	
	let q = solve_endgame_mo(state.black(), state.white(), -1, 1, 7);
	println!("{}. score={}", 0, q);
	
}

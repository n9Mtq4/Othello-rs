mod othello_board;
mod othello_state;
mod othello_hash;
mod heuristic;
mod endgame;

use crate::endgame::solve_endgame_root;
use crate::heuristic::{heuristic_eg_slow_nega, heuristic_minimax};
use crate::othello_state::OthelloState;

fn main() {
	
	let mut initial_state = OthelloState::starting_state();
	
	let moves = initial_state.available_moves();
	
	println!("{}", initial_state.apply_move(moves[0]));
	
	initial_state = initial_state.apply_move(moves[0]);
	
	println!("eval={}", heuristic_minimax(initial_state.ply(), initial_state.black(), initial_state.white()));
	
	let state = OthelloState::new(40, 9241636472995985464u64, 4484490210071479296u64, 0);
	println!("{}", state);
	
	println!("guessed score={}", heuristic_eg_slow_nega(state.black(), state.white()));
	let (mov, q) = solve_endgame_root(state.black(), state.white(), 0, 2);
	println!("{}. move={}, score={}", 0, mov, q);
	
}

mod othello_board;
mod othello_state;
mod othello_hash;
mod endgame;

use crate::endgame::solve_endgame_root;
use crate::othello_state::OthelloState;

fn main() {
	
	let state = OthelloState::new(40, 9241636472995985464u64, 4484490210071479296u64, 0);
	println!("{}", state);
	
	let (mov, q) = solve_endgame_root(state.black(), state.white(), 0, 2);
	println!("{}. move={}, score={}", 0, mov, q);
	
}

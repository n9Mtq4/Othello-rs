mod othello_board;
mod othello_state;
mod othello_hash;
mod endgame;
mod neural_heuristic;
mod neural_search;

use tch::Tensor;
use crate::endgame::solve_endgame_root;
use crate::neural_search::{nnsearch_mo, nnsearch_nomo};
use crate::othello_state::OthelloState;

fn main() {
	
	let state = OthelloState::new(40, 9241636472995985464u64, 4484490210071479296u64, 0);
	println!("{}", state);
	
	// let (mov, q) = solve_endgame_root(state.black(), state.white(), -100, 100);
	// println!("{}. move={}, score={}", 0, mov, q);
	
	let model = tch::CModule::load("data/model.pt").expect("loading model failed");
	
	let prediction = nnsearch_mo(&model, state.black(), state.white(), -100000, 100000, 5, 3);
	println!("{}", prediction);
	
}

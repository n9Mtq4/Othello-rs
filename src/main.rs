mod othello_board;
mod othello_state;
mod endgame;
mod neural_heuristic;
mod neural_search;
mod server;
mod opening_book;
mod othello_symmetry;
mod consts;

use crate::endgame::solve_endgame_root;
use crate::server::server_start;

fn main() {
	
	// print out compiled features
	#[cfg(feature = "gpu")] {
		println!("Feature \"gpu\" enabled");
	}
	#[cfg(feature = "large_batch")] {
		println!("Feature \"large_batch\" enabled");
	}
	
	let (mov, q) = solve_endgame_root(9241636472995985464, 4484490210071479296, -127, 127);
	println!("{}, {}", mov, q);
	
	// let mut port = 35326u16;
	// 
	// // try to set port from CLI args
	// let mut args = std::env::args();
	// if args.len() > 1 {
	// 	port = args.nth(1)
	// 		.unwrap()
	// 		.parse()
	// 		.expect("provided argument isn't a valid port number");
	// }
	// 
	// // start the server
	// tch::no_grad(|| {
	// 	server_start(port);
	// });
	
}

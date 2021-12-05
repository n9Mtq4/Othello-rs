mod othello_board;
mod othello_state;
mod othello_hash;
mod endgame;
mod neural_heuristic;
mod neural_search;
mod server;
mod opening_book;
mod othello_symmetry;

use crate::server::server_start;

fn main() {
	
	// print out compiled features
	#[cfg(feature = "gpu")] {
		println!("Feature \"gpu\" enabled");
	}
	#[cfg(feature = "large_batch")] {
		println!("Feature \"large_batch\" enabled");
	}
	
	let mut port = 35326u16;
	
	// try to set port from CLI args
	let mut args = std::env::args();
	if args.len() > 1 {
		port = args.nth(1)
			.unwrap()
			.parse()
			.expect("provided argument isn't a valid port number");
	}
	
	// start the server
	server_start(port);
	
}

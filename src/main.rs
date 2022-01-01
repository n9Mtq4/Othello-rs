mod othello_board;
mod othello_state;
mod endgame;
mod server;
mod opening_book;
mod othello_symmetry;
mod classic_weights;
mod classic_search;

use crate::server::server_start;

fn main() {
	
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

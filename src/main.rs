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
	
	server_start(35326u16);
	
}

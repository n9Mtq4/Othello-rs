#![allow(unused_imports)]

use std::cmp::max;
use std::fmt::{Display, Formatter};
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use byteorder::{NetworkEndian, WriteBytesExt};
use tch::{CModule, Device, Kind};
use crate::neural_search::nnsearch_root;
use crate::othello_board::{empty_disks, evaluation, game_over, generate_moves};
use crate::endgame::solve_endgame_root;
use crate::opening_book::{OthelloBook, read_book, search_book};

struct SearchParams {
	/// If true will use remaining time to adjust settings
	adj_time: bool,
	/// If true will use the opening book
	use_book: bool,
	/// If true will solve exact endgame, If false solves WLD
	solve_end_exact: bool,
	/// Mid game (nn_search) search depth
	mid_depth: u8,
	/// Endgame search depth
	end_depth: u8
}

impl Display for SearchParams {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "SearchParams(utime={}, book={}, exact={}, mid_d={}, end_d={})",
		       self.adj_time, self.use_book, self.solve_end_exact, self.mid_depth, self.end_depth)
	}
}

impl SearchParams {
	
	/// Decodes algorithm parameters requested from client
	/// p: u16 with layout _TBSDDDDDDEEEEEE
	/// T - bit to adjust based on time (1 = adjust params to fit in remaining time, 0 = ignore remaining time)
	/// B - bit to use the opening book (1 = use book, 0 = no book)
	/// S - bit to solve exact endgame (1 = exact, 0 = WLD)
	/// D - 6 bits for neural network depth (0-63)
	/// E - 6 bits for endgame depth (0-63)
	fn from_u16(p: u16) -> Self {
		
		let mut end_depth = ((p >> 0) & 0b111111) as u8;
		let mut mid_depth = ((p >> 6) & 0b111111) as u8;
		let solve_end_exact = ((p >> 12) & 0b1) != 0;
		let use_book = ((p >> 13) & 0b1) != 0;
		let adj_time = ((p >> 14) & 0b1) != 0;
		
		// ensure not too deep
		end_depth = end_depth.clamp(1, 22);
		mid_depth = mid_depth.clamp(1, 10);
		
		SearchParams {
			adj_time,
			use_book,
			solve_end_exact,
			mid_depth,
			end_depth
		}
		
	}
	
	/// Gets the adjusted depth to pass to midgame search
	/// for a true depth of mid_depth
	fn adjusted_mid_depth(&self) -> u8 {
		if cfg!(feature = "large_batch") {
			max(1 + 3, self.mid_depth) - 3
		} else {
			max(1 + 1, self.mid_depth) - 1
		}
	}
	
	/// Gets the window size for the endgame alphabeta search
	/// If solve_end_exact is true, sets to 100, to find exact in -64 to 64
	/// If solve_end_exact is false, sets to 1 for WLD
	fn end_window(&self) -> i8 {
		if self.solve_end_exact {
			100
		} else {
			1
		}
	}
	
}

/// Returns (best_move, centidisk_score) for the given position
/// Performs search according to search params
fn server_get_move(book: &OthelloBook, model: &CModule, me: u64, enemy: u64, params: &SearchParams) -> (u8, i16) {
	
	// if the game is over, return the evaluation
	if game_over(me, enemy) {
		return (65, 100 * (evaluation(me, enemy) as i16));
	}
	
	// if there are no moves, pass
	if generate_moves(me, enemy) == 0 {
		return (65, i16::MAX);
	}
	
	// if there are <= 18 disks left, solve the endgame
	if empty_disks(me, enemy) <= params.end_depth {
		let (mov, q) = solve_endgame_root(me, enemy, -params.end_window(), params.end_window());
		return (mov, 100 * (q as i16));
	}
	
	// try the opening book
	if params.use_book {
		if let Some(res) = search_book(book, me, enemy) {
			return res;
		}
	}
	
	// otherwise perform a negamax neural network search
	let (mov, q) = nnsearch_root(model, me, enemy, -640000, 640000, params.adjusted_mid_depth() as i8);
	(mov, q as i16)
	
}

/// Handle client
/// Protocol expects 20 bytes
/// me: u64, enemy: u64, time: u16, params: u16
/// me: u64 - the bitboard for current player
/// enemy: u64 - the bitboard for enemy player
/// time: u16 - the remaining time for the game in 10ths of second
/// params: u16 - algorithm params
/// params: _TBSDDDDDDEEEEEE
/// T - bit to adjust based on time (1 = adjust params to fit in remaining time)
/// B - bit to use the opening book (1 = use book)
/// S - bit to solve exact endgame (1 = exact, 0 = WLD)
/// D - 6 bits for neural network depth (0-63)
/// E - 6 bits for endgame depth
fn server_handle_client(book: &OthelloBook, model: &CModule, mut stream: TcpStream) {
	
	const PROT_SIZE: usize = 8 + 8 + 2 + 2;
	
	// data from client goes here
	let mut data = [0 as u8; PROT_SIZE];
	
	// read exactly 20 bytes from client
	let mut buf = [0 as u8; PROT_SIZE];
	let mut read_bytes = 0;
	while read_bytes < PROT_SIZE {
		
		// read bytes into buffer
		let read = stream.read(&mut buf)
			.unwrap_or_else(|_| { println!("Failed to read from client"); 0 });
		
		// copy bytes from buffer into data
		for i in 0..read {
			data[i + read_bytes] = buf[i];
		}
		
		read_bytes += read;
		
	}
	
	// parse client data
	let me = u64::from_be_bytes(data[0..8].try_into().unwrap());
	let enemy = u64::from_be_bytes(data[8..16].try_into().unwrap());
	let time = u16::from_be_bytes(data[16..18].try_into().unwrap());
	let params = u16::from_be_bytes(data[18..20].try_into().unwrap());
	
	let search_params = SearchParams::from_u16(params);
	
	// TODO: if search_params.adj_time, change based on time
	
	// evaluate position
	let before = Instant::now();
	let (mov, q) = server_get_move(book, model, me, enemy, &search_params);
	let after = Instant::now();
	
	let ms = (after - before).as_millis();
	
	println!("me={}, e={}, rt={}, mov={}, q={}, t={}ms, p={}", me, enemy, time, mov, q, ms, search_params);
	
	// return best move to client
	stream.write_u8(mov).unwrap_or_else(|_| { println!("Failed to write mov to socket"); });
	stream.write_i16::<NetworkEndian>(q).unwrap_or_else(|_| { println!("Failed to write q to socket"); });
	stream.flush().unwrap_or_else(|_| { println!("Failed to flush socket"); });
	stream.shutdown(Shutdown::Both).unwrap_or_else(|_| { println!("Failed to close socket"); });
	
}

#[allow(unused_mut)]
pub fn server_start(port: u16) {
	
	println!("Starting server...");
	
	// load opening book
	// TODO: since we only read from the book, a lock isn't needed, although may be good to add RwLock anyway
	let book = Arc::new(read_book("data/book.bin"));
	println!("Loaded {} positions into book", book.len());
	
	// load pytorch model
	let mut model = tch::CModule::load("data/model.pt")
		.unwrap();
	
	// move to the GPU
	#[cfg(feature = "gpu")] {
		println!("Moving model to GPU...");
		model.to(Device::Cuda(0), Kind::Float, false);
	}
	
	model.set_eval();
	
	let model = Arc::new(Mutex::new(model));
	
	// count network parameters for nice log message
	let num_params = model.lock().unwrap().named_parameters().unwrap()
		.iter()
		.map(|(_, t)| t.size().into_iter().reduce(|a, b| a * b).unwrap())
		.reduce(|a, b| a + b)
		.unwrap();
	println!("Loaded nn heuristic model with {} params", num_params);
	
	// start listening on localhost:port
	let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
	println!("Server listening on port {}", port);
	
	// accept connections and process them, spawning a new thread for each one
	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				let my_book = book.clone();
				let my_model = model.clone();
				thread::spawn(move || {
					// TODO: only lock model when using it (now locks even for endgame search)
					server_handle_client(&my_book, &my_model.lock().unwrap(), stream);
				});
			}
			Err(e) => {
				println!("Error: {}", e);
			}
		}
	}
	
	// close the socket server
	drop(listener);
	
}

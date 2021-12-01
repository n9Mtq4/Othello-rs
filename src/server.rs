use std::borrow::Borrow;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str;
use std::sync::{Arc, Mutex};
use tch::CModule;
use crate::neural_search::nnsearch_root;
use crate::othello_board::{empty_disks, generate_moves};
use crate::endgame::solve_endgame_root;
use crate::opening_book::{OthelloBook, read_book, search_book};

fn server_get_move(book: &OthelloBook, model: &CModule, me: u64, enemy: u64) -> u8 {
	
	// if there are no moves, pass
	if generate_moves(me, enemy) == 0 {
		return 65;
	}
	
	// if there are <= 18 disks left, solve the endgame
	if empty_disks(me, enemy) <= 18 {
		return solve_endgame_root(me, enemy, -100, 100).0;
	}
	
	// try the opening book
	let book_move = search_book(book, me, enemy);
	if book_move < 64 {
		return book_move;
	}
	
	// otherwise perform a negamax neural network search
	return nnsearch_root(model, me, enemy, -640000, 640000, 5).0;
	
}

fn server_handle_client(book: &OthelloBook, model: &CModule, mut stream: TcpStream) {
	
	// read data from client
	let mut data = [0 as u8; 128];
	let read_bytes = stream.read(&mut data)
		.unwrap_or_else(|_| { println!("Failed to read position"); 0 });
	
	if read_bytes == 0 { return; }
	
	let s = str::from_utf8(&data[..read_bytes]).unwrap();
	
	// parse client data
	let tokens: Vec<&str> = s.trim().split(",").collect();
	
	let me = match tokens[0].parse() {
		Ok(value) => value,
		Err(e) => {
			println!("Failed to parse position: {}", s);
			return;
		}
	};
	let enemy = match tokens[1].parse() {
		Ok(value) => value,
		Err(e) => {
			println!("Failed to parse position: {}", s);
			return;
		}
	};
	
	// evaluate position
	let mov = server_get_move(book, model, me, enemy);
	println!("me={}, enemy={}, best_move={}", me, enemy, mov);
	
	// return best move to client
	stream.write(format!("{}\n", mov).as_bytes())
		.unwrap_or_else(|_| { println!("Failed to write to socket"); 0 });
	stream.flush()
		.unwrap_or_else(|_| { println!("Failed to flush socket"); });
	stream.shutdown(Shutdown::Both)
		.unwrap_or_else(|_| { println!("Failed to close socket"); });
	
}

pub fn server_start(port: u16) {
	
	println!("Starting server...");
	
	// load opening book
	// TODO: since we only read from the book, a lock isn't needed, although may be good to add RwLock anyway
	let book = Arc::new(read_book("data/book.bin"));
	println!("Loaded {} positions into book", book.len());
	
	// load pytorch model
	let model = Arc::new(Mutex::new(tch::CModule::load("data/model.pt")
		.expect("loading model failed")));
	
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

use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str;
use tch::CModule;
use crate::neural_search::nnsearch_root;
use crate::othello_board::empty_disks;
use crate::solve_endgame_root;

fn server_get_move(model: &CModule, me: u64, enemy: u64) -> u8 {
	
	if empty_disks(me, enemy) <= 18 {
		return solve_endgame_root(me, enemy, -100, 100).0;
	}
	
	return nnsearch_root(model, me, enemy, -640000, 640000, 5).0;
	
}

fn server_handle_client(model: &CModule, mut stream: TcpStream) {
	
	// read data from client
	let mut data = [0 as u8; 128];
	let read_bytes = stream.read(&mut data).unwrap();
	let s = str::from_utf8(&data[..read_bytes]).unwrap();
	
	// parse client data
	let tokens: Vec<&str> = s.trim().split(",").collect();
	let me: u64 = tokens[0].parse().unwrap();
	let enemy: u64 = tokens[1].parse().unwrap();
	
	// evaluate position
	let mov = server_get_move(model, me, enemy);
	println!("me={}, enemy={}, best_move={}", me, enemy, mov);
	
	// return best move to client
	stream.write(mov.to_string().as_bytes()).unwrap();
	stream.flush();
	stream.shutdown(Shutdown::Both).unwrap();
	
}

pub fn server_start() {
	
	let model = tch::CModule::load("data/model.pt").expect("loading model failed");
	
	let listener = TcpListener::bind("0.0.0.0:35326").unwrap();
	// accept connections and process them, spawning a new thread for each one
	println!("Server listening on port 35326");
	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				// println!("New connection: {}", stream.peer_addr().unwrap());
				// thread::spawn(move|| {
				// TODO: spawn thread & share model
				server_handle_client(&model, stream);
				// });
			}
			Err(e) => {
				println!("Error: {}", e);
				/* connection failed */
			}
		}
	}
	// close the socket server
	drop(listener);
	
}

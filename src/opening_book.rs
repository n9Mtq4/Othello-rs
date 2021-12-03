use std::collections::HashMap;
use std::fs::File;
use byteorder::{LittleEndian, ReadBytesExt};
use crate::othello_symmetry::{sym_invert_loc, sym_min_board};

#[derive(PartialEq, Eq, Hash)]
pub struct OthelloBookKey {
	me: u64,
	enemy: u64
}

pub struct OthelloBookValue {
	/// the best move to make in the position
	best_move: u8,
	/// the evaluation of the position in 1/2 disks
	eval: i8
}

pub type OthelloBook = HashMap<OthelloBookKey, OthelloBookValue>;

/// searches book for position & returns move, centidisk eval
pub fn search_book(book: &OthelloBook, me: u64, enemy: u64) -> Option<(u8, i16)> {
	
	// find min symmetry of the board
	let (m, e, transform) = sym_min_board(me, enemy);
	
	let key = OthelloBookKey {
		me: m,
		enemy: e
	};
	
	// search book for the min board
	// if found, invert the min sym transformation
	match book.get(&key) {
		Some(mq) => Some((sym_invert_loc(transform, mq.best_move), 200 * (mq.eval as i16))),
		None => None
	}
	
}

pub fn read_book(file_name: &str) -> OthelloBook {
	
	let mut book: OthelloBook = HashMap::new();
	
	let mut file = File::open(&file_name).expect("Error opening book file");
	
	loop {
		
		// try reading an entry
		let me = match file.read_u64::<LittleEndian>() {
			Ok(res) => res,
			Err(_) => break
		};
		
		// if the first read succeeded, there must be another 8 + 1 + 1 bytes
		let enemy = file.read_u64::<LittleEndian>().unwrap();
		let best_move = file.read_u8().unwrap();
		let eval = file.read_i8().unwrap();
		
		// add the state to the book
		let key = OthelloBookKey {
			me,
			enemy
		};
		let value = OthelloBookValue {
			best_move,
			eval
		};
		
		book.insert(key, value);
		
	}
	
	return book;
	
}

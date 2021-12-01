use std::collections::HashMap;
use std::fs::File;
use byteorder::{LittleEndian, ReadBytesExt};
use crate::othello_symmetry::{sym_invert_loc, sym_min_board};

#[derive(PartialEq, Eq, Hash)]
pub struct OthelloStateKey {
	me: u64,
	enemy: u64
}

pub type OthelloBook = HashMap<OthelloStateKey, u8>;

// impl PartialEq<Self> for OthelloStateKey {
// 	fn eq(&self, other: &Self) -> bool {
// 		(self.me == other.me) && (self.enemy == other.enemy)
// 	}
// }
// 
// impl Eq for OthelloStateKey {
// 	
// }
// 
// impl Hash for OthelloStateKey {
// 	fn hash<H: Hasher>(&self, state: &mut H) {
// 		self.me.hash(state);
// 		self.enemy.hash(state);
// 	}
// }

/// searches book for position & returns move
/// if position wasn't found, return 65
pub fn search_book(book: &OthelloBook, me: u64, enemy: u64) -> u8 {
	
	// find min symmetry of the board
	let (m, e, transform) = sym_min_board(me, enemy);
	
	let key = OthelloStateKey {
		me: m,
		enemy: e
	};
	
	// search book for the min board
	// if found, invert the min sym transformation
	match book.get(&key) {
		Some(best_move) => sym_invert_loc(transform, *best_move),
		None => 65
	}
	
}

pub fn read_book(file_name: &str) -> OthelloBook {
	
	let mut book: HashMap<OthelloStateKey, u8> = HashMap::new();
	
	let mut file = File::open(&file_name).expect("Error opening book file");
	
	loop {
		
		// try reading an entry
		let me = match file.read_u64::<LittleEndian>() {
			Ok(res) => res,
			Err(_) => break
		};
		
		// if the first read succeeded, the next two must also work
		let enemy = file.read_u64::<LittleEndian>().unwrap();
		let best_move = file.read_u8().unwrap();
		
		// add the state to the book
		let key = OthelloStateKey {
			me,
			enemy
		};
		
		book.insert(key, best_move);
		
	}
	
	return book;
	
}

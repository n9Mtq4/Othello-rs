use std::fmt::{Display, Formatter};
use crate::othello_board::{game_over, generate_moves, make_move, to_idx_move_vec};
use crate::othello_hash::KEYS;

pub struct OthelloState {
	
	ply: u8,
	black: u64,
	white: u64,
	hash: u32
	
}

impl OthelloState {
	
	pub fn new(ply: u8, black: u64, white: u64, hash: u32) -> Self {
		OthelloState { ply, black, white, hash }
	}
	
	pub fn starting_state() -> Self {
		OthelloState {
			ply: 0,
			black: 0b00000000_00000000_00000000_00001000_00010000_00000000_00000000_00000000u64,
			white: 0b00000000_00000000_00000000_00010000_00001000_00000000_00000000_00000000u64,
			hash: 0
		}
	}
	
	pub fn player_coeff(&self) -> i8 {
		if self.ply & 1 == 0 { 1 } else { -1 }
	}
	
	#[inline(always)]
	pub fn num_black_disks(&self) -> u8 {
		self.black.count_ones() as u8
	}
	
	#[inline(always)]
	pub fn num_white_disks(&self) -> u8 {
		self.white.count_ones() as u8
	}
	
	#[inline(always)]
	pub fn disks_placed(&self) -> u8 {
		(self.black | self.white).count_ones() as u8
	}
	
	#[inline(always)]
	pub fn empty_disks(&self) -> u8 {
		64 - self.disks_placed()
	}
	
	#[inline(always)]
	pub fn game_over(&self) -> bool {
		game_over(self.black, self.white)
	}
	
	pub fn has_move(&self) -> bool {
		(if self.ply & 1 == 0 { generate_moves(self.black, self.white) } else { generate_moves(self.white, self.black) }) != 0
	}
	
	#[inline(always)]
	pub fn evaluation(&self) -> i8 {
		(self.num_black_disks() as i8) - (self.num_white_disks() as i8)
	}
	
	pub fn winner(&self) -> i8 {
		self.evaluation().signum()
	}
	
	pub fn pass(&self) -> Self {
		Self::new(self.ply + 1, self.black, self.white, self.hash ^ KEYS[self.ply as usize][64])
	}
	
	pub fn apply_move(&self, mov: u8) -> Self {
		if self.ply & 1 == 0 {
			let (me, enemy) = make_move(1u64 << mov, self.black, self.white);
			Self::new(self.ply + 1, me, enemy, self.hash ^ KEYS[self.ply as usize][mov as usize])
		} else {
			let (me, enemy) = make_move(1u64 << mov, self.white, self.black);
			Self::new(self.ply + 1, enemy, me, self.hash ^ KEYS[self.ply as usize][mov as usize])
		}
	}
	
	pub fn number_of_moves(&self) -> u8 {
		(if self.ply & 1 == 0 { generate_moves(self.black, self.white) } else { generate_moves(self.white, self.black) }).count_ones() as u8
	}
	
	pub fn available_moves(&self) -> Vec<u8> {
		to_idx_move_vec(if self.ply & 1 == 0 { generate_moves(self.black, self.white) } else { generate_moves(self.white, self.black) })
	}
	
	pub fn to_short_string(&self) -> String {
		format!("{},{},{},{}", self.ply, self.black, self.white, self.hash)
	}
	
}

impl Display for OthelloState {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for i in 0..64 {
			if i % 8 == 0 { write!(f, "\n"); }
			if self.black & (1u64 << i) != 0 { write!(f, "O "); }
			else if self.white & (1u64 << i) != 0 { write!(f, "@ "); }
			else { write!(f, "  "); }
		}
		write!(f, "\nply={}, black={}, white={}, hash={}", self.ply, self.black, self.white, self.hash)
	}
}

impl PartialEq for OthelloState {
	fn eq(&self, other: &Self) -> bool {
		(self.ply & 1) == (other.ply & 1) &&
			self.black == other.black &&
			self.white == other.white
	}
}

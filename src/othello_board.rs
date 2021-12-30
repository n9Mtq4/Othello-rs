#![allow(dead_code)]

use bitintr::{Blsi, Blsr};

const MASK_E: u64 = 0b11111110_11111110_11111110_11111110_11111110_11111110_11111110_11111110u64;
const MASK_W: u64 = 0b01111111_01111111_01111111_01111111_01111111_01111111_01111111_01111111u64;

#[inline(always)]
fn shift_n(bb: u64) -> u64 {
	return bb << 8;
}

#[inline(always)]
fn shift_s(bb: u64) -> u64 {
	return bb >> 8;
}

#[inline(always)]
fn shift_e(bb: u64) -> u64 {
	return (bb & MASK_E) >> 1;
}

#[inline(always)]
fn shift_w(bb: u64) -> u64 {
	return (bb & MASK_W) << 1;
}

#[inline(always)]
fn shift_nw(bb: u64) -> u64 {
	return shift_n(shift_w(bb));
}

#[inline(always)]
fn shift_ne(bb: u64) -> u64 {
	return shift_n(shift_e(bb));
}

#[inline(always)]
fn shift_sw(bb: u64) -> u64 {
	return shift_s(shift_w(bb));
}

#[inline(always)]
fn shift_se(bb: u64) -> u64 {
	return shift_s(shift_e(bb));
}

pub fn generate_moves(bb_self: u64, bb_enemy: u64) -> u64 {
	
	let open: u64 = !(bb_self | bb_enemy);
	let mut moves: u64 = 0;
	let mut captured: u64;
	
	captured = shift_n(bb_self) & bb_enemy;
	for _ in 0..5 {
		captured |= shift_n(captured) & bb_enemy;
	}
	moves |= shift_n(captured) & open;
	
	captured = shift_s(bb_self) & bb_enemy;
	for _ in 0..5 {
		captured |= shift_s(captured) & bb_enemy;
	}
	moves |= shift_s(captured) & open;
	
	captured = shift_w(bb_self) & bb_enemy;
	for _ in 0..5 {
		captured |= shift_w(captured) & bb_enemy;
	}
	moves |= shift_w(captured) & open;
	
	captured = shift_e(bb_self) & bb_enemy;
	for _ in 0..5 {
		captured |= shift_e(captured) & bb_enemy;
	}
	moves |= shift_e(captured) & open;
	
	captured = shift_nw(bb_self) & bb_enemy;
	for _ in 0..5 {
		captured |= shift_nw(captured) & bb_enemy;
	}
	moves |= shift_nw(captured) & open;
	
	captured = shift_ne(bb_self) & bb_enemy;
	for _ in 0..5 {
		captured |= shift_ne(captured) & bb_enemy;
	}
	moves |= shift_ne(captured) & open;
	
	captured = shift_sw(bb_self) & bb_enemy;
	for _ in 0..5 {
		captured |= shift_sw(captured) & bb_enemy;
	}
	moves |= shift_sw(captured) & open;
	
	captured = shift_se(bb_self) & bb_enemy;
	for _ in 0..5 {
		captured |= shift_se(captured) & bb_enemy;
	}
	moves |= shift_se(captured) & open;
	
	return moves;
	
}

/// Gets the next mov bit mask (1 << mov_idx) from moves
/// Removes it as a move from moves
#[inline(always)]
pub fn next_bit_move(moves: &mut u64) -> u64 {
	// the following 3 lines work without bitintr, but are slightly slower
	// let i = moves.trailing_zeros();
	// let mov = 1 << i;
	// *moves &= !mov;
	let mov = moves.blsi();
	*moves = moves.blsr();
	return mov;
}

/// Gets the next mov idx from moves
/// Removes it as a move from moves
#[inline(always)]
pub fn next_idx_move(moves: &mut u64) -> u8 {
	// the following 3 lines work without bitintr, but are slightly slower
	// let i = moves.trailing_zeros();
	// let mov = 1 << i;
	// *moves &= !mov;
	let i = moves.trailing_zeros() as u8;
	*moves = moves.blsr();
	return i;
}

#[inline(always)]
pub fn number_of_moves(bb_self: u64, bb_enemy: u64) -> u8 {
	return generate_moves(bb_self, bb_enemy).count_ones() as u8;
}

pub fn make_move(mov: u64, mut bb_self: u64, mut bb_enemy: u64) -> (u64, u64) {
	make_move_inplace(mov, &mut bb_self, &mut bb_enemy);
	return (bb_self, bb_enemy);
}

pub fn make_move_inplace(mov: u64, bb_self: &mut u64, bb_enemy: &mut u64) {
	
	// TODO: Optimize. in endgame search, 46% of the time is spent here
	// for each of 64 disks, have 8 direction masks. use pext to get ray
	// use blcmsk to find which disks to flip
	// test to make sure we flank. then use pdep to make flip mask
	// also investigate SSE method https://github.com/abulmo/edax-reversi/blob/master/src/flip_sse.c
	
	let mut captured;
	let mut flips = 0u64;
	
	*bb_self |= mov;
	
	captured = shift_n(mov) & *bb_enemy;
	for _ in 0..5 {
		captured |= shift_n(captured) & *bb_enemy;
	}
	if shift_n(captured) & *bb_self != 0 {
		flips |= captured;
	}
	
	captured = shift_s(mov) & *bb_enemy;
	for _ in 0..5 {
		captured |= shift_s(captured) & *bb_enemy;
	}
	if shift_s(captured) & *bb_self != 0 {
		flips |= captured;
	}
	
	captured = shift_w(mov) & *bb_enemy;
	for _ in 0..5 {
		captured |= shift_w(captured) & *bb_enemy;
	}
	if shift_w(captured) & *bb_self != 0 {
		flips |= captured;
	}
	
	captured = shift_e(mov) & *bb_enemy;
	for _ in 0..5 {
		captured |= shift_e(captured) & *bb_enemy;
	}
	if shift_e(captured) & *bb_self != 0 {
		flips |= captured;
	}
	
	captured = shift_nw(mov) & *bb_enemy;
	for _ in 0..5 {
		captured |= shift_nw(captured) & *bb_enemy;
	}
	if shift_nw(captured) & *bb_self != 0 {
		flips |= captured;
	}
	
	captured = shift_ne(mov) & *bb_enemy;
	for _ in 0..5 {
		captured |= shift_ne(captured) & *bb_enemy;
	}
	if shift_ne(captured) & *bb_self != 0 {
		flips |= captured;
	}
	
	captured = shift_sw(mov) & *bb_enemy;
	for _ in 0..5 {
		captured |= shift_sw(captured) & *bb_enemy;
	}
	if shift_sw(captured) & *bb_self != 0 {
		flips |= captured;
	}
	
	captured = shift_se(mov) & *bb_enemy;
	for _ in 0..5 {
		captured |= shift_se(captured) & *bb_enemy;
	}
	if shift_se(captured) & *bb_self != 0 {
		flips |= captured;
	}
	
	*bb_self |= flips;
	*bb_enemy &= !flips;
	
}

pub fn game_over(bb_p1: u64, bb_p2: u64) -> bool {
	return (bb_p1 | bb_p2 == u64::MAX) ||
		(generate_moves(bb_p1, bb_p2) == 0 &&
			generate_moves(bb_p2, bb_p1) == 0);
}

#[inline(always)]
pub fn evaluation(black: u64, white: u64) -> i8 {
	(black.count_ones() as i8) - (white.count_ones() as i8)
}

#[inline(always)]
pub fn empty_disks(black: u64, white: u64) -> u8 {
	(!(black | white)).count_ones() as u8
}

pub fn to_idx_move_vec(mut moves: u64) -> Vec<u8> {
	
	let num_moves: usize = moves.count_ones() as usize;
	let mut vec = Vec::with_capacity(num_moves);
	
	while moves != 0 {
		vec.push(next_idx_move(&mut moves));
	}
	
	return vec;
	
}

pub fn to_bit_move_vec(mut moves: u64) -> Vec<u64> {
	
	let num_moves: usize = moves.count_ones() as usize;
	let mut vec = Vec::with_capacity(num_moves);
	
	while moves != 0 {
		vec.push(next_bit_move(&mut moves));
	}
	
	return vec;
	
}

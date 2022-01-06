#![allow(dead_code)]

use bitintr::{Blsi, Blsr, Pdep};
use crate::consts;

const A_FILE: u64 = 0x0101010101010101;
const H_FILE: u64 = 0x8080808080808080;

#[inline(always)]
fn shift_n(bb: u64) -> u64 {
	return bb << 8;
}

#[inline(always)]
fn shift_s(bb: u64) -> u64 {
	return bb >> 8;
}

#[inline(always)]
fn shift_w(bb: u64) -> u64 {
	return (bb & (!A_FILE)) >> 1;
}

#[inline(always)]
fn shift_e(bb: u64) -> u64 {
	return (bb & (!H_FILE)) << 1;
}

#[inline(always)]
fn shift_ne(bb: u64) -> u64 {
	return shift_n(shift_e(bb));
}

#[inline(always)]
fn shift_nw(bb: u64) -> u64 {
	return shift_n(shift_w(bb));
}

#[inline(always)]
fn shift_se(bb: u64) -> u64 {
	return shift_s(shift_e(bb));
}

#[inline(always)]
fn shift_sw(bb: u64) -> u64 {
	return shift_s(shift_w(bb));
}

pub fn generate_moves(bb_self: u64, bb_enemy: u64) -> u64 {
	
	let mut flips = gen(bb_self, bb_enemy, -9);
	flips |= gen(bb_self, bb_enemy, -8);
	flips |= gen(bb_self, bb_enemy, -7);
	flips |= gen(bb_self, bb_enemy, -1);
	flips |= gen(bb_self, bb_enemy, 1);
	flips |= gen(bb_self, bb_enemy, 7);
	flips |= gen(bb_self, bb_enemy, 8);
	flips |= gen(bb_self, bb_enemy, 9);
	
	return flips & !bb_self & !bb_enemy;
}

fn gen(bb_self: u64, bb_enemy: u64, dir: isize) -> u64 {
	//rotate might be faster on AVX-512
	fn shift(x: u64, y: isize) -> u64 {
		if y > 0 {
			x >> y
		} else {
			x << -y
		}
	}
	let x = bb_self;
	//if we change above to rotate, we should also modify the following
	let y = bb_enemy
		& match dir.rem_euclid(8) {
		0 => !0,
		1 | 7 => 0x7E7E_7E7E_7E7E_7E7E,
		_ => unreachable!(),
	};
	let d = dir;
	let x = x | y & shift(x, d);
	let y = y & shift(y, d);
	let d = d * 2;
	let x = x | y & shift(x, d);
	let y = y & shift(y, d);
	let d = d * 2;
	let x = x | y & shift(x, d);
	shift(x ^ bb_self, dir)
}


pub fn generate_moves2(bb_self: u64, bb_enemy: u64) -> u64 {
	
	let mut moves: u64 = 0;
	let mut captured: u64;
	
	captured = shift_n(bb_self) & bb_enemy;
	for _ in 0..5 {
		captured |= shift_n(captured) & bb_enemy;
	}
	moves |= shift_n(captured);
	
	captured = shift_s(bb_self) & bb_enemy;
	for _ in 0..5 {
		captured |= shift_s(captured) & bb_enemy;
	}
	moves |= shift_s(captured);
	
	captured = shift_e(bb_self) & bb_enemy;
	for _ in 0..5 {
		captured |= shift_e(captured) & bb_enemy;
	}
	moves |= shift_e(captured);
	
	captured = shift_w(bb_self) & bb_enemy;
	for _ in 0..5 {
		captured |= shift_w(captured) & bb_enemy;
	}
	moves |= shift_w(captured);
	
	captured = shift_ne(bb_self) & bb_enemy;
	for _ in 0..5 {
		captured |= shift_ne(captured) & bb_enemy;
	}
	moves |= shift_ne(captured);
	
	captured = shift_nw(bb_self) & bb_enemy;
	for _ in 0..5 {
		captured |= shift_nw(captured) & bb_enemy;
	}
	moves |= shift_nw(captured);
	
	captured = shift_se(bb_self) & bb_enemy;
	for _ in 0..5 {
		captured |= shift_se(captured) & bb_enemy;
	}
	moves |= shift_se(captured);
	
	captured = shift_sw(bb_self) & bb_enemy;
	for _ in 0..5 {
		captured |= shift_sw(captured) & bb_enemy;
	}
	moves |= shift_sw(captured);
	
	return moves & !(bb_self | bb_enemy);
	
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
	/// https://gitlab.com/rust-othello/8x8-othello
	let place = mov.trailing_zeros() as usize;
	let diff = (0..4)
		.map(|i| unsafe {
			use bitintr::*;
			use consts::*;
			u64::from(*consts::RESULT.get_unchecked(
				INDEX.get_unchecked(place)[i] as usize * 32
					+ bb_self.pext(MASK.get_unchecked(place)[i][0]) as usize * 64
					+ bb_enemy.pext(MASK.get_unchecked(place)[i][1]) as usize,
			))
			.pdep(MASK.get_unchecked(place)[i][1])
		})
		.fold(0, core::ops::BitOr::bitor);
	return (bb_self ^ diff ^ mov, bb_enemy ^ diff);
}

pub fn make_move1(mov: u64, mut bb_self: u64, mut bb_enemy: u64) -> (u64, u64) {
	make_move_inplace(mov, &mut bb_self, &mut bb_enemy);
	return (bb_self, bb_enemy);
}

pub fn make_move_inplace(mov: u64, bb_self: &mut u64, bb_enemy: &mut u64) {
	
	// TODO: Optimize. in endgame search, 46% of the time is spent here
	// for each of 64 disks, have 8 direction masks. use pext to get ray
	// use blcmsk to find which disks to flip
	// test to make sure we flank. then use pdep to make flip mask
	// also investigate SSE method https://github.com/abulmo/edax-reversi/blob/master/src/flip_sse.c
	// https://www.chessprogramming.org/BMI2#PEXTBitboards
	// https://gitlab.com/rust-othello/8x8-othello
	
	// Kogge-Stone algorithm. Modified for Othello from
	// https://www.chessprogramming.org/Kogge-Stone_Algorithm#Occluded_Fill
	
	// 2 directions (NE, SW) use the Dumb7Fill algorithm in tune_zen2 mode
	// https://www.chessprogramming.org/Dumb7Fill#OccludedFill
	// Kogge-Stone is vectorized with AVX2, which uses few, but slow, instructions.
	// Processing 2 directions Dumb7fill, allows the CPU to parallelize AVX and normal registers.
	// This uses 26% more instructions, but keeps IPC at 3.27 vs. 2.4 which is 7% faster on zen2
	// On intel tigerlake, this function uses AVX-512 and using entirely Kogge-stone is 5% faster
	
	let mut captured;
	let mut flips = 0u64;
	
	*bb_self |= mov;
	
	// NORTH
	let mut pro = *bb_enemy;
	captured = mov;
	captured |= pro & (captured << 8);
	pro &= pro << 8;
	captured |= pro & (captured << 16);
	pro &= pro << 16;
	captured |= pro & (captured << 32);
	// captured = shift_n(mov) & *bb_enemy;
	// for _ in 0..5 {
	// 	captured |= shift_n(captured) & *bb_enemy;
	// }
	if shift_n(captured) & *bb_self != 0 {
		flips |= captured;
	}
	
	// SOUTH
	pro = *bb_enemy;
	captured = mov;
	captured |= pro & (captured >>  8);
	pro &= pro >> 8;
	captured |= pro & (captured >> 16);
	pro &= pro >> 16;
	captured |= pro & (captured >> 32);
	// captured = shift_s(mov) & *bb_enemy;
	// for _ in 0..5 {
	// 	captured |= shift_s(captured) & *bb_enemy;
	// }
	if shift_s(captured) & *bb_self != 0 {
		flips |= captured;
	}
	
	// EAST
	pro = *bb_enemy;
	captured = mov;
	pro &= !A_FILE;
	captured |= pro & (captured << 1);
	pro &= pro << 1;
	captured |= pro & (captured << 2);
	pro &= pro << 2;
	captured |= pro & (captured << 4);
	// captured = shift_e(mov) & *bb_enemy;
	// for _ in 0..5 {
	// 	captured |= shift_e(captured) & *bb_enemy;
	// }
	if shift_e(captured) & *bb_self != 0 {
		flips |= captured;
	}
	
	// WEST
	pro = *bb_enemy;
	captured = mov;
	pro &= !H_FILE;
	captured |= pro & (captured >> 1);
	pro &= pro >> 1;
	captured |= pro & (captured >> 2);
	pro &= pro >> 2;
	captured |= pro & (captured >> 4);
	// captured = shift_w(mov) & *bb_enemy;
	// for _ in 0..5 {
	// 	captured |= shift_w(captured) & *bb_enemy;
	// }
	if shift_w(captured) & *bb_self != 0 {
		flips |= captured;
	}
	
	
	// NORTH EAST
	if cfg!(feature = "tune_zen2") {
		captured = shift_ne(mov) & *bb_enemy;
		for _ in 0..5 {
			captured |= shift_ne(captured) & *bb_enemy;
		}
	} else {
		pro = *bb_enemy;
		captured = mov;
		pro &= !A_FILE;
		captured |= pro & (captured << 9);
		pro &= pro << 9;
		captured |= pro & (captured << 18);
		pro &= pro << 18;
		captured |= pro & (captured << 36);
	}
	if shift_ne(captured) & *bb_self != 0 {
		flips |= captured;
	}
	
	// NORTH WEST
	pro = *bb_enemy;
	captured = mov;
	pro &= !H_FILE;
	captured |= pro & (captured << 7);
	pro &= pro << 7;
	captured |= pro & (captured << 14);
	pro &= pro << 14;
	captured |= pro & (captured << 28);
	// captured = shift_nw(mov) & *bb_enemy;
	// for _ in 0..5 {
	// 	captured |= shift_nw(captured) & *bb_enemy;
	// }
	if shift_nw(captured) & *bb_self != 0 {
		flips |= captured;
	}
	
	// SOUTH EAST
	pro = *bb_enemy;
	captured = mov;
	pro &= !A_FILE;
	captured |= pro & (captured >> 7);
	pro &= pro >> 7;
	captured |= pro & (captured >> 14);
	pro &= pro >> 14;
	captured |= pro & (captured >> 28);
	// captured = shift_se(mov) & *bb_enemy;
	// for _ in 0..5 {
	// 	captured |= shift_se(captured) & *bb_enemy;
	// }
	if shift_se(captured) & *bb_self != 0 {
		flips |= captured;
	}
	
	// SOUTH WEST
	if cfg!(feature = "tune_zen2") {
		captured = shift_sw(mov) & *bb_enemy;
		for _ in 0..5 {
			captured |= shift_sw(captured) & *bb_enemy;
		}
	} else {
		pro = *bb_enemy;
		captured = mov;
		pro &= !H_FILE;
		captured |= pro & (captured >> 9);
		pro &= pro >> 9;
		captured |= pro & (captured >> 18);
		pro &= pro >> 18;
		captured |= pro & (captured >> 36);
	}
	if shift_sw(captured) & *bb_self != 0 {
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

/// Evaluates a position for win/loss/draw for midgame search
/// It is possible for the ai to get eliminated early, and since
/// evaluation doesn't add empty disks to the winner, the search
/// could pick loosing early as the best move if the neural network is pessimistic
/// Inversely, it will delay killing the opponent early as continuing will allow for more
/// disks to be played, thus improving the score.
/// But, we should take the earliest win and delay the loss, so this returns 65 + empties
/// if forced winning (end game early) and -65 - empties if forced loss (don't play an early losing move)
/// +/- empties makes negamax pick the earliest win rather than a win.
#[inline(always)]
pub fn wld_evaluation(black: u64, white: u64) -> i8 {
	let q = evaluation(black, white);
	if q == 0 { 0 }
	else if q > 0 { 65 + empty_disks(black, white) as i8 }
	else { -65 - empty_disks(black, white) as i8 }
}

/// Evaluates the board at the end of the game
/// Returns disk difference, DOES NOT add empty disks to the winner's score
#[inline(always)]
pub fn evaluation(black: u64, white: u64) -> i8 {
	(black.count_ones() as i8) - (white.count_ones() as i8)
}

/// Evaluates the board at the end of the game
/// Returns disk difference, DOES add empty disks to the winner's score
/// Slightly slower than the simple evaluation, use that when possible
#[inline(always)]
pub fn evaluation_full(black: u64, white: u64) -> i8 {
	let score = (black.count_ones() as i8) - (white.count_ones() as i8);
	score + score.signum() * empty_disks(black, white) as i8
}

#[inline(always)]
pub fn empty_disks(black: u64, white: u64) -> u8 {
	(black | white).count_zeros() as u8
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

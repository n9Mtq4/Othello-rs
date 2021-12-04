const SYM_IDENTITY: [u8; 64] = [
	 0,  1,  2,  3,  4,  5,  6,  7,
	 8,  9, 10, 11, 12, 13, 14, 15,
	16, 17, 18, 19, 20, 21, 22, 23,
	24, 25, 26, 27, 28, 29, 30, 31,
	32, 33, 34, 35, 36, 37, 38, 39,
	40, 41, 42, 43, 44, 45, 46, 47,
	48, 49, 50, 51, 52, 53, 54, 55,
	56, 57, 58, 59, 60, 61, 62, 63
];

const SYM_ROTATE_90_CC: [u8; 64] = [
	56, 48, 40, 32, 24, 16,  8,  0,
	57, 49, 41, 33, 25, 17,  9,  1,
	58, 50, 42, 34, 26, 18, 10,  2,
	59, 51, 43, 35, 27, 19, 11,  3,
	60, 52, 44, 36, 28, 20, 12,  4,
	61, 53, 45, 37, 29, 21, 13,  5,
	62, 54, 46, 38, 30, 22, 14,  6,
	63, 55, 47, 39, 31, 23, 15,  7
];

const SYM_ROTATE_180_CC: [u8; 64] = [
	63, 62, 61, 60, 59, 58, 57, 56,
	55, 54, 53, 52, 51, 50, 49, 48,
	47, 46, 45, 44, 43, 42, 41, 40,
	39, 38, 37, 36, 35, 34, 33, 32,
	31, 30, 29, 28, 27, 26, 25, 24,
	23, 22, 21, 20, 19, 18, 17, 16,
	15, 14, 13, 12, 11, 10,  9,  8,
	 7,  6,  5,  4,  3,  2,  1,  0
];

const SYM_ROTATE_270_CC: [u8; 64] = [
	 7, 15, 23, 31, 39, 47, 55, 63,
	 6, 14, 22, 30, 38, 46, 54, 62,
	 5, 13, 21, 29, 37, 45, 53, 61,
	 4, 12, 20, 28, 36, 44, 52, 60,
	 3, 11, 19, 27, 35, 43, 51, 59,
	 2, 10, 18, 26, 34, 42, 50, 58,
	 1,  9, 17, 25, 33, 41, 49, 57,
	 0,  8, 16, 24, 32, 40, 48, 56
];

const SYM_FLIP_X_AXIS: [u8; 64] = [
	56, 57, 58, 59, 60, 61, 62, 63,
	48, 49, 50, 51, 52, 53, 54, 55,
	40, 41, 42, 43, 44, 45, 46, 47,
	32, 33, 34, 35, 36, 37, 38, 39,
	24, 25, 26, 27, 28, 29, 30, 31,
	16, 17, 18, 19, 20, 21, 22, 23,
	 8,  9, 10, 11, 12, 13, 14, 15,
	 0,  1,  2,  3,  4,  5,  6,  7
];

const SYM_FLIP_Y_AXIS: [u8; 64] = [
	 7,  6,  5,  4,  3,  2,  1,  0,
	15, 14, 13, 12, 11, 10,  9,  8,
	23, 22, 21, 20, 19, 18, 17, 16,
	31, 30, 29, 28, 27, 26, 25, 24,
	39, 38, 37, 36, 35, 34, 33, 32,
	47, 46, 45, 44, 43, 42, 41, 40,
	55, 54, 53, 52, 51, 50, 49, 48,
	63, 62, 61, 60, 59, 58, 57, 56
];

const SYM_FLIP_TOPL_DIAGONAL: [u8; 64] = [
	 0,  8, 16, 24, 32, 40, 48, 56,
	 1,  9, 17, 25, 33, 41, 49, 57,
	 2, 10, 18, 26, 34, 42, 50, 58,
	 3, 11, 19, 27, 35, 43, 51, 59,
	 4, 12, 20, 28, 36, 44, 52, 60,
	 5, 13, 21, 29, 37, 45, 53, 61,
	 6, 14, 22, 30, 38, 46, 54, 62,
	 7, 15, 23, 31, 39, 47, 55, 63
];

const SYM_FLIP_TOPR_DIAGONAL: [u8; 64] = [
	63, 55, 47, 39, 31, 23, 15,  7,
	62, 54, 46, 38, 30, 22, 14,  6,
	61, 53, 45, 37, 29, 21, 13,  5,
	60, 52, 44, 36, 28, 20, 12,  4,
	59, 51, 43, 35, 27, 19, 11,  3,
	58, 50, 42, 34, 26, 18, 10,  2,
	57, 49, 41, 33, 25, 17,  9,  1,
	56, 48, 40, 32, 24, 16,  8,  0
];

/// Given a transform and a board location (like a move index)
/// Applies the inverse of the transformation and returns the
/// index of that location before the transformation was applied
pub fn sym_invert_loc(transform: &[u8; 64], pos: u8) -> u8 {
	
	// scan transform and return where i == pos
	for i in 0u8..64u8 {
		if transform[i as usize] == pos {
			return i;
		}
	}
	
	panic!("didn't find mov in sym_invert");
	
}

/// Find the minimum symmetry of a board
/// Each board position has 8 symmetries, this will find the minimum one
/// Returns the min board and the transform used to get the min board
/// Returns (me, enemy, transform)
pub fn sym_min_board(me: u64, enemy: u64) -> (u64, u64, &'static [u8; 64]) {
	
	// all the symmetries that can be applied
	let syms: [&[u8; 64]; 7] = [
		&SYM_ROTATE_90_CC,
		&SYM_ROTATE_180_CC,
		&SYM_ROTATE_270_CC,
		&SYM_FLIP_X_AXIS,
		&SYM_FLIP_Y_AXIS,
		&SYM_FLIP_TOPL_DIAGONAL,
		&SYM_FLIP_TOPR_DIAGONAL,
	];
	
	// start with identity
	let mut min_m = me;
	let mut min_e = enemy;
	let mut min_transform = &SYM_IDENTITY;
	
	for sym in syms {
		
		// apply a symmetry
		let (m, e) = sym_apply_to(sym, me, enemy);
		
		// if we found a smaller me, or me is the same, but enemy is smaller, we found a smaller board
		if (m < min_m) || ((m == min_m) && (e < min_e)) {
			min_m = m;
			min_e = e;
			min_transform = sym;
		}
		
	}
	
	return (min_m, min_e, min_transform);
	
}

/// Applies a transform to me and enemy
fn sym_apply_to(transform: &[u8; 64], m: u64, e: u64) -> (u64, u64) {
	
	(sym_apply_to_bb(transform, m), sym_apply_to_bb(transform, e))
	
}

/// Applies a transform to a single bitboard
fn sym_apply_to_bb(transform: &[u8; 64], bb: u64) -> u64 {
	
	let mut new_board = 0u64;
	
	for i in 0..64 {
		// if the bitboard has the ith bit set
		if bb & (1u64 << i) != 0 {
			// set the transformed bit in the new bitboard
			new_board |= 1u64 << transform[i]
		}
	}
	
	return new_board;
	
}

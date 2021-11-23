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

fn generate_moves_loops(bb_self: u64, bb_enemy: u64) -> u64 {
	
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

pub fn generate_moves(bb_self: u64, bb_enemy: u64) -> u64 {
	
	let open: u64 = !(bb_self | bb_enemy);
	let mut moves: u64 = 0;
	let mut captured: u64;
	
	captured = shift_n(bb_self) & bb_enemy;
	captured |= shift_n(captured) & bb_enemy;
	captured |= shift_n(captured) & bb_enemy;
	captured |= shift_n(captured) & bb_enemy;
	captured |= shift_n(captured) & bb_enemy;
	captured |= shift_n(captured) & bb_enemy;
	moves |= shift_n(captured) & open;
	
	captured = shift_s(bb_self) & bb_enemy;
	captured |= shift_s(captured) & bb_enemy;
	captured |= shift_s(captured) & bb_enemy;
	captured |= shift_s(captured) & bb_enemy;
	captured |= shift_s(captured) & bb_enemy;
	captured |= shift_s(captured) & bb_enemy;
	moves |= shift_s(captured) & open;
	
	captured = shift_w(bb_self) & bb_enemy;
	captured |= shift_w(captured) & bb_enemy;
	captured |= shift_w(captured) & bb_enemy;
	captured |= shift_w(captured) & bb_enemy;
	captured |= shift_w(captured) & bb_enemy;
	captured |= shift_w(captured) & bb_enemy;
	moves |= shift_w(captured) & open;
	
	captured = shift_e(bb_self) & bb_enemy;
	captured |= shift_e(captured) & bb_enemy;
	captured |= shift_e(captured) & bb_enemy;
	captured |= shift_e(captured) & bb_enemy;
	captured |= shift_e(captured) & bb_enemy;
	captured |= shift_e(captured) & bb_enemy;
	moves |= shift_e(captured) & open;
	
	captured = shift_nw(bb_self) & bb_enemy;
	captured |= shift_nw(captured) & bb_enemy;
	captured |= shift_nw(captured) & bb_enemy;
	captured |= shift_nw(captured) & bb_enemy;
	captured |= shift_nw(captured) & bb_enemy;
	captured |= shift_nw(captured) & bb_enemy;
	moves |= shift_nw(captured) & open;
	
	captured = shift_ne(bb_self) & bb_enemy;
	captured |= shift_ne(captured) & bb_enemy;
	captured |= shift_ne(captured) & bb_enemy;
	captured |= shift_ne(captured) & bb_enemy;
	captured |= shift_ne(captured) & bb_enemy;
	captured |= shift_ne(captured) & bb_enemy;
	moves |= shift_ne(captured) & open;
	
	captured = shift_sw(bb_self) & bb_enemy;
	captured |= shift_sw(captured) & bb_enemy;
	captured |= shift_sw(captured) & bb_enemy;
	captured |= shift_sw(captured) & bb_enemy;
	captured |= shift_sw(captured) & bb_enemy;
	captured |= shift_sw(captured) & bb_enemy;
	moves |= shift_sw(captured) & open;
	
	captured = shift_se(bb_self) & bb_enemy;
	captured |= shift_se(captured) & bb_enemy;
	captured |= shift_se(captured) & bb_enemy;
	captured |= shift_se(captured) & bb_enemy;
	captured |= shift_se(captured) & bb_enemy;
	captured |= shift_se(captured) & bb_enemy;
	moves |= shift_se(captured) & open;
	
	return moves;
	
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
	
	let mut captured;
	
	*bb_self |= mov;
	
	captured = shift_n(mov) & *bb_enemy;
	captured |= shift_n(captured) & *bb_enemy;
	captured |= shift_n(captured) & *bb_enemy;
	captured |= shift_n(captured) & *bb_enemy;
	captured |= shift_n(captured) & *bb_enemy;
	captured |= shift_n(captured) & *bb_enemy;
	if shift_n(captured) & *bb_self != 0 {
		*bb_self |= captured;
		*bb_enemy &= !captured;
	}
	
	captured = shift_s(mov) & *bb_enemy;
	captured |= shift_s(captured) & *bb_enemy;
	captured |= shift_s(captured) & *bb_enemy;
	captured |= shift_s(captured) & *bb_enemy;
	captured |= shift_s(captured) & *bb_enemy;
	captured |= shift_s(captured) & *bb_enemy;
	if shift_s(captured) & *bb_self != 0 {
		*bb_self |= captured;
		*bb_enemy &= !captured;
	}
	
	captured = shift_w(mov) & *bb_enemy;
	captured |= shift_w(captured) & *bb_enemy;
	captured |= shift_w(captured) & *bb_enemy;
	captured |= shift_w(captured) & *bb_enemy;
	captured |= shift_w(captured) & *bb_enemy;
	captured |= shift_w(captured) & *bb_enemy;
	if shift_w(captured) & *bb_self != 0 {
		*bb_self |= captured;
		*bb_enemy &= !captured;
	}
	
	captured = shift_e(mov) & *bb_enemy;
	captured |= shift_e(captured) & *bb_enemy;
	captured |= shift_e(captured) & *bb_enemy;
	captured |= shift_e(captured) & *bb_enemy;
	captured |= shift_e(captured) & *bb_enemy;
	captured |= shift_e(captured) & *bb_enemy;
	if shift_e(captured) & *bb_self != 0 {
		*bb_self |= captured;
		*bb_enemy &= !captured;
	}
	
	captured = shift_nw(mov) & *bb_enemy;
	captured |= shift_nw(captured) & *bb_enemy;
	captured |= shift_nw(captured) & *bb_enemy;
	captured |= shift_nw(captured) & *bb_enemy;
	captured |= shift_nw(captured) & *bb_enemy;
	captured |= shift_nw(captured) & *bb_enemy;
	if shift_nw(captured) & *bb_self != 0 {
		*bb_self |= captured;
		*bb_enemy &= !captured;
	}
	
	captured = shift_ne(mov) & *bb_enemy;
	captured |= shift_ne(captured) & *bb_enemy;
	captured |= shift_ne(captured) & *bb_enemy;
	captured |= shift_ne(captured) & *bb_enemy;
	captured |= shift_ne(captured) & *bb_enemy;
	captured |= shift_ne(captured) & *bb_enemy;
	if shift_ne(captured) & *bb_self != 0 {
		*bb_self |= captured;
		*bb_enemy &= !captured;
	}
	
	captured = shift_sw(mov) & *bb_enemy;
	captured |= shift_sw(captured) & *bb_enemy;
	captured |= shift_sw(captured) & *bb_enemy;
	captured |= shift_sw(captured) & *bb_enemy;
	captured |= shift_sw(captured) & *bb_enemy;
	captured |= shift_sw(captured) & *bb_enemy;
	if shift_sw(captured) & *bb_self != 0 {
		*bb_self |= captured;
		*bb_enemy &= !captured;
	}
	
	captured = shift_se(mov) & *bb_enemy;
	captured |= shift_se(captured) & *bb_enemy;
	captured |= shift_se(captured) & *bb_enemy;
	captured |= shift_se(captured) & *bb_enemy;
	captured |= shift_se(captured) & *bb_enemy;
	captured |= shift_se(captured) & *bb_enemy;
	if shift_se(captured) & *bb_self != 0 {
		*bb_self |= captured;
		*bb_enemy &= !captured;
	}
	
}

pub fn game_over(bb_p1: u64, bb_p2: u64) -> bool {
	return (bb_p1 | bb_p2 == u64::MAX) ||
		(generate_moves(bb_p1, bb_p2) == 0 &&
			generate_moves(bb_p2, bb_p1) == 0);
}

pub fn to_idx_move_vec(moves: u64) -> Vec<u8> {
	
	let num_moves: usize = moves.count_ones() as usize;
	let mut vec = Vec::with_capacity(num_moves);
	
	let mut move_idx: usize = 0;
	let mut i: u8 = 0;
	while move_idx < num_moves {
		if ((moves >> i) & 1) == 1 {
			vec.push(i);
			move_idx += 1;
		}
		i += 1;
	}
	
	return vec;
	
}

pub fn to_bit_move_vec(moves: u64) -> Vec<u64> {
	
	let num_moves: usize = moves.count_ones() as usize;
	let mut vec = Vec::with_capacity(num_moves);
	
	let mut move_idx: usize = 0;
	let mut i: u32 = 0;
	while move_idx < num_moves {
		if ((moves >> i) & 1) == 1 {
			vec.push(1u64 << i);
			move_idx += 1;
		}
		i += 1;
	}
	
	return vec;
	
}

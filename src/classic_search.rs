use crate::classic_weights::CLASSIC_WEIGHTS;
use crate::othello_board::{empty_disks, evaluation, game_over, generate_moves, make_move, to_idx_move_vec};

/// Flips a board along the diagonal
/// https://www.chessprogramming.org/Flipping_Mirroring_and_Rotating
fn flip_diag_a1h8(mut x: u64) -> u64 {
	let mut t: u64;
	let k1 = 0x5500550055005500u64;
	let k2 = 0x3333000033330000u64;
	let k4 = 0x0f0f0f0f00000000u64;
	t  = k4 & (x ^ (x << 28));
	x ^=       t ^ (t >> 28) ;
	t  = k2 & (x ^ (x << 14));
	x ^=       t ^ (t >> 14) ;
	t  = k1 & (x ^ (x <<  7));
	x ^=       t ^ (t >>  7) ;
	return x;
}

/// Guesses how many centidisks (100 * disks) `me` will have at the end of the game.
/// Positions with 7-25 empties are within the expected range for this function.
/// MAE=937 centidisks
/// Returns from POV of `me` and is for use in a negamax framework.
fn heuristic_eg_nega(me: u64, enemy: u64, w_idx: usize) -> i32 {
	
	let m00 = (me >> 0 * 8) & 0b11111111u64;
	let m10 = (me >> 1 * 8) & 0b11111111u64;
	let m20 = (me >> 2 * 8) & 0b11111111u64;
	let m30 = (me >> 3 * 8) & 0b11111111u64;
	let m31 = (me >> 4 * 8) & 0b11111111u64;
	let m21 = (me >> 5 * 8) & 0b11111111u64;
	let m11 = (me >> 6 * 8) & 0b11111111u64;
	let m01 = (me >> 7 * 8) & 0b11111111u64;
	
	let e00 = (enemy >> 0 * 8) & 0b11111111u64;
	let e10 = (enemy >> 1 * 8) & 0b11111111u64;
	let e20 = (enemy >> 2 * 8) & 0b11111111u64;
	let e30 = (enemy >> 3 * 8) & 0b11111111u64;
	let e31 = (enemy >> 4 * 8) & 0b11111111u64;
	let e21 = (enemy >> 5 * 8) & 0b11111111u64;
	let e11 = (enemy >> 6 * 8) & 0b11111111u64;
	let e01 = (enemy >> 7 * 8) & 0b11111111u64;
	
	let me_flip = flip_diag_a1h8(me);
	let mf00 = (me_flip >> 0 * 8) & 0b11111111u64;
	let mf10 = (me_flip >> 1 * 8) & 0b11111111u64;
	let mf20 = (me_flip >> 2 * 8) & 0b11111111u64;
	let mf30 = (me_flip >> 3 * 8) & 0b11111111u64;
	let mf31 = (me_flip >> 4 * 8) & 0b11111111u64;
	let mf21 = (me_flip >> 5 * 8) & 0b11111111u64;
	let mf11 = (me_flip >> 6 * 8) & 0b11111111u64;
	let mf01 = (me_flip >> 7 * 8) & 0b11111111u64;
	
	let enemy_flip = flip_diag_a1h8(enemy);
	let ef00 = (enemy_flip >> 0 * 8) & 0b11111111u64;
	let ef10 = (enemy_flip >> 1 * 8) & 0b11111111u64;
	let ef20 = (enemy_flip >> 2 * 8) & 0b11111111u64;
	let ef30 = (enemy_flip >> 3 * 8) & 0b11111111u64;
	let ef31 = (enemy_flip >> 4 * 8) & 0b11111111u64;
	let ef21 = (enemy_flip >> 5 * 8) & 0b11111111u64;
	let ef11 = (enemy_flip >> 6 * 8) & 0b11111111u64;
	let ef01 = (enemy_flip >> 7 * 8) & 0b11111111u64;
	
	let me_score = CLASSIC_WEIGHTS[w_idx + 0][m00 as usize] + CLASSIC_WEIGHTS[w_idx + 0][m01 as usize] +
		CLASSIC_WEIGHTS[w_idx + 1][m10 as usize] + CLASSIC_WEIGHTS[w_idx + 1][m11 as usize] +
		CLASSIC_WEIGHTS[w_idx + 2][m20 as usize] + CLASSIC_WEIGHTS[w_idx + 2][m21 as usize] +
		CLASSIC_WEIGHTS[w_idx + 3][m30 as usize] + CLASSIC_WEIGHTS[w_idx + 3][m31 as usize] +
		CLASSIC_WEIGHTS[w_idx + 0][mf00 as usize] + CLASSIC_WEIGHTS[w_idx + 0][mf01 as usize] +
		CLASSIC_WEIGHTS[w_idx + 1][mf10 as usize] + CLASSIC_WEIGHTS[w_idx + 1][mf11 as usize] +
		CLASSIC_WEIGHTS[w_idx + 2][mf20 as usize] + CLASSIC_WEIGHTS[w_idx + 2][mf21 as usize] +
		CLASSIC_WEIGHTS[w_idx + 3][mf30 as usize] + CLASSIC_WEIGHTS[w_idx + 3][mf31 as usize];
	
	let enemy_score = CLASSIC_WEIGHTS[w_idx + 0][e00 as usize] + CLASSIC_WEIGHTS[w_idx + 0][e01 as usize] +
		CLASSIC_WEIGHTS[w_idx + 1][e10 as usize] + CLASSIC_WEIGHTS[w_idx + 1][e11 as usize] +
		CLASSIC_WEIGHTS[w_idx + 2][e20 as usize] + CLASSIC_WEIGHTS[w_idx + 2][e21 as usize] +
		CLASSIC_WEIGHTS[w_idx + 3][e30 as usize] + CLASSIC_WEIGHTS[w_idx + 3][e31 as usize] +
		CLASSIC_WEIGHTS[w_idx + 0][ef00 as usize] + CLASSIC_WEIGHTS[w_idx + 0][ef01 as usize] +
		CLASSIC_WEIGHTS[w_idx + 1][ef10 as usize] + CLASSIC_WEIGHTS[w_idx + 1][ef11 as usize] +
		CLASSIC_WEIGHTS[w_idx + 2][ef20 as usize] + CLASSIC_WEIGHTS[w_idx + 2][ef21 as usize] +
		CLASSIC_WEIGHTS[w_idx + 3][ef30 as usize] + CLASSIC_WEIGHTS[w_idx + 3][ef31 as usize];
	
	return me_score - enemy_score;
	
}

/// Returns (move, eval)
pub fn classic_search_root(me: u64, enemy: u64, mut alpha: i32, beta: i32, depth: i8) -> (u8, i32) {
	
	// if the game is over, evaluate who won
	if game_over(me, enemy) {
		return (65, 100 * (evaluation(me, enemy) as i32));
	}
	
	// get possible moves
	let moves = generate_moves(me, enemy);
	
	// compute weights offset. assume one disk per ply
	let w_idx: usize = 4 * ((empty_disks(me, enemy) as usize) - (depth as usize));
	
	// if no moves, pass
	if moves == 0 {
		return (65, -classic_search_mo(enemy, me, -beta, -alpha, depth - 1, 7, w_idx));
	}
	
	// apply each move and get the state
	let mut states: Vec<(u8, u64, u64)> = to_idx_move_vec(moves)
		.iter()
		.map(|mov| {
			let (new_me, new_enemy) = make_move(1u64 << *mov, me, enemy);
			(*mov, new_me, new_enemy)
		})
		.collect();
	
	// sort the child states, best one first
	states.sort_by_cached_key(|(_, me, enemy)| heuristic_eg_nega(*enemy, *me, w_idx));
	
	let mut best_score = -640000;
	let mut best_move: u8 = 65;
	
	// for each child state
	for (mov, me, enemy) in states {
		
		let q = -classic_search_mo(enemy, me, -beta, -alpha, depth - 1, 7, w_idx);
		
		if q >= beta {
			return (mov as u8, q);
		}
		
		if q > best_score {
			best_score = q;
			best_move = mov as u8;
			if q > alpha {
				alpha = q;
			}
		}
		
	}
	
	return (best_move, best_score);
	
}

/// Fail-soft negamax for endgame solving
/// Does not use move ordering
/// https://www.chessprogramming.org/Alpha-Beta
fn classic_search_nomo(me: u64, enemy: u64, mut alpha: i32, beta: i32, depth: i8, w_idx: usize) -> i32 {
	
	// if the game is over, evaluate who won
	if game_over(me, enemy) {
		return 100 * (evaluation(me, enemy) as i32);
	}
	
	// if the depth is 0, evaluate the position with the nn
	if depth <= 0 {
		return heuristic_eg_nega(me, enemy, w_idx);
	}
	
	// get possible moves
	let moves = generate_moves(me, enemy);
	
	// if no moves, pass
	if moves == 0 {
		return -classic_search_nomo(enemy, me, -beta, -alpha, depth - 1, w_idx);
	}
	
	let num_moves: usize = moves.count_ones() as usize;
	let mut best_score = -127;
	
	// for each move
	let mut move_idx: usize = 0;
	let mut i: u8 = 0;
	while move_idx < num_moves {
		if ((moves >> i) & 1) == 1 {
			
			// evaluate the child state
			let (me, enemy) = make_move(1u64 << i, me, enemy);
			let q = -classic_search_nomo(enemy, me, -beta, -alpha, depth - 1, w_idx);
			
			if q >= beta {
				return q;
			}
			
			if q > best_score {
				best_score = q;
				if q > alpha {
					alpha = q;
				}
			}
			
			move_idx += 1;
		}
		i += 1;
	}
	
	return best_score;
	
}

fn classic_search_mo(me: u64, enemy: u64, mut alpha: i32, beta: i32, depth: i8, stop_mo_at_empties: u8, w_idx: usize) -> i32 {
	
	// if the game is over, evaluate who won
	if game_over(me, enemy) {
		return 100 * (evaluation(me, enemy) as i32);
	}
	
	// if the depth is 0, evaluate the position with the nn
	if depth <= 0 {
		return heuristic_eg_nega(me, enemy, w_idx);
	}
	
	// get possible moves
	let moves = generate_moves(me, enemy);
	
	// if no moves, pass
	if moves == 0 {
		return -classic_search_mo(enemy, me, -beta, -alpha, depth - 1, stop_mo_at_empties, w_idx);
	}
	
	// apply each move and get the state
	let mut states: Vec<(u64, u64)> = to_idx_move_vec(moves)
		.iter()
		.map(|mov| make_move(1u64 << *mov, me, enemy))
		.collect();
	
	// sort the child states, best one first
	// benchmark: sort_by_key=39.54s, sort_unstable_by_key=39.79s, sort_by_cached_key=38.74s
	states.sort_by_cached_key(|(me, enemy)| heuristic_eg_nega(*enemy, *me, w_idx));
	
	let mut best_score = -640000;
	let empty_disks = empty_disks(me, enemy);
	
	// for each child state
	for (me, enemy) in states {
		
		// stop ordering the moves if the empty disks is smaller than the cutoff
		let q = if empty_disks > stop_mo_at_empties {
			-classic_search_mo(enemy, me, -beta, -alpha, depth - 1, stop_mo_at_empties, w_idx)
		} else {
			-classic_search_nomo(enemy, me, -beta, -alpha, depth - 1, w_idx)
		};
		
		if q >= beta {
			return q;
		}
		
		if q > best_score {
			best_score = q;
			if q > alpha {
				alpha = q;
			}
		}
		
	}
	
	return best_score;
	
}

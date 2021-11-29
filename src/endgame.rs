use crate::othello_board::{empty_disks, evaluation, game_over, generate_moves, make_move, to_idx_move_vec};

/// End game heuristic weights
/// Generated by thor database games & gradient descent
/// Designed for between 25-7 empties
static EG_WEIGHTS: [[i32; 256]; 4] = [
	[152, 637, -83, 640, 28, 153, -52, 770, -40, 241, -295, 37, -117, -135, -183, 726, -58, 148, -306, 83, -240, -234, -391, 243, -22, 140, -448, 2, -48, -57, -220, 690, 15, -94, -280, 129, -177, -162, -126, 294, -229, -291, -135, -28, -405, -660, -384, 120, -136, -87, -276, -107, -400, -311, -133, -208, -28, 160, -470, -29, 60, 0, 112, 768, -115, -146, -302, 52, -258, -169, -157, 202, -361, -49, -102, -2, -277, -350, -62, 220, -418, -74, -82, -2, -154, -15, 4, 2, -454, -71, -4, -2, -473, -83, -327, 31, -94, -185, -200, 91, -155, -326, -30, 88, -531, -118, -14, -2, -154, -49, -149, -13, -217, 13, -121, 97, -450, -134, -137, -83, -224, 18, -305, -105, 101, -420, 338, 934, 654, 258, -111, 176, -115, -126, -123, 159, 217, 74, -111, -1, 2, -130, 79, 287, 283, 56, -79, 13, -281, -87, -15, -9, 73, 16, -115, -2, 137, -36, 18, 555, 142, -78, -121, 20, -117, -195, -134, 33, -190, -61, -20, -1, -293, -78, -171, -40, -186, -177, -307, -31, -687, -87, -38, -49, -75, -16, -74, -12, -39, -515, -458, 661, 663, 175, 26, 9, 155, -3, 91, 71, 50, 7, -21, 0, -149, -42, 13, 117, 4, -16, -11, 5, -69, -3, -1, -1, -22, -4, -1, 0, -31, -16, -131, 37, 788, 214, 200, 61, 303, 21, 70, 103, 182, 5, 3, -1, -52, -34, -67, 91, 741, 346, 420, 109, 344, 21, 89, 57, 749, 606, 39, 45, 771, 693, 944, 1176],
	[649, 520, 265, 401, 272, 111, -23, 400, 343, 174, 43, 289, 167, -4, 87, 517, 342, 181, -10, 93, 118, -29, -127, 220, 192, -35, -104, 74, 59, -95, 50, 475, 263, 13, -131, -101, -116, -285, -410, -54, 113, -180, -116, 25, -33, -223, -160, 103, 168, -104, -184, -177, -23, -249, -332, -1, 65, -232, -178, -60, 34, -144, -33, 422, 259, 90, -164, -80, -109, -191, -319, 27, -11, -162, -237, -76, -190, -252, -258, 143, 3, -135, -260, -75, -97, -97, -202, -24, -113, -291, -372, -152, -188, -250, -226, 289, 0, -212, -271, -160, -390, -449, -486, -9, -126, -360, -268, -32, -324, -397, -203, 242, 83, -125, -237, -152, -186, -233, -257, 201, 41, -171, -201, -31, -39, -90, 117, 855, 530, 259, 106, 152, 43, -92, -244, 146, 201, -52, -123, 80, -109, -286, -123, 293, 196, -24, -96, -46, -185, -174, -330, 17, -6, -251, -234, -98, -219, -352, -141, 253, 136, -90, -169, -46, -256, -423, -463, -75, -22, -229, 1, 0, -229, -431, -204, 43, 17, -247, -201, -112, -207, -379, -388, -110, -67, -343, -195, -45, -128, -311, -95, 309, 368, 136, -40, 25, -47, -76, -175, 156, 157, -112, -124, 50, -124, -69, -160, 326, 283, 70, -84, 34, 17, 56, -78, 152, 152, -69, -149, 0, -14, -32, 36, 561, 402, 158, 41, 211, -56, -123, 43, 323, 223, -51, 24, 120, 29, -30, 244, 560, 519, 241, 181, 237, 169, 80, 247, 503, 488, 206, 306, 449, 449, 345, 894, 1505],
	[688, 749, 287, 441, 181, 148, 141, 208, 229, 269, 3, 139, 222, 208, 283, 364, 287, 236, -55, 22, -74, -194, -80, -48, 40, 39, -175, -43, 126, 75, 228, 332, 255, 171, -104, -26, -163, -335, -221, -223, -48, -108, -248, -164, -60, -128, 20, 49, 202, 126, -114, -33, -110, -292, -144, -138, 30, -10, -198, -79, 177, 84, 271, 361, 253, 173, 20, 86, -190, -344, -157, -146, -172, -155, -210, -130, -122, -196, 26, 31, 30, 6, -150, -135, -263, -376, -184, -170, -228, -197, -240, -193, -116, -180, 80, 143, 164, 92, -112, -75, -247, -451, -260, -268, -128, -161, -208, -192, -136, -233, 19, 68, 227, 153, 0, -1, -77, -263, -32, -23, 82, 20, -9, -2, 259, 128, 488, 573, 736, 631, 229, 354, 87, 6, 15, 63, 168, 189, -18, 50, 121, 85, 179, 236, 304, 226, -80, 7, -123, -225, -175, -133, 32, 28, -189, -111, 41, -5, 124, 228, 203, 55, -260, -177, -339, -450, -482, -431, -171, -219, -368, -292, -269, -313, -210, -135, 206, 89, -200, -145, -200, -377, -287, -276, 1, -37, -241, -162, 74, -62, 144, 273, 437, 342, 113, 180, -91, -245, -102, -113, -40, -26, -143, -85, -27, -119, 44, 58, 197, 135, -40, -36, -152, -257, -118, -116, -17, -50, -180, -118, 17, -55, 146, 201, 286, 186, -58, -39, -212, -378, -245, -250, -55, -110, -192, -157, -50, -122, 79, 102, 375, 272, 74, 64, 21, -134, 47, 33, 276, 184, 73, 93, 402, 292, 617, 698],
	[430, 396, 114, 272, 95, 83, 226, 527, 263, 133, 2, 150, 200, 162, 423, 761, 400, 181, 24, 99, 40, 16, 123, 361, 161, 12, -58, 62, 144, 122, 369, 740, 273, 52, -190, -107, -141, -212, -146, 49, 56, -136, -252, -124, -69, -158, 53, 345, 235, 14, -241, -151, -161, -228, -145, 72, -8, -177, -320, -208, -129, -196, 35, 391, 93, -179, -345, -281, -353, -429, -254, -83, -108, -264, -339, -254, -244, -315, -51, 202, 120, -117, -255, -187, -246, -306, -178, 27, -107, -270, -317, -229, -221, -259, -6, 301, 356, 61, -136, -82, -150, -262, -156, 31, 100, -117, -183, -114, -61, -147, 65, 329, 409, 134, -103, -36, -84, -191, -52, 127, 208, -30, -171, -103, 1, -61, 146, 469, 312, 30, -214, -104, -150, -261, -99, 182, 33, -86, -261, -117, -65, -117, 109, 442, 201, -24, -230, -121, -196, -262, -148, 84, -49, -209, -308, -187, -97, -139, 86, 423, 202, -66, -370, -278, -271, -381, -314, -82, -6, -194, -341, -252, -198, -284, -112, 180, 156, -66, -336, -259, -282, -348, -275, -67, -62, -232, -406, -316, -221, -268, -95, 262, 257, -38, -264, -174, -249, -369, -206, -17, 24, -172, -255, -174, -157, -260, 22, 254, 286, 56, -180, -88, -120, -251, -95, 119, 49, -129, -206, -139, -58, -125, 91, 382, 677, 399, 84, 138, 85, -33, 33, 216, 380, 162, 41, 116, 165, 97, 264, 522, 800, 499, 186, 258, 223, 105, 221, 385, 640, 389, 165, 221, 436, 343, 512, 735]
];

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
fn heuristic_eg_nega(me: u64, enemy: u64) -> i32 {
	
	let m00 = (me >> 0*8) & 0b11111111u64;
	let m10 = (me >> 1*8) & 0b11111111u64;
	let m20 = (me >> 2*8) & 0b11111111u64;
	let m30 = (me >> 3*8) & 0b11111111u64;
	let m31 = (me >> 4*8) & 0b11111111u64;
	let m21 = (me >> 5*8) & 0b11111111u64;
	let m11 = (me >> 6*8) & 0b11111111u64;
	let m01 = (me >> 7*8) & 0b11111111u64;
	
	let e00 = (enemy >> 0*8) & 0b11111111u64;
	let e10 = (enemy >> 1*8) & 0b11111111u64;
	let e20 = (enemy >> 2*8) & 0b11111111u64;
	let e30 = (enemy >> 3*8) & 0b11111111u64;
	let e31 = (enemy >> 4*8) & 0b11111111u64;
	let e21 = (enemy >> 5*8) & 0b11111111u64;
	let e11 = (enemy >> 6*8) & 0b11111111u64;
	let e01 = (enemy >> 7*8) & 0b11111111u64;
	
	let me_flip = flip_diag_a1h8(me);
	let mf00 = (me_flip >> 0*8) & 0b11111111u64;
	let mf10 = (me_flip >> 1*8) & 0b11111111u64;
	let mf20 = (me_flip >> 2*8) & 0b11111111u64;
	let mf30 = (me_flip >> 3*8) & 0b11111111u64;
	let mf31 = (me_flip >> 4*8) & 0b11111111u64;
	let mf21 = (me_flip >> 5*8) & 0b11111111u64;
	let mf11 = (me_flip >> 6*8) & 0b11111111u64;
	let mf01 = (me_flip >> 7*8) & 0b11111111u64;
	
	let enemy_flip = flip_diag_a1h8(enemy);
	let ef00 = (enemy_flip >> 0*8) & 0b11111111u64;
	let ef10 = (enemy_flip >> 1*8) & 0b11111111u64;
	let ef20 = (enemy_flip >> 2*8) & 0b11111111u64;
	let ef30 = (enemy_flip >> 3*8) & 0b11111111u64;
	let ef31 = (enemy_flip >> 4*8) & 0b11111111u64;
	let ef21 = (enemy_flip >> 5*8) & 0b11111111u64;
	let ef11 = (enemy_flip >> 6*8) & 0b11111111u64;
	let ef01 = (enemy_flip >> 7*8) & 0b11111111u64;
	
	let me_score = EG_WEIGHTS[0][m00 as usize] + EG_WEIGHTS[0][m01 as usize] +
		EG_WEIGHTS[1][m10 as usize] + EG_WEIGHTS[1][m11 as usize] +
		EG_WEIGHTS[2][m20 as usize] + EG_WEIGHTS[2][m21 as usize] +
		EG_WEIGHTS[3][m30 as usize] + EG_WEIGHTS[3][m31 as usize] +
		EG_WEIGHTS[0][mf00 as usize] + EG_WEIGHTS[0][mf01 as usize] +
		EG_WEIGHTS[1][mf10 as usize] + EG_WEIGHTS[1][mf11 as usize] +
		EG_WEIGHTS[2][mf20 as usize] + EG_WEIGHTS[2][mf21 as usize] +
		EG_WEIGHTS[3][mf30 as usize] + EG_WEIGHTS[3][mf31 as usize];
	
	let enemy_score = EG_WEIGHTS[0][e00 as usize] + EG_WEIGHTS[0][e01 as usize] +
		EG_WEIGHTS[1][e10 as usize] + EG_WEIGHTS[1][e11 as usize] +
		EG_WEIGHTS[2][e20 as usize] + EG_WEIGHTS[2][e21 as usize] +
		EG_WEIGHTS[3][e30 as usize] + EG_WEIGHTS[3][e31 as usize] +
		EG_WEIGHTS[0][ef00 as usize] + EG_WEIGHTS[0][ef01 as usize] +
		EG_WEIGHTS[1][ef10 as usize] + EG_WEIGHTS[1][ef11 as usize] +
		EG_WEIGHTS[2][ef20 as usize] + EG_WEIGHTS[2][ef21 as usize] +
		EG_WEIGHTS[3][ef30 as usize] + EG_WEIGHTS[3][ef31 as usize];
	
	return me_score - enemy_score;
	
}

/// Solves the endgame.
/// Fail-soft negamax
/// Returns (move, eval)
pub fn solve_endgame_root(me: u64, enemy: u64, mut alpha: i8, beta: i8) -> (u8, i8) {
	
	// if the game is over, evaluate who won
	if game_over(me, enemy) {
		return (65, evaluation(me, enemy));
	}
	
	// get possible moves
	let moves = generate_moves(me, enemy);
	
	// if no moves, pass
	if moves == 0 {
		return (65, -solve_endgame_mo(enemy, me, -beta, -alpha, 7));
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
	states.sort_by_cached_key(|(_, me, enemy)| -heuristic_eg_nega(*me, *enemy));
	
	let mut best_score = -127;
	let mut best_move: u8 = 65;
	
	// for each child state
	for (mov, me, enemy) in states {
		
		let q = -solve_endgame_mo(enemy, me, -beta, -alpha, 7);
		
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
fn solve_endgame_nomo(me: u64, enemy: u64, mut alpha: i8, beta: i8) -> i8 {
	
	// if the game is over, evaluate who won
	if game_over(me, enemy) {
		return evaluation(me, enemy);
	}
	
	// get possible moves
	let moves = generate_moves(me, enemy);
	
	// if no moves, pass
	if moves == 0 {
		return -solve_endgame_nomo(enemy, me, -beta, -alpha);
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
			let q = -solve_endgame_nomo(enemy, me, -beta, -alpha);
			
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

/// Fail-soft negamax for endgame solving
/// Uses move ordering for states with more than stop_mo_at_empties number of empty disks
/// Optimal stop_mo_at_empties=7
/// https://www.chessprogramming.org/Alpha-Beta
fn solve_endgame_mo(me: u64, enemy: u64, mut alpha: i8, beta: i8, stop_mo_at_empties: u8) -> i8 {
	
	// TODO: This is slower than java. why?
	
	// if the game is over, evaluate who won
	if game_over(me, enemy) {
		return evaluation(me, enemy);
	}
	
	// get possible moves
	let moves = generate_moves(me, enemy);
	
	// if no moves, pass
	if moves == 0 {
		return -solve_endgame_mo(enemy, me, -beta, -alpha, stop_mo_at_empties);
	}
	
	// apply each move and get the state
	let mut states: Vec<(u64, u64)> = to_idx_move_vec(moves)
		.iter()
		.map(|mov| make_move(1u64 << *mov, me, enemy))
		.collect();
	
	// sort the child states, best one first
	// benchmark: sort_by_key=39.54s, sort_unstable_by_key=39.79s, sort_by_cached_key=38.74s
	states.sort_by_cached_key(|(me, enemy)| -heuristic_eg_nega(*me, *enemy));
	
	let mut best_score = -127;
	let empty_disks = empty_disks(me, enemy);
	
	// for each child state
	for (me, enemy) in states {
		
		let q = if empty_disks > stop_mo_at_empties {
			-solve_endgame_mo(enemy, me, -beta, -alpha, stop_mo_at_empties)
		} else {
			-solve_endgame_nomo(enemy, me, -beta, -alpha)
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

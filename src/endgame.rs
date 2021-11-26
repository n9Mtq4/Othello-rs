use crate::heuristic::heuristic_eg_slow_nega;
use crate::othello_board::{empty_disks, evaluation, game_over, generate_moves, make_move, to_idx_move_vec};

/// Solves the endgame.
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
	states.sort_by_cached_key(|(_, me, enemy)| -heuristic_eg_slow_nega(*me, *enemy));
	
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
	states.sort_by_cached_key(|(me, enemy)| -heuristic_eg_slow_nega(*me, *enemy));
	
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

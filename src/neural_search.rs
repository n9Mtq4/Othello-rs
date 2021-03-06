use tch::CModule;
use crate::neural_heuristic::{nnpredict_batch, nnpredict_d1, nnpredict_dn};
use crate::othello_board::{game_over, generate_moves, make_move, next_bit_move, to_bit_move_vec, to_idx_move_vec, wld_evaluation};

/// optimal depth until the bottom of the tree to stop move ordering
const BEST_STOP_MO_AT_DEPTH: i8 = if cfg!(feature = "large_batch") {
	2 // optimal mo cutoff is 2 for large batch
} else {
	2 // optimal mo cutoff is 2 for cpu
};

/// Use neural network to evaluate a state
#[inline(always)]
fn nnsearch_heuristic(model: &CModule, me: u64, enemy: u64) -> i32 {
	if cfg!(feature = "large_batch") {
		// optimal batch depth is 3 for a GPU
		nnpredict_dn(model, me, enemy, 3)
	} else {
		// optimal batch depth is 1 for CPU
		nnpredict_d1(model, me, enemy)
	}
}

/// Perform a mid-game evaluation on a node
/// Call with alpha=-640000, beta=640000
/// Returns (move, centidisk eval from current player's POV)
pub fn nnsearch_root(model: &CModule, me: u64, enemy: u64, mut alpha: i32, beta: i32, depth: i8) -> (u8, i32) {
	
	// if the game is over, evaluate who won
	if game_over(me, enemy) {
		return (65, 100 * (wld_evaluation(me, enemy) as i32));
	}
	
	// if the depth is 0, evaluate the position with the nn
	if depth <= 0 {
		return (65, nnsearch_heuristic(model, me, enemy));
	}
	
	// get possible moves
	let moves = generate_moves(me, enemy);
	
	// if no moves, pass
	if moves == 0 {
		return (65, -nnsearch_mo(model, enemy, me, -beta, -alpha, depth - 1, BEST_STOP_MO_AT_DEPTH));
	}
	
	// apply each move and get the state
	// TODO: optimize sorting here
	let states: Vec<(u8, u64, u64)> = to_idx_move_vec(moves)
		.iter()
		.map(|mov| {
			let (new_me, new_enemy) = make_move(1u64 << *mov, me, enemy);
			(*mov, new_me, new_enemy)
		})
		.collect();
	
	// run a prediction on every state
	let keys: Vec<f32> = nnpredict_batch(model, &states.iter().map(|(_, m, e)| (*m, *e)).collect());
	
	// map the predictions to the states (applied move, me, enemy, q)
	let mut states: Vec<(u8, u64, u64, i32)> = states
		.iter()
		.zip(keys.iter())
		.map(|((mov, m, e), q)| (*mov, *m, *e, (100.0 * 64.0 * (*q)) as i32))
		.collect();
	
	// sort the child states, best one first
	// benchmark: sort_by_key=39.54s, sort_unstable_by_key=39.79s, sort_by_cached_key=38.74s
	states.sort_by_key(|(_, _, _, q)| -(*q));
	
	let mut best_score = -640000;
	let mut best_move = 65;
	
	// for each child state
	for (mov, me, enemy, _) in states {
		
		let q = -nnsearch_mo(model, enemy, me, -beta, -alpha, depth - 1, BEST_STOP_MO_AT_DEPTH);
		
		if q >= beta {
			return (mov, q);
		}
		
		if q > best_score {
			best_score = q;
			best_move = mov;
			if q > alpha {
				alpha = q;
			}
		}
		
	}
	
	return (best_move, best_score);
	
}

/// Get the mid-game evaluation of a board without move ordering
/// Call with alpha=-640000, beta=640000
/// returns centidisk eval from current player's POV
fn nnsearch_nomo(model: &CModule, me: u64, enemy: u64, mut alpha: i32, beta: i32, depth: i8) -> i32 {
	
	// if the game is over, evaluate who won
	if game_over(me, enemy) {
		return 100 * (wld_evaluation(me, enemy) as i32);
	}
	
	// if the depth is 0, evaluate the position with the nn
	if depth <= 0 {
		return nnsearch_heuristic(model, me, enemy);
	}
	
	// get possible moves
	let mut moves = generate_moves(me, enemy);
	
	// if no moves, pass
	if moves == 0 {
		return -nnsearch_nomo(model, enemy, me, -beta, -alpha, depth - 1);
	}
	
	let mut best_score = -640000;
	
	// for each move
	while moves != 0 {
		
		let mov = next_bit_move(&mut moves);
		
		// evaluate the child state
		let (me, enemy) = make_move(mov, me, enemy);
		let q = -nnsearch_nomo(model, enemy, me, -beta, -alpha, depth - 1);
		
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

/// Get the mid-game evaluation of a board with move ordering
/// Call with alpha=-640000, beta=640000
/// Optimal stop_mo_at_depth=3
/// returns centidisk eval from current player's POV
fn nnsearch_mo(model: &CModule, me: u64, enemy: u64, mut alpha: i32, beta: i32, depth: i8, stop_mo_at_depth: i8) -> i32 {
	
	// if the game is over, evaluate who won
	if game_over(me, enemy) {
		return 100 * (wld_evaluation(me, enemy) as i32);
	}
	
	// if the depth is 0, evaluate the position with the nn
	if depth <= 0 {
		return nnsearch_heuristic(model, me, enemy);
	}
	
	// get possible moves
	let moves = generate_moves(me, enemy);
	
	// if no moves, pass
	if moves == 0 {
		return -nnsearch_mo(model, enemy, me, -beta, -alpha, depth - 1, stop_mo_at_depth);
	}
	
	// apply each move and get the state
	// TODO: optimize sorting here
	let states: Vec<(u64, u64)> = to_bit_move_vec(moves)
		.iter()
		.map(|mov| make_move(*mov, me, enemy))
		.collect();
	
	let keys: Vec<f32> = nnpredict_batch(model, &states);
	
	let mut states: Vec<(u64, u64, i32)> = states
		.iter()
		.zip(keys.iter())
		.map(|((m, e), q)| (*m, *e, (100.0 * 64.0 * (*q)) as i32))
		.collect();
	
	// sort the child states, best one first
	// benchmark: sort_by_key=39.54s, sort_unstable_by_key=39.79s, sort_by_cached_key=38.74s
	states.sort_by_key(|(_, _, q)| -(*q));
	
	let mut best_score = -640000;
	
	// for each child state
	for (me, enemy, _) in states {
		
		let q = if depth > stop_mo_at_depth {
			-nnsearch_mo(model, enemy, me, -beta, -alpha, depth - 1, stop_mo_at_depth)
		} else {
			-nnsearch_nomo(model,enemy, me, -beta, -alpha, depth - 1)
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

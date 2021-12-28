#![allow(dead_code)]

use tch::{CModule, Device, Tensor};
use crate::othello_board::{evaluation, game_over, generate_moves, make_move, next_bit_move, to_bit_move_vec};

/// Convert an othello board into a tensor
fn board_to_tensor(me: u64, enemy: u64) -> Tensor {
	
	let mut data: [f32; 64] = [0.0; 64];
	
	for i in 0..64 {
		if me & (1 << i) != 0 {
			data[i] = 1.0;
		} else if enemy & (1 << i) != 0 {
			data[i] = -1.0;
		}
	}
	
	Tensor::of_slice(&data)
	
}

/// Performs a prediction on a vector othello boards in 1 batch
/// Returns a vector q such that `q[i]` is the eval of `v[i]`
pub fn nnpredict_batch(model: &CModule, v: &Vec<(u64, u64)>) -> Vec<f32> {
	
	// map boards to tensors
	let states: Vec<Tensor> = v
		.iter()
		.map(|(m, e)| board_to_tensor(*m, *e))
		.collect();
	
	// stack all board tensors into 1 batch
	let t = if cfg!(feature = "gpu") {
		Tensor::stack(&states, 0).to(Device::Cuda(0))
	} else {
		Tensor::stack(&states, 0)
	};
	
	// run through model
	let output: Tensor = if cfg!(feature = "gpu") {
		model.forward_ts(&[t])
			.expect("model prediction failed")
			.to(Device::Cpu)
	} else {
		model.forward_ts(&[t])
			.expect("model prediction failed")
	};
	
	Vec::from(output)
	
}

/// Performs a prediction on a state. Performs minimax down to depth of `depth`.
/// All states in the minimax tree are batched and evaluated together.
pub fn nnpredict_dn(model: &CModule, me: u64, enemy: u64, depth: i8) -> i32 {
	
	// TODO: determine optimal capacity
	// 4096 is sufficient for depth=3
	let mut tensors: Vec<Tensor> = Vec::with_capacity(4096);
	
	// negamax to collect batch
	board_children_to_flat_tensor(&mut tensors, me, enemy, depth);
	
	// if there are no moves 3 plys down (even with moves for us), tensors is empty.
	// If this is the case, stacking will fail. If this is the case, rerun with a lesser depth
	// Ex: depth=3, fails on (me=18446744043523145728, enemy=3948544) without this check
	if tensors.is_empty() {
		return nnpredict_dn(model, me, enemy, depth - 1);
	}
	
	// stack all states in to 1 batch
	let t = if cfg!(feature = "gpu") {
		Tensor::stack(&tensors, 0).to(Device::Cuda(0))
	} else {
		Tensor::stack(&tensors, 0)
	};
	
	// run through model
	let output: Tensor = if cfg!(feature = "gpu") {
		model.forward_ts(&[t])
			.expect("model prediction failed")
			.to(Device::Cuda(0))
	} else {
		model.forward_ts(&[t])
			.expect("model prediction failed")
	};
	
	let result_vec: Vec<f32> = Vec::from(output);
	
	// negamax to evaluate using result from batch
	(100.0 * 64.0 * negamax_vec(&result_vec, me, enemy, depth, &mut 0)) as i32
	
}

/// Perform negamax on a state, using evaluations stored in a vector `v`.
/// Fail-soft framework
fn negamax_vec(v: &Vec<f32>, me: u64, enemy: u64, depth: i8, vec_idx: &mut usize) -> f32 {
	
	if game_over(me, enemy) {
		return evaluation(me, enemy) as f32;
	}
	
	// if the depth is 0, uses the evaluation stored in the vector
	if depth <= 0 {
		(*vec_idx) += 1;
		return v[(*vec_idx) - 1];
	}
	
	// get possible moves
	let mut moves = generate_moves(me, enemy);
	
	// if no moves, pass
	if moves == 0 {
		return negamax_vec(v, enemy, me, depth - 1, vec_idx);
	}
	
	// let num_moves: usize = moves.count_ones() as usize;
	let mut best_score = -127.0;
	
	// for each move
	while moves != 0 {
		
		let mov = next_bit_move(&mut moves);
		
		// evaluate the child state
		let (me, enemy) = make_move(mov, me, enemy);
		let q = -negamax_vec(v, enemy, me, depth - 1, vec_idx);
		
		if q > best_score {
			best_score = q;
		}
		
	}
	
	return best_score;
	
}

/// put all children board tensors to a depth of `depth` into a flat vector.
fn board_children_to_flat_tensor(v: &mut Vec<Tensor>, me: u64, enemy: u64, depth: i8) {
	
	// if the game is over, evaluate who won
	if game_over(me, enemy) {
		return;
	}
	
	// if the depth is 0, push the tensor to the vector
	if depth <= 0 {
		v.push(board_to_tensor(me, enemy));
		return;
	}
	
	// get possible moves
	let mut moves = generate_moves(me, enemy);
	
	// if no moves, pass
	if moves == 0 {
		return board_children_to_flat_tensor(v, enemy, me, depth - 1);
	}
	
	// for each move
	while moves != 0 {
		
		let mov = next_bit_move(&mut moves);
		
		let (m, e) = make_move(mov, me, enemy);
		board_children_to_flat_tensor(v, e, m, depth - 1);
		
	}
	
}

/// Perform a prediction on an othello board
/// Goes down 1 depth and runs the prediction on all children in 1 batch
pub fn nnpredict_d1(model: &CModule, me: u64, enemy: u64) -> i32 {
	
	// get all moves
	let moves = generate_moves(me, enemy);
	
	// if no moves, the game must have ended, so return endgame evaluation
	if moves == 0 {
		return 100 * (evaluation(me, enemy) as i32);
	}
	
	// Get all the state's children to tensors
	let states: Vec<Tensor> = to_bit_move_vec(moves)
		.iter()
		.map(|mov| {
			let (enemy, me) = make_move(*mov, me, enemy);
			board_to_tensor(me, enemy)
		})
		.collect();
	
	// move all tensors into 1 batch
	let t = if cfg!(feature = "gpu") {
		Tensor::stack(&states, 0).to(Device::Cuda(0))
	} else {
		Tensor::stack(&states, 0)
	};
	
	// perform prediction on batch & do 1 level of negamax
	let output: Tensor = model.forward_ts(&[t])
		.expect("model prediction failed");
	
	let best_child: Tensor = if cfg!(feature = "gpu") {
		output.min().to(Device::Cpu)
	} else {
		output.min()
	};
	
	(-100.0 * 64.0 * f32::from(best_child)) as i32
	
}

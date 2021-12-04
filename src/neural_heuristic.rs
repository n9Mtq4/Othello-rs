#![allow(dead_code)]

use tch::{CModule, Tensor};
use crate::othello_board::{evaluation, generate_moves, make_move, to_bit_move_vec};

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
	let t = Tensor::stack(&states, 0);
	
	let output: Tensor = model.forward_ts(&[t]).expect("model prediction failed");
	
	Vec::from(output)
	
}

/// Perform a predicton on an othello board
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
	let t = Tensor::stack(&states, 0);
	
	// perform prediction on batch & do 1 level of negamax
	let output: Tensor = model.forward_ts(&[t]).expect("model prediction failed");
	let best_child: Tensor = output.min();
	
	(-100.0 * 64.0 * f32::from(best_child)) as i32
	
}

/// Predict on a single state
fn nnpredict_d0(model: &CModule, me: u64, enemy: u64) -> i32 {
	
	let output: Tensor = model.forward_ts(&[board_to_tensor(me, enemy)]).expect("model prediction failed");
	(100.0 * 64.0 * f32::from(output)) as i32
	
}

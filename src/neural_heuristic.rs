use std::slice::Iter;
use tch::{CModule, Tensor};
use crate::othello_board::{generate_moves, make_move, to_bit_move_vec, to_idx_move_vec};

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

pub fn nnpredict_batch(model: &CModule, v: &Vec<(u64, u64)>) -> Vec<f32> {
	
	let states: Vec<Tensor> = v
		.iter()
		.map(|(m, e)| board_to_tensor(*m, *e))
		.collect();
	
	let t = Tensor::stack(&states, 0);
	
	let output: Tensor = model.forward_ts(&[t]).expect("model prediction failed");
	
	let v: Vec<f32> = Vec::from(output);
	
	v
	
}

pub fn nnpredict_d1(model: &CModule, me: u64, enemy: u64) -> i32 {
	
	let moves = generate_moves(me, enemy);
	let states: Vec<Tensor> = to_bit_move_vec(moves)
		.iter()
		.map(|mov| {
			let (enemy, me) = make_move(*mov, me, enemy);
			board_to_tensor(me, enemy)
		})
		.collect();
	
	let t = Tensor::stack(&states, 0);
	
	let output: Tensor = model.forward_ts(&[t]).expect("model prediction failed");
	let output: Tensor = output * -1;
	let best_child: Tensor = output.max();
	
	(100.0 * 64.0 * f32::from(best_child)) as i32
	
}

fn nnpredict_d0(model: &CModule, me: u64, enemy: u64) -> i32 {
	
	let output: Tensor = model.forward_ts(&[board_to_tensor(me, enemy)]).expect("model prediction failed");
	(100.0 * 64.0 * f32::from(output)) as i32
	
}

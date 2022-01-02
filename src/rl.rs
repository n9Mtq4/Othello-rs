use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use tch::{Device, Kind, Tensor};
use crate::neural_heuristic::nnpredict_dn;
use crate::othello_board::empty_disks;

pub fn rl_network() {
	
	// load pytorch model
	let mut model = tch::CModule::load("data/model.pt").unwrap();
	model.to(Device::Cuda(0), Kind::Float, false);
	model.set_eval();
	
	// read file
	let file = File::open("/mnt/L4/workspace/OthelloGui/train_data/unique.csv").unwrap();
	let reader = BufReader::new(file);
	
	let out_file = File::create("/mnt/L4/workspace/OthelloGui/train_data/unique_q.csv").unwrap();
	let mut writer = BufWriter::new(out_file);
	
	let mut i: i64 = 0;
	for line in reader.lines() {
		let line = line.unwrap();
		
		let tokens: Vec<&str> = line.split(",").collect();
		
		let ply: u8 = tokens[0].parse().unwrap();
		let black: u64 = tokens[1].parse().unwrap();
		let white: u64 = tokens[2].parse().unwrap();
		
		if empty_disks(black, white) < 10 {
			continue;
		}
		
		let me;
		let enemy;
		
		if ply & 1 == 0 {
			me = black;
			enemy = white;
		} else {
			me = white;
			enemy = black;
		}
		
		let q = nnpredict_dn(&model, me, enemy, 2);
		
		if q == i32::MIN {
			continue;
		}
		
		write!(&mut writer, "{},{}\n", line, q);
		
		if i % 10000 == 0 {
			println!("position {}", i);
			writer.flush();
		}
		
		i += 1;
		
	}
	
	writer.flush();
	
}

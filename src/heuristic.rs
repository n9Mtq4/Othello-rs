use crate::othello_board::number_of_moves;

const STABLE_2_0: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000001_00000011;
const STABLE_2_1: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_10000000_11000000;
const STABLE_2_2: u64 = 0b00000011_00000001_00000000_00000000_00000000_00000000_00000000_00000000;
const STABLE_2_3: u64 = 0b11000000_10000000_00000000_00000000_00000000_00000000_00000000_00000000;
const STABLE_3_0: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000001_00000011_00000111;
const STABLE_3_1: u64 = 0b00000111_00000011_00000001_00000000_00000000_00000000_00000000_00000000;
const STABLE_3_2: u64 = 0b00000000_00000000_00000000_00000000_00000000_10000000_11000000_11100000;
const STABLE_3_3: u64 = 0b11100000_11000000_10000000_00000000_00000000_00000000_00000000_00000000;

pub fn heuristic_fast(ply: u8, black: u64, white: u64) -> i32 {
	
	let moves = if ply & 1 == 0 {
		number_of_moves(black, white)
	} else {
		number_of_moves(white, black)
	};
	
	if moves == 0 {
		return 1000000i32 * ((black.count_ones() as i32) - (white.count_ones() as i32));
	}
	
	let board_sum = heuristic_board_weight(black, white);
	let stability = heuristic_stability(black, white);
	
	return 8i32 * board_sum +
		202 * -(2 * ((ply & 1) as i32) - 1) * (moves as i32) +
		2016 * stability;
	
}

fn heuristic_board_weight(black: u64, white: u64) -> i32 {
	
	let mut board_sum: i32 = 0;
	
	board_sum += 120i32 * ((black & 9295429630892703873u64).count_ones() - (white & 9295429630892703873u64).count_ones()) as i32;
	board_sum += -20i32 * ((black & 4792111478498951490u64).count_ones() - (white & 4792111478498951490u64).count_ones()) as i32;
	board_sum += 20i32 * ((black & 2594215222373842980u64).count_ones() - (white & 2594215222373842980u64).count_ones()) as i32;
	board_sum += 5i32 * ((black & 1729382813125312536u64).count_ones() - (white & 1729382813125312536u64).count_ones()) as i32;
	board_sum += -40i32 * ((black & 18577348462920192u64).count_ones() - (white & 18577348462920192u64).count_ones()) as i32;
	board_sum += -5i32 * ((black & 16961350949551104u64).count_ones() - (white & 16961350949551104u64).count_ones()) as i32;
	board_sum += 15i32 * ((black & 39582420959232u64).count_ones() - (white & 39582420959232u64).count_ones()) as i32;
	board_sum += 3i32 * ((black & 26646985310208u64).count_ones() - (white & 26646985310208u64).count_ones()) as i32;
	
	return board_sum;
	
}

fn heuristic_stability(black: u64, white: u64) -> i32 {
	
	// this gets vectorized with -C opt-level=3 -C target-cpu=native
	
	let mut stability: i32 = 0;
	
	if black & STABLE_2_0 == STABLE_2_0 {
		stability += 1;
		if black & STABLE_3_0 == STABLE_3_0 {
			stability += 1;
		}
	}
	if black & STABLE_2_1 == STABLE_2_1 {
		stability += 1;
		if black & STABLE_3_1 == STABLE_3_1 {
			stability += 1;
		}
	}
	if black & STABLE_2_2 == STABLE_2_2 {
		stability += 1;
		if black & STABLE_3_2 == STABLE_3_2 {
			stability += 1;
		}
	}
	if black & STABLE_2_3 == STABLE_2_3 {
		stability += 1;
		if black & STABLE_3_3 == STABLE_3_3 {
			stability += 1;
		}
	}
	if white & STABLE_2_0 == STABLE_2_0 {
		stability += -1;
		if white & STABLE_3_0 == STABLE_3_0 {
			stability += -1;
		}
	}
	if white & STABLE_2_1 == STABLE_2_1 {
		stability += -1;
		if white & STABLE_3_1 == STABLE_3_1 {
			stability += -1;
		}
	}
	if white & STABLE_2_2 == STABLE_2_2 {
		stability += -1;
		if white & STABLE_3_2 == STABLE_3_2 {
			stability += -1;
		}
	}
	if white & STABLE_2_3 == STABLE_2_3 {
		stability += -1;
		if white & STABLE_3_3 == STABLE_3_3 {
			stability += -1;
		}
	}
	
	return stability;
	
}

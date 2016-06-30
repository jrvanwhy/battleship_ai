// Temporary warning bypass to let me develop the building blocks in peace
#![allow(dead_code)]
#![allow(unused_variables)]

// Board size (width and height)
//const BOARD_SIZE: u8 = 10;
const BOARD_SIZE: u8 = 5;

// This represents a board position.
// The positions are numbered in a row major manner,
// so position 0 is A1, position 9 is A10, and
// position 99 is J10. Larger values are invalid.
// This is different from the conventions for the different
// ship types
pub type BoardPos = u8;

// Construct a board position from its row and column parts
pub fn pos_from_parts(row: u8, col: u8) -> BoardPos {
	BOARD_SIZE * row + col
}

// Ship type
#[derive(Clone,Copy,Debug,PartialEq)]
enum ShipType {
	Patrol,
	Destroyer,
	Submarine,
	Battleship,
	Carrier
}

// A list of all ship types
const NUM_SHIP_TYPES: usize = 5;
const SHIP_TYPES: [ShipType; NUM_SHIP_TYPES] = [ShipType::Patrol, ShipType::Destroyer, ShipType::Submarine, ShipType::Battleship, ShipType::Carrier];

// Compute the ID (index into SHIP_TYPES) of the given ship type
fn stype_id(stype: ShipType) -> u8 {
	SHIP_TYPES.iter().position(|&t| t == stype).expect("Unknown ship type!!!") as u8
}

// Decode a ship type from a character describing it
fn decode_shiptype(desc: u8) -> ShipType {
	match desc as char {
		'P' => ShipType::Patrol,
		'D' => ShipType::Destroyer,
		'S' => ShipType::Submarine,
		'B' => ShipType::Battleship,
		'C' => ShipType::Carrier,
		_ => panic!("Invalid ship type {}", desc)
	}
}

// Compute the size of the given ship type
fn ship_size(shiptype: ShipType) -> u8 {
	use ShipType::*;

	match shiptype {
		Patrol => 2,
		Destroyer => 3,
		Submarine => 3,
		Battleship => 4,
		Carrier => 5,
	}
}

// The number of columns the ship can be in oriented horizontally
// or the number of rows it can be in oriented vertically
fn reduced_poscount(shiptype: ShipType) -> u8 {
	BOARD_SIZE - ship_size(shiptype) + 1
}

// Computes the number of valid positions for the given ship type
fn num_positions(shiptype: ShipType) -> u8 {
	// Consider rotations and the rectangular pattern of horizontal vs.
	// vertical positioning
	2 * reduced_poscount(shiptype) * BOARD_SIZE
}

// Compute the occupied squares for the given ship type and position ID
fn ship_range(shiptype: ShipType, pos: u8) -> Vec<u8> {
	// Starting square and step size for this ship's span
	let start_square;
	let step_size;

	// Lower-numbered positions are horizontal, higher-numbered positions
	// are vertically-oriented.
	if pos < num_positions(shiptype)/2 {
		// Horizontally oriented

		// Compute the starting square for the ship
		start_square = pos_from_parts(pos / reduced_poscount(shiptype), pos % reduced_poscount(shiptype));

		// Step size is just 1 for horizontal
		step_size = 1;
	} else {
		// Vertically oriented
		let pos = pos - num_positions(shiptype)/2;

		// Compute the starting square for the ship
		start_square = pos_from_parts(pos / BOARD_SIZE, pos % BOARD_SIZE);

		// 1 row per step
		step_size = BOARD_SIZE;
	}

	// Compute the ship span from the given starting square and step size
	(0..ship_size(shiptype)).map(|v| start_square + step_size * v).collect()
}

// Check if the given ship positions overlap
fn calc_has_overlap(ship1: ShipType, pos1: u8, ship2: ShipType, pos2: u8) -> bool {
	let range1 = ship_range(ship1, pos1);
	let range2 = ship_range(ship2, pos2);

	range1.iter().any(|p| range2.contains(p))
}

// Generate the overlap cache
fn gen_overlap_cache() -> [[Vec<Vec<bool>>; NUM_SHIP_TYPES]; NUM_SHIP_TYPES] {
	// The variable we will be outputting
	let mut out = [[Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
	               [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
	               [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
	               [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
	               [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()]];

	// Go through each ship type combination and fill out the overlap vector
	for stype1_idx in 0..NUM_SHIP_TYPES {
		let stype1 = SHIP_TYPES[stype1_idx];

		for stype2_idx in 0..NUM_SHIP_TYPES {
			let stype2 = SHIP_TYPES[stype2_idx];

			// Resize the vector to be as minimal as possible
			out[stype1_idx][stype2_idx] = Vec::with_capacity(num_positions(stype1) as usize);

			// Iterate through the first ship positions and push back vectors of overlap solutions
			for pos1 in 0..num_positions(stype1) {
				out[stype1_idx][stype2_idx].push((0..num_positions(stype2)).map(|pos2| {
					calc_has_overlap(stype1, pos1, stype2, pos2)
				}).collect());
			}
		}
	}

	out
}

// Fast overlap checker (uses the provided cache)
#[inline]
fn has_overlap(ship1: ShipType, pos1: u8, ship2: ShipType, pos2: u8, cache: &[[Vec<Vec<bool>>; NUM_SHIP_TYPES]; NUM_SHIP_TYPES]) -> bool {
	cache[stype_id(ship1) as usize][stype_id(ship2) as usize][pos1 as usize][pos2 as usize]
}

// Read in the moves list from the input file
fn read_moves() -> Vec<(BoardPos, Option<ShipType>)> {
	use std::io::BufRead;

	// Open the moves file and create a read buffer for it (needed for line-by-line reading)
	let filereader = std::io::BufReader::new(std::fs::File::open("moves.txt").expect("Unable to open moves.txt"));

	// Generate the output vector by processing moves.txt line-by-line
	filereader.lines().map(|line| {
		let line = line.expect("Unable to read line in moves.txt");
		let line_bytes = line.as_bytes();

		let (pos_col, type_idx) =
			if (line_bytes.len() >= 3) && (line_bytes[2] == '0' as u8) {
				(9, 3)
			} else {
				(line_bytes[1] as u8 - '1' as u8, 2)
			};

		(pos_from_parts(line_bytes[0] as u8 - 'A' as u8, pos_col),
			if line_bytes.len() > type_idx {
				Some(decode_shiptype(line_bytes[type_idx]))
			} else {
				None
			}
		)
	}).collect()
}

// Apply the effect of a miss on the position lists
fn process_miss(pos_positions: &mut Vec<Vec<u8>>, pos: BoardPos) {
	for stype_idx in 0..pos_positions.len() {
		let plist = &mut pos_positions[stype_idx];

		let mut idx = 0;

		while idx < plist.len() {
			// Check if the given position overlaps the miss
			if ship_range(SHIP_TYPES[stype_idx], plist[idx]).contains(&pos) {
				plist.swap_remove(idx);
			} else {
				idx += 1;
			}
		}
	}
}

// Apply the effect of a hit on the given ship type
fn process_hit(poslist: &mut Vec<u8>, stype: ShipType, pos: BoardPos) {
	let mut idx = 0;

	while idx < poslist.len() {
		// Check if the given position overlaps the hit
		if ship_range(stype, poslist[idx]).contains(&pos) {
			// It overlaps, so this position is acceptable
			idx += 1;
		} else {
			// No overlap; remove this position
			poslist.swap_remove(idx);
		}
	}
}

// Apply the effect of a known move result on the list of possible positions
fn apply_move(pos_positions: &mut Vec<Vec<u8>>, move_val: (BoardPos, Option<ShipType>)) {
	// We operate completely differently depending on whether it was a hit or miss
	match move_val.1 {
		None => {
			// It was a miss. Remove BoardPos from all position lists
			process_miss(pos_positions, move_val.0);
		},
		Some(stype) => {
			// It was a hit. Make sure that the relevant ship type
			// overlaps the hit position
			process_hit(&mut pos_positions[stype_id(stype) as usize], stype, move_val.0);
		},
	}
}

fn main() {
	// The list of possible moves per ship type
	let mut pos_positions = SHIP_TYPES.iter().map(|&stype| (0..num_positions(stype)).collect::<Vec<_>>()).collect::<Vec<_>>();

	// Load in the moves file and process the moves
	for cur_move in read_moves() {
		apply_move(&mut pos_positions, cur_move);
	}

	// Generate the ship position overlap cache
	let olap_cache = gen_overlap_cache();
}

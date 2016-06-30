// Board size (width and height)
const BOARD_SIZE: u8 = 10;

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
#[derive(Clone,Copy,Debug)]
enum ShipType {
	Patrol,
	Destroyer,
	Submarine,
	Battleship,
	Carrier
}

// A list of all ship types
const SHIP_TYPES: [ShipType; 5] = [ShipType::Patrol, ShipType::Destroyer, ShipType::Submarine, ShipType::Battleship, ShipType::Carrier];

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
fn has_overlap(ship1: ShipType, pos1: u8, ship2: ShipType, pos2: u8) -> bool {
	let range1 = ship_range(ship1, pos1);
	let range2 = ship_range(ship2, pos2);

	range1.iter().any(|p| range2.contains(p))
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

fn main() {
	println!("{:?}", has_overlap(ShipType::Carrier, 71, ShipType::Patrol, 9));
}

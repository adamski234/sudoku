use std::fmt::Display;

const BOARD_SIZE: u8 = 9;

#[derive(Debug, PartialEq, Eq)]
pub struct SudokuBoard {
	// index = row * BOARD_SIZE + column
	data: [SudokuSpot; (BOARD_SIZE * BOARD_SIZE) as usize]
}

impl SudokuBoard {
	pub fn fill_row_from_string(&mut self, input: &str, row: u8) {
		for (column, part) in input.split(' ').enumerate() {
			let digit: u8 = part.parse().unwrap();
			if digit != 0 {
				self.set_tile_to_number(digit, row, column as u8);
			}
		}
	}
	pub fn get_spot_at_coords(&self, row: u8, column: u8) -> SudokuSpot {
		return self.data[(row * BOARD_SIZE + column) as usize];
	}
	pub fn get_spot_at_coords_by_ref(&mut self, row: u8, column: u8) -> &mut SudokuSpot {
		return &mut self.data[(row * BOARD_SIZE + column) as usize];
	}
	pub fn set_tile_to_number(&mut self, number: u8, row: u8, column: u8) {
		self.set_tile(SudokuSpot::Number(number), row, column);
	}
	pub fn set_tile(&mut self, value: SudokuSpot, row: u8, column: u8) {
		self.data[(row * BOARD_SIZE + column) as usize] = value;
	}
	pub fn is_column_valid(&self, column: u8) -> bool {
		let mut already_checked = Possibilities::default();
		for row in 0..9 {
			match self.get_spot_at_coords(row, column) {
				SudokuSpot::Number(number) => {
					if already_checked.is_number_marked(number) {
						return false;
					} else {
						already_checked.set_marked(number);
					}
				}
				_ => {
					continue;
				}
			}
		}
		return true;
	}
	pub fn is_row_valid(&self, row: u8) -> bool {
		let mut already_checked = Possibilities::default();
		for column in 0..9 {
			match self.get_spot_at_coords(row, column) {
				SudokuSpot::Number(number) => {
					if already_checked.is_number_marked(number) {
						return false;
					} else {
						already_checked.set_marked(number);
					}
				}
				_ => {
					continue;
				}
			}
		}
		return true;
	}
	pub fn is_square_valid(&self, row: u8, column: u8) -> bool {
		let square_top_row = (row / 3) * 3;
		let square_first_column = (column / 3) * 3;
		let mut already_checked = Possibilities::default();
		for row_offset in 0..3 {
			for column_offset in 0..3 {
				match self.get_spot_at_coords(square_top_row + row_offset, square_first_column + column_offset) {
					SudokuSpot::Number(number) => {
						if already_checked.is_number_marked(number) {
							already_checked.set_marked(number);
						} else {
							return false;
						}
					}
					_ => {
						continue;
					}
				}
			}
		}
		return true;
	}
	pub fn get_valid_numbers_for_spot(&self, row: u8, column: u8) -> Possibilities {
		return self.get_valid_numbers_for_row(row)
			.common_numbers(self.get_valid_numbers_for_column(column))
			.common_numbers(self.get_valid_numbers_for_square(row, column));
	}
	pub fn get_valid_numbers_for_row(&self, row: u8) -> Possibilities {
		let mut result = Possibilities::create_full();
		for column in 0..9 {
			let tile = self.get_spot_at_coords(row, column);
			match tile {
				SudokuSpot::Number(number) => {
					result.clear_marked(number);
				}
				_ => {
					continue;
				}
			}
		}
		return result;
	}
	pub fn get_valid_numbers_for_column(&self, column: u8) -> Possibilities {
		let mut result = Possibilities::create_full();
		for row in 0..9 {
			let tile = self.get_spot_at_coords(row, column);
			match tile {
				SudokuSpot::Number(number) => {
					result.clear_marked(number);
				}
				_ => {
					continue;
				}
			}
		}
		return result;
	}
	pub fn get_valid_numbers_for_square(&self, row: u8, column: u8) -> Possibilities {
		let square_top_row = (row / 3) * 3;
		let square_first_column = (column / 3) * 3;
		let mut result = Possibilities::create_full();
		for row_offset in 0..3 {
			for column_offset in 0..3 {
				match self.get_spot_at_coords(square_top_row + row_offset, square_first_column + column_offset) {
					SudokuSpot::Number(number) => {
						result.clear_marked(number);
					}
					_ => {
						continue;
					}
				}
			}
		}
		return result;
	}
	pub fn convert_all_single_possibilities(&mut self) -> bool {
		let mut was_changed = false;
		for row in 0..BOARD_SIZE {
			for column in 0..BOARD_SIZE {
				let tile = self.get_spot_at_coords(row, column);
				match tile {
					SudokuSpot::Possibilities(possibilities) => {
						match possibilities.find_only_marked_number() {
							Some(number) => {
								self.set_tile_to_number(number, row, column);
								self.update_board_possibilities_after_placing(number, row, column);
								was_changed = true;
							}
							None => {
								continue;
							}
						}
					}
					_ => {
						continue;
					}
				}
			}
		}
		return was_changed;
	}
	pub fn update_board_possibilities_after_placing(&mut self, number: u8, row: u8, column: u8) {
		self.update_row_after_placing(number, row);
		self.update_column_after_placing(number, column);
		self.update_square_after_placing(number, row, column);
	}
	pub fn update_row_after_placing(&mut self, number: u8, row: u8) {
		for column in 0..BOARD_SIZE {
			match self.get_spot_at_coords_by_ref(row, column) {
				SudokuSpot::Possibilities(possibilities) => {
					possibilities.clear_marked(number);
				}
				_ => {
					continue;
				}
			}
		}
	}
	pub fn update_column_after_placing(&mut self, number: u8, column: u8) {
		for row in 0..BOARD_SIZE {
			match self.get_spot_at_coords_by_ref(row, column) {
				SudokuSpot::Possibilities(possibilities) => {
					possibilities.clear_marked(number);
				}
				_ => {
					continue;
				}
			}
		}
	}
	pub fn update_square_after_placing(&mut self, number: u8, row: u8, column: u8) {
		let square_top_row = (row / 3) * 3;
		let square_first_column = (column / 3) * 3;
		for row_offset in 0..3 {
			for column_offset in 0..3 {
				match self.get_spot_at_coords_by_ref(square_top_row + row_offset, square_first_column + column_offset) {
					SudokuSpot::Possibilities(possibilities) => {
						possibilities.clear_marked(number);
					}
					_ => {
						continue;
					}
				}
			}
		}
	}
	pub fn fill_board_with_possibilities(&mut self) {
		for row in 0..BOARD_SIZE {
			for column in 0..BOARD_SIZE {
				match self.get_spot_at_coords(row, column) {
					SudokuSpot::Uninitialized => {
						self.set_tile(SudokuSpot::Possibilities(self.get_valid_numbers_for_spot(row, column)), row, column);
					}
					_ => {
						continue;
					}
				}
			}
		}
	}
	pub fn is_solved(&self) -> bool {
		//Board is solved when all tiles add up to 405 (9 * 45)
		let mut sum: u16 = 0;
		for i in 0..(BOARD_SIZE * BOARD_SIZE) {
			match self.data[i as usize] {
				SudokuSpot::Number(number) => {
					sum += number as u16;
				}
				_ => {
					continue;
				}
			}
		}
		return sum == 405;
	}
	pub fn solve(&mut self) -> bool {
		loop {
			if !self.convert_all_single_possibilities() {
				return false;
			}
			if self.is_solved() {
				break;
			}
		}
		return true;
	}
}

impl Display for SudokuBoard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for row in 0..BOARD_SIZE {
			for column in 0..BOARD_SIZE {
				match self.get_spot_at_coords(row, column) {
					SudokuSpot::Possibilities(pos) => {
						write!(f, "Pos({}), ", pos.data).unwrap();
					}
					SudokuSpot::Number(number) => {
						write!(f, "{}, ", number).unwrap();
					}
					SudokuSpot::Uninitialized => {
						write!(f, "Uninit, ").unwrap();
					}
				}
			}
			writeln!(f).unwrap();
		}
		return Ok(());
	}
}

impl Default for SudokuBoard {
	fn default() -> Self {
		return Self { data: [Default::default(); (BOARD_SIZE * BOARD_SIZE) as usize] }
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SudokuSpot {
	Number(u8),
	Uninitialized,
	Possibilities(Possibilities),
}

impl SudokuSpot {
	pub fn maybe_cast_into_number_from_possibilities(self) -> Self {
		match &self {
			Self::Possibilities(pos) => {
				match pos.find_only_marked_number() {
					Some(number) => {
						return Self::Number(number);
					}
					None => {
						return self;
					}
				}
			}
			_ => {
				return self;
			}
		}
	}
}

impl Default for SudokuSpot {
	fn default() -> Self {
		return Self::Uninitialized;
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Possibilities {
	pub data: u16,
}

impl Possibilities {
	pub fn create_full() -> Self {
		return Self { data: 0b01111111111 }; 
	}
	pub fn common_numbers(self, other: Self) -> Self {
		let mut result = self;
		result.data &= other.data;
		return result;
	}
	pub fn is_number_marked(&self, number: u8) -> bool {
		if self.data & (1 << number) != 0 {
			return true;
		} else {
			return false;
		}
	}
	pub fn set_marked(&mut self, number: u8) {
		self.data |= 1 << number;
	}
	pub fn clear_marked(&mut self, number: u8) {
		self.data &= !(1 << number);
	}
	pub fn find_only_marked_number(&self) -> Option<u8> {
		let mut was_marked_number_found = false;
		let mut first_marked_number = 0;
		for i in 1..=9 {
			if self.is_number_marked(i) {
				if !was_marked_number_found {
					first_marked_number = i;
					was_marked_number_found = true;
				} else {
					return None;
				}
			}
		}
		if first_marked_number == 0 {
			return None;
		} else {
			return Some(first_marked_number);
		}
	}
}

impl Default for Possibilities {
	fn default() -> Self {
		return Self {
			data: 0,
		}
	}
}
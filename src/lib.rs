const BOARD_SIZE: u8 = 9;

#[derive(Debug, PartialEq, Eq)]
pub struct SudokuBoard {
	// index = row * BOARD_SIZE + column
	pub data: [SudokuSpot; (BOARD_SIZE * BOARD_SIZE) as usize]
}

impl SudokuBoard {
	pub fn set_tile_to_number(&mut self, number: u8, row: u8, column: u8) {
		self.data[(row * BOARD_SIZE + column) as usize] = SudokuSpot::Number(number);
	}
	pub fn is_column_valid(&self, column: u8) -> bool {
		let mut already_checked = Possibilities::default();
		for row in 0..9 {
			match self.data[(row * BOARD_SIZE + column) as usize] {
				SudokuSpot::Number(number) => {
					if already_checked.is_number_valid(number) {
						return false;
					} else {
						already_checked.set_valid(number);
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
			match self.data[(row * BOARD_SIZE + column) as usize] {
				SudokuSpot::Number(number) => {
					if already_checked.is_number_valid(number) {
						return false;
					} else {
						already_checked.set_valid(number);
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
		return true;
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
				match pos.find_single_valid_number() {
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
	pub fn is_number_valid(&self, number: u8) -> bool {
		if self.data & (1 << number) != 0 {
			return true;
		} else {
			return false;
		}
	}
	pub fn set_valid(&mut self, number: u8) {
		self.data |= 1 << number;
	}
	pub fn clear_valid(&mut self, number: u8) {
		self.data &= !(1 << number);
	}
	pub fn find_single_valid_number(&self) -> Option<u8> {
		let mut was_valid_number_found = false;
		let mut first_valid_number = 0;
		for i in 1..=9 {
			if self.is_number_valid(i) {
				if !was_valid_number_found {
					first_valid_number = i;
					was_valid_number_found = true;
				} else {
					return None;
				}
			}
		}
		if first_valid_number == 0 {
			return None;
		} else {
			return Some(first_valid_number);
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
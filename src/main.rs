#![allow(clippy::needless_return)]

use sudoku::*;

fn main() {
	let mut penis = SudokuBoard::default();
	penis.set_tile_to_number(5, 1, 2);
	penis.set_tile_to_number(5, 2, 2);
	penis.set_tile_to_number(5, 2, 3);
	println!("{:#?}", penis.is_column_valid(2));
	println!("{:#?}", penis.is_row_valid(3));
	println!("{:#?}", penis.is_row_valid(2));
	println!("{:#?}", penis.is_column_valid(3));
	println!("{:#?}", penis.is_row_valid(1));
}
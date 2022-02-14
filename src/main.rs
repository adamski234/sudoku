#![allow(clippy::needless_return)]

use sudoku::*;

fn main() {
	let mut board = SudokuBoard::default();
	board.fill_row_from_string("0 0 0 6 0 0 0 3 8", 0);
	board.fill_row_from_string("1 8 6 0 3 7 0 4 2", 1);
	board.fill_row_from_string("7 0 4 8 5 2 0 9 1", 2);
	board.fill_row_from_string("0 0 0 0 7 9 0 0 4", 3);
	board.fill_row_from_string("6 7 0 4 0 5 0 0 0", 4);
	board.fill_row_from_string("0 0 9 0 0 0 8 0 0", 5);
	board.fill_row_from_string("3 0 7 0 0 0 2 0 6", 6);
	board.fill_row_from_string("0 0 0 5 0 0 0 0 7", 7);
	board.fill_row_from_string("8 9 0 0 2 0 4 0 0", 8);
	board.fill_board_with_possibilities();
	board.solve();
	println!("{}", board);
	let d = Possibilities { data: 129 };
	for i in 1..=9 {
		println!("{}", d.is_number_marked(i));
	}
	println!("{:#?}", d.find_only_marked_number());
}
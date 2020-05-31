use std::fmt;
use std::iter;
use crate::iterators::NeighborsIterator;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BoardWrapper {
    board: Board,
}

#[wasm_bindgen]
impl BoardWrapper {
    pub fn foo() -> String {
        "Hi, I'm foo".to_string()
    }

    pub fn new(width : usize, height : usize) -> BoardWrapper {
        let mut board = BoardWrapper { board: Board::new(width, height) };
        for i in 0..width * height {
            if i % 2 == 0 || i % 7 == 0 {
                board.board.grid[i] = true;
            }
        }

        board
    }

    pub fn tick(&mut self) {
        self.board.tick()
    }

    pub fn render(&self) -> String {
        self.board.render()
    }

    pub fn width(&self) -> usize {
        self.board.cols
    }

    pub fn height(&self) -> usize {
        self.board.rows
    }

    pub fn cells(&self) -> *const bool {
        self.board.grid.as_ptr()
    }
}

#[derive(Debug)]
pub struct Board {
    pub grid : Vec<bool>,
    pub cols : usize,
    pub rows : usize,
}

impl Board {
    pub fn new(width : usize, height : usize) -> Board {
        if width == 0 || height == 0 { panic!("Invalid board size: Array dimensions must be non-zero {} {}", width, height); }

        Board{grid : vec![false; width * height], cols : width, rows : height}
    }

    pub fn flip(&mut self, col : usize, row : usize){
        let idx = self.get_index(col, row);
        self.grid[idx] = !self.grid[idx];
    }

    fn get_index(&self, col: usize, row: usize) -> usize {
        assert!(col < self.cols);
        assert!(row < self.rows);

        row * self.cols + col
    }

    pub fn get(&self, col: usize, row:usize) -> bool {
        self.grid[self.get_index(col, row)]
    }

    pub fn set(&mut self, col: usize, row:usize, v: bool) {
        let idx = self.get_index(col, row);
        self.grid[idx] = v;
    }

    pub fn neighbors(&self, x : usize, y : usize) -> NeighborsIterator {
        NeighborsIterator::new(self, x, y)
    }

    pub fn tick(&mut self) {
        let flips =
            (0..self.cols)
                .flat_map(|x| iter::repeat(x).take(self.rows))
                .zip((0..self.rows).cycle())                        // all coordinates
            .map(|(x,y)| (
                    (x,y),
                    self.grid[self.get_index(x, y)],
                    self.neighbors(x,y).filter(|c| *c).count()))    // map to (point, value, count of live neighbors)
            .filter(|&(_, v, live_neighbors)|
                    (v && live_neighbors != 2 && live_neighbors != 3)
                    || (!v && live_neighbors == 3))
            .map(|(p, _, _)| p)
            .collect::<Vec<(usize, usize)>>();

        for f in flips {
            self.flip(f.0, f.1);
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.rows {
            for x in 0..self.cols {
                write!(f, "{}", if self.get(x,y) { '◼' } else { '◻' })?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    #[test]
    fn test_basic_creation() {
        let width = 30;
        let height = 27;
        let b = Board::new(width,height);
        assert_eq!(b.cols, width);
        assert_eq!(b.rows, height);
        assert_eq!(b.grid.len(), width * height);
    }

    #[test]
    #[should_panic(expected = "Invalid board size")]
    fn test_empty_board_creation() {
        let b = Board::new(0,0);
        println!("{:?}", b);
    }

    #[test]
    fn test_one_x_one_creation() {
        let mut b = Board::new(1,1);

        b.set(0, 0, true);
        assert_eq!(b.get(0,0), true);
        b.set(0, 0, false);
        assert_eq!(b.get(0,0), false);
    }

    #[test]
    fn test_population() {
        let width = 1999;
        let height = 1000;
        let mut b = Board::new(width, height);

        for point in (0..width).flat_map(|x| iter::repeat(x).take(height)).zip((0..height).cycle()) {
            let x = point.0;
            let y = point.1;
            b.set(x, y, (x + y) % 2 == 0);
        }

        for point in (0..width).flat_map(|x| iter::repeat(x).take(height)).zip((0..height).cycle()) {
            let x = point.0;
            let y = point.1;
            assert_eq!(b.get(x, y), (x + y) % 2 == 0);
        }
    }

    #[test]
    fn test_flip() {
        let width = 1999;
        let height = 1000;
        let mut b = Board::new(width, height);

        for point in (0..width).flat_map(|x| iter::repeat(x).take(height)).zip((0..height).cycle()) {
            let x = point.0;
            let y = point.1;
            b.set(x, y,  (x + y) % 2 == 0);
        }

        for point in (0..width).flat_map(|x| iter::repeat(x).take(height)).zip((0..height).cycle()) {
            b.flip(point.0, point.1);
        }

        for point in (0..width).flat_map(|x| iter::repeat(x).take(height)).zip((0..height).cycle()) {
            let x = point.0;
            let y = point.1;
            assert_eq!(b.get(x, y), (x + y) % 2 != 0);
        }
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds_col_get() {
        let b = Board::new(10,10);
        println!("{:?}", b.get(15, 5));
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds_row_get() {
        let b = Board::new(10,10);
        println!("{:?}", b.get(15, 5));
    }

    #[test]
    #[should_panic(expected = "col < self.cols")]
    fn test_out_of_bounds_col_set() {
        let mut b = Board::new(10,10);
        b.set(15, 5, true);
    }

    #[test]
    #[should_panic(expected = "col < self.cols")]
    fn test_out_of_bounds_row_set() {
        let mut b = Board::new(10,10);
        b.set(15, 5, true);
    }

    #[test]
    fn test_neighbors_iterator_all_empty() {
        let b = Board::new(10,10);
        for x in b.neighbors(1,1) {
            println!("{:?}", x);
            assert_eq!(x, false);
        }
    }

    #[test]
    fn test_boundary_iterator() {
        let b = Board::new(10,10);
        assert_eq!(b.neighbors(1,1).count(), 8);
        assert_eq!(b.neighbors(0,0).count(), 3);
        assert_eq!(b.neighbors(0,9).count(), 3);
        assert_eq!(b.neighbors(9,0).count(), 3);
        assert_eq!(b.neighbors(9,9).count(), 3);
        assert_eq!(b.neighbors(0,3).count(), 5);
        assert_eq!(b.neighbors(3,0).count(), 5);
        assert_eq!(b.neighbors(8,9).count(), 5);
        assert_eq!(b.neighbors(9,8).count(), 5);
    }

    #[test]
    fn test_neighbors_iterator_all_full() {

    }

    #[test]
    fn test_neighbors_iterator_mixed() {

    }
}

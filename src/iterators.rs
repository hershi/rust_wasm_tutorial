use super::board::*;
use std::iter;

#[derive(Debug)]
pub struct NeighborsIterator<'a> {
    board : &'a Board,
    points : Vec<(usize,usize)>,
    offset : usize,
}

impl <'a> NeighborsIterator<'a> {
    pub fn new(board : &Board, x : usize, y : usize) -> NeighborsIterator {
        let tx = x as isize;
        let ty = y as isize;
        let points = (-1..2).flat_map(|i| iter::repeat(i).take(3))
               .zip((-1..2).cycle())
               .filter(|&(i,j)| (i != 0 || j != 0))
               .map(|(i,j)| (tx + i, ty + j))
               .filter(|&(i,j)| i >= 0 && j >= 0)
               .filter(|&(i,j)| i < board.cols as isize && j < board.rows as isize)
               .map(|(i,j)| (i as usize, j as usize))
               .collect();

        NeighborsIterator {
            board : board,
            points : points,
            offset : 0,
        }
    }
}

impl <'a> Iterator for NeighborsIterator<'a> {
    type Item = bool;
    fn next(&mut self) -> Option<bool> {
        if self.offset >= self.points.len() {
            None
        } else {
            let point = self.points[self.offset];
            self.offset += 1;
            Some(self.board.get(point.0, point.1))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::*;
    use std::iter;
    #[test]
    fn test_neighbors_iterator_all_empty() {
        let b = Board::new(10,10);

        assert!(b.neighbors(1,1).all(|x| x == false));
        assert!(b.neighbors(0,0).all(|x| x == false));
        assert!(b.neighbors(0,9).all(|x| x == false));
        assert!(b.neighbors(9,0).all(|x| x == false));
        assert!(b.neighbors(9,9).all(|x| x == false));
        assert!(b.neighbors(0,3).all(|x| x == false));
        assert!(b.neighbors(3,0).all(|x| x == false));
        assert!(b.neighbors(8,9).all(|x| x == false));
        assert!(b.neighbors(9,8).all(|x| x == false));
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
        let width = 1999;
        let height = 1000;
        let mut b = Board::new(width, height);

        for point in (0..width).flat_map(|x| iter::repeat(x).take(height)).zip((0..height).cycle()) {
            let x = point.0;
            let y = point.1;
            b.set(x, y, true);
        }

        assert!(b.neighbors(1,1).all(|x| x == true));
        assert!(b.neighbors(0,0).all(|x| x == true));
        assert!(b.neighbors(0,9).all(|x| x == true));
        assert!(b.neighbors(9,0).all(|x| x == true));
        assert!(b.neighbors(9,9).all(|x| x == true));
        assert!(b.neighbors(0,3).all(|x| x == true));
        assert!(b.neighbors(3,0).all(|x| x == true));
        assert!(b.neighbors(8,9).all(|x| x == true));
        assert!(b.neighbors(9,8).all(|x| x == true));
    }

    #[test]
    fn test_neighbors_iterator_mixed() {
        let width = 1999;
        let height = 1000;
        let mut b = Board::new(width, height);

        for point in (0..width).flat_map(|x| iter::repeat(x).take(height)).zip((0..height).cycle()) {
            let x = point.0;
            let y = point.1;
            b.set(x, y, (x + y) % 2 == 0);
        }

        assert_eq!(b.neighbors(1,1).fold((0,0),|acc, x| if x {(acc.0 + 1, acc.1)} else {(acc.0, acc.1 + 1)} ), (4,4));
    }
}


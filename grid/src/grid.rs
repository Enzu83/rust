use std::error::Error;

pub struct Grid<T> {
    data: Vec<T>,
    size: (usize, usize),
}

impl<T> Grid<T>
where 
    T: Default + Clone
{
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { 
            data: vec![T::default(); rows * cols], 
            size: (rows, cols),
        }
    }

    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.data.resize_with(rows * cols, T::default);
        self.size = (rows, cols);
    }

    pub fn rows(&self) -> usize {
        self.size.0
    }

    pub fn cols(&self) -> usize {
        self.size.1
    }

    fn in_bounds(&self, i: usize, j: usize) -> bool {
        i < self.rows() && j < self.cols()
    }

    fn to_pos(&self, i: usize, j: usize) -> usize {
        i * self.cols() + j
    }

    pub fn set(&mut self, i: usize, j: usize, val: T) -> Result<(), Box<dyn Error>> {
        if !self.in_bounds(i, j) {
            return Err(format!("Out of bounds index {:?}, should be less than {:?}", (i, j), self.size).into());
        }

        let pos = self.to_pos(i, j);
        self.data[pos] = val;

        return Ok(());
    }

    pub fn get(&self, i: usize, j: usize) -> Option<&T> {
        let pos = self.to_pos(i, j);
        self.data.get(pos)
    }

    pub fn fill(&mut self, val: T) {
        self.data.fill(val);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_value() {
        let grid: Grid<i32> = Grid::new(1, 1);
        assert_eq!(grid.get(0, 0), Some(&0));

        let grid: Grid<String> = Grid::new(1, 1);
        assert_eq!(grid.get(0, 0), Some(&String::default()));
    }

    #[test]
    fn resize() {
        let mut grid: Grid<i32> = Grid::new(3, 3);
        grid.resize(5, 6);

        assert_eq!(grid.rows(), 5);
        assert_eq!(grid.cols(), 6);
    }

    #[test]
    fn set_and_get_value() {
        //in bounds
        let mut grid: Grid<i32> = Grid::new(3, 3);
        grid.set(2, 1, 7).unwrap();
        assert_eq!(grid.get(2, 1), Some(&7));

        // out of bounds
        grid.set(6, 2, 1).expect_err("");
        grid.set(2, 6, 1).expect_err("");
    }
}

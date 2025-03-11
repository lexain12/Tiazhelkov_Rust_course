#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self::from_slice(&vec![T::default(); rows * cols], rows, cols)
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: Vec::from(grid),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.grid[row * self.cols + col]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        self.grid[row * self.cols + col] = value
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let up_left = || (row - 1, col - 1);
        let up = || (row - 1, col);
        let up_right = || (row - 1, col + 1);
        let right = || (row, col + 1);
        let down_right = || (row + 1, col + 1);
        let down = || (row + 1, col);
        let down_left = || (row + 1, col - 1);
        let left = || (row, col - 1);

        match (row, col) {
            (0, 0) if self.size() == (1, 1) => vec![],
            (0, 0) => vec![right(), down(), down_right()],
            (y, 0) if y == self.rows - 1 => vec![up(), up_right(), right()],
            (0, x) if x == self.cols - 1 => vec![left(), down_left(), down()],
            (y, x) if (y, x) == (self.rows - 1, self.cols - 1) => vec![up_left(), up(), left()],
            (0, _) => vec![left(), right(), down_left(), down(), down_right()],
            (_, 0) => vec![up(), up_right(), right(), down(), down_right()],
            (y, _) if y == self.rows - 1 => vec![up_left(), up(), up_right(), left(), right()],
            (_, x) if x == self.cols - 1 => vec![up_left(), up(), left(), down_left(), down()],
            (_, _) => vec![
                up_left(),
                up(),
                up_right(),
                left(),
                right(),
                down_left(),
                down(),
                down_right(),
            ],
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Debug)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        Self { grid }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    pub fn step(&mut self) {
        let mut new_grid = vec![];
        let (rows, cols) = self.grid.size();
        for row in 0..rows {
            for col in 0..cols {
                dbg!((row, col));
                dbg!(self.grid.neighbours(row, col));
                match self
                    .grid
                    .neighbours(row, col)
                    .iter()
                    .filter(|neighbour| self.grid.get(neighbour.0, neighbour.1) == &Cell::Alive)
                    .count()
                {
                    0..=1 => new_grid.push(Cell::Dead),
                    2 => new_grid.push(self.grid.get(row, col).to_owned()),
                    3 => new_grid.push(Cell::Alive),
                    4..=8 => new_grid.push(Cell::Dead),
                    _ => panic!("Impossible number of neighbours"),
                }
            }
        }
        dbg!(&new_grid);
        self.grid = Grid::from_slice(&new_grid, rows, cols);
    }
}

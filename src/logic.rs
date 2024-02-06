#[derive(Clone, PartialEq, Debug)]
pub struct Cell {
    pub alive: bool,
    live_neighb: u8,
}
#[derive(Clone, PartialEq, Debug)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn update(&mut self) -> &Self {
        for i in 0..self.height {
            for j in 0..self.width {
                match self.grid[i][j].live_neighb {
                    0 | 1 => self.grid[i][j].alive = false,
                    2 => {}
                    3 => self.grid[i][j].alive = true,
                    _ => self.grid[i][j].alive = false,
                }
            }
        }
        self.update_neighbours();
        self
    }

    pub fn update_neighbours(&mut self) -> &Self {
        for i in 0..self.height {
            for j in 0..self.width {
                let mut cant = 0;
                for (x, y) in self.neighbours(i as u32, j as u32) {
                    if self.grid[x][y].alive {
                        cant += 1;
                    }
                }
                self.grid[i][j].live_neighb = cant;
            }
        }
        self
    }

    fn new_cell(&mut self, i: usize, j: usize) -> &Self {
        if !self.grid[i][j].alive {
            self.grid[i][j].alive = true;
            for (x, y) in self.neighbours(i as u32, j as u32) {
                self.grid[x][y].live_neighb += 1;
            }
        }
        self
    }

    fn kill_cell(&mut self, i: usize, j: usize) -> &Self {
        if self.grid[i][j].alive {
            self.grid[i][j].alive = false;
            for (x, y) in self.neighbours(i as u32, j as u32) {
                self.grid[x][y].live_neighb -= 1;
            }
        }
        self
    }

    pub fn change_cell(&mut self, i: usize, j: usize) -> &Self {
        if self.grid[i][j].alive {
            self.kill_cell(i, j);
            return self;
        }
        self.new_cell(i, j);
        self
    }

    fn neighbours(&self, i: u32, j: u32) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        for k in 0..=2_u32 {
            for l in 0..=2_u32 {
                let x: i32 = ((i + k) - 1).try_into().unwrap();
                let y: i32 = ((j + l) - 1).try_into().unwrap();
                if x >= 0
                    && x < self.height.try_into().unwrap()
                    && y < self.width.try_into().unwrap()
                    && y >= 0
                    && (k != 1 || l != 1)
                {
                    res.push(((i + k - 1) as usize, (j + l - 1) as usize));
                }
            }
        }
        res
    }
    pub fn show(&self) {
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                if self.grid[i][j].alive {
                    print!("*");
                } else {
                    print!("_");
                }
            }
            println!(" ");
        }
    }

    pub fn new(width: usize, height: usize) -> Self {
        let deadcell = Cell {
            alive: false,
            live_neighb: 0,
        };
        Grid {
            width,
            height,
            grid: vec![vec![deadcell; width.into()]; height.into()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn test_neigh() {
        let g = Grid::new(5, 5);
        assert_eq!(g.neighbours(4, 4), vec![(3, 3), (3, 4), (4, 3)]);
        println!("{:?}", g.neighbours(4, 4));
    }

    #[test]
    fn test_worm() {
        let mut g = Grid::new(5, 5);
        g.grid[2][2].alive = true;
        g.grid[2][1].alive = true;
        g.grid[2][3].alive = true;
        g.update_neighbours();
        /*g
        _____
        _____
        _***_
        _____
        _____
         */
        let mut g2 = Grid::new(5, 5);
        g2.grid[1][2].alive = true;
        g2.grid[2][2].alive = true;
        g2.grid[3][2].alive = true;
        g2.update_neighbours();
        /*
        _____
        __*__
        __*__
        __*__
        _____
         */
        g.update();
        assert_eq!(g2, g);
    }
}

#[warn(dead_code)]
#[warn(unused_variables)]
#[derive(Clone)]
pub struct Cell {
    alive: bool,
    live_neighb: usize,
}

pub struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>,
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
                for (x, y) in Self::neighbours(i, j, self.width, self.height) {
                    if self.grid[x][y].alive {
                        cant += 1;
                    }
                }
                self.grid[i][j].live_neighb = cant;
            }
        }
        self
    }

    fn neighbours(i: usize, j: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        for k in 0..=2 {
            for l in 0..=2 {
                let x: i32 = (i + k) as i32 - 1;
                let y: i32 = (j + l) as i32 - 1;
                if x >= 0 && x < height as i32 && y < width as i32 && y >= 0 && (k != 1 || l != 1) {
                    res.push((i + k - 1, j + l - 1));
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
            grid: vec![vec![deadcell; width]; height],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn test_show() {}

    #[test]
    fn test_neigh() {
        println!("{:?}", Grid::neighbours(4, 4, 5, 5));
    }

    use std::{thread, time::Duration};
    #[test]
    fn test_always() {
        let mut g = Grid::new(5, 5);
        g.grid[2][2].alive = true;
        g.grid[2][2].live_neighb = 2;
        g.grid[2][1].alive = true;
        g.grid[2][1].live_neighb = 1;
        g.grid[2][3].alive = true;
        g.grid[2][3].live_neighb = 1;
        g.update_neighbours().show();

        loop {
            g.update().show();
            println!("##################");
            thread::sleep(Duration::from_secs(1));
        }
    }
}

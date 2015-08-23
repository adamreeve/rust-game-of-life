use std::vec::Vec;

#[derive(Clone,Copy)]
pub enum CellState {
    Alive,
    Dead
}

struct GameState {
    width: usize,
    height: usize,
    state: Vec<CellState>
}

impl GameState {
    pub fn new(width: usize, height: usize) -> GameState {
        return GameState {
            width: width,
            height: height,
            state: vec![CellState::Alive; width * height]
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> CellState {
        self.state[y * self.width + x]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, state: CellState) {
        self.state[y * self.width + x] = state;
    }

    pub fn get_neighbours(&self, x: usize, y: usize) -> Vec<CellState> {
        let mut neighbours = Vec::new();
        for xdif in -1..1 {
            for ydif in -1..1 {
                if !(x == 0 && y == 0) {
                    match self.get_neighbour(x, y, xdif, ydif) {
                        Some(neighbour) => neighbours.push(neighbour),
                        None => ()
                    }
                }
            }
        }
        neighbours
    }

    fn get_neighbour(&self, x: usize, y: usize, xdif: isize, ydif: isize) -> Option<CellState> {
        let xpos = (x as isize) + xdif;
        let ypos = (y as isize) + ydif;
        return if xpos >= 0 && ypos >= 0 {
            let xposu = xpos as usize;
            let yposu = ypos as usize;
            if xposu < self.width && yposu < self.height {
                Some(self.get_cell(xposu, yposu))
            } else {
                None
            }
        } else {
            None
        }
    }
}

use std::vec::Vec;
use std::collections::HashSet;
use std::collections::hash_set;

#[derive(Clone,Copy,PartialEq,Eq,Hash)]
pub struct CellPosition {
    x: i32,
    y: i32
}

#[derive(PartialEq,Eq,Clone,Copy)]
pub enum CellState {
    Alive,
    Dead
}

pub struct GameState {
    _alive_cells: HashSet<CellPosition>
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            _alive_cells: HashSet::new()
        }
    }

    pub fn get_cell(&self, cell: CellPosition) -> CellState {
        match self._alive_cells.contains(&cell) {
            true => CellState::Alive,
            false => CellState::Dead
        }
    }

    pub fn set_cell(&mut self, cell: CellPosition, state: CellState) {
        match state {
            CellState::Alive => { self._alive_cells.insert(cell); },
            CellState::Dead => { self._alive_cells.remove(&cell); }
        }
    }

    pub fn alive_cells(&self) -> hash_set::Iter<CellPosition> {
        self._alive_cells.iter()
    }

    pub fn neighbours(&self, cell: CellPosition) -> Vec<CellPosition> {
        let mut neighbours = Vec::with_capacity(8);
        for xdif in -1..1 {
            for ydif in -1..1 {
                if !(xdif == 0 && ydif == 0) {
                    let neighbour = CellPosition {
                        x: cell.x + xdif,
                        y: cell.y + ydif
                    };
                    neighbours.push(neighbour);
                }
            }
        }
        neighbours
    }

    pub fn alive_neighbours(&self, cell: CellPosition) -> usize {
        return self.neighbours(cell).iter()
            .map(|&neighbour| self.get_cell(neighbour))
            .filter(|&state| state == CellState::Alive)
            .count()
    }
}

use std::vec::Vec;
use std::collections::HashSet;
use std::collections::hash_set;

#[derive(Clone,Copy,PartialEq,Eq,Hash)]
pub struct CellPosition {
    pub x: i32,
    pub y: i32
}

#[derive(PartialEq,Eq,Clone,Copy)]
pub enum CellState {
    Alive,
    Dead
}

pub struct WorldState {
    _alive_cells: HashSet<CellPosition>
}

impl WorldState {
    pub fn new() -> WorldState {
        WorldState {
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
        for xdif in -1..2 {
            for ydif in -1..2 {
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

// Rules:
// 1. Any live cell with fewer than two live neighbours dies, as if caused by under-population.
// 2. Any live cell with two or three live neighbours lives on to the next generation.
// 3. Any live cell with more than three live neighbours dies, as if by overcrowding.
// 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
pub fn tick(world: WorldState) -> WorldState {
    let mut cells_to_visit = HashSet::new();

    let alive_cells = world.alive_cells();
    for &cell in alive_cells {
        cells_to_visit.insert(cell);
        for neighbour in world.neighbours(cell) {
            cells_to_visit.insert(neighbour);
        }
    }

    let mut next_world_state = WorldState::new();
    for cell in cells_to_visit {
        let cell_state = world.get_cell(cell);
        let alive_neighbours = world.alive_neighbours(cell);
        let new_cell_state = match cell_state {
            CellState::Alive if alive_neighbours < 2 => CellState::Dead,
            CellState::Alive if alive_neighbours > 3 => CellState::Dead,
            CellState::Alive => CellState::Alive,
            CellState::Dead if alive_neighbours == 3 => CellState::Alive,
            CellState::Dead => CellState::Dead
        };
        next_world_state.set_cell(cell, new_cell_state);
    }
    next_world_state
}

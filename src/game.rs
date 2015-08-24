// Rules:
// 1. Any live cell with fewer than two live neighbours dies, as if caused by under-population.
// 2. Any live cell with two or three live neighbours lives on to the next generation.
// 3. Any live cell with more than three live neighbours dies, as if by overcrowding.
// 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

use std::collections::HashSet;
use state::{GameState, CellState};

pub fn tick(state: GameState) -> GameState {
    let mut cells_to_visit = HashSet::new();

    let alive_cells = state.alive_cells();
    for cell in alive_cells {
        cells_to_visit.insert(*cell);
        for neighbour in state.neighbours(*cell) {
            cells_to_visit.insert(neighbour);
        }
    }

    let mut new_state = GameState::new();
    for cell in cells_to_visit {
        let cell_state = state.get_cell(cell);
        let alive_neighbours = state.alive_neighbours(cell);
        let new_cell_state = match cell_state {
            CellState::Alive if alive_neighbours < 2 => CellState::Dead,
            CellState::Alive if alive_neighbours > 3 => CellState::Dead,
            CellState::Alive => CellState::Alive,
            CellState::Dead if alive_neighbours == 3 => CellState::Alive,
            CellState::Dead => CellState::Dead
        };
        new_state.set_cell(cell, new_cell_state);
    }
    new_state
}

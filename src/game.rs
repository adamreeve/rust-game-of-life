extern crate rustty;

use self::rustty::{Terminal,Event};
use std::thread::sleep_ms;

use life;

const ALIVE: char = '\u{25A0}';

macro_rules! tryterm {
    ($expr:expr) => (match $expr {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(err) => {
            return Err(format!("Terminal error: {}", err))
        }
    })
}

struct GameState {
    cursor_x: i32,
    cursor_y: i32,
    running: bool
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            cursor_x: 0,
            cursor_y: 0,
            running: false
        }
    }
}

pub fn run(mut world_state: life::WorldState) -> Result<(), String> {
    let mut term = Terminal::new().unwrap();
    let mut game_state = GameState::new();

    tryterm!(term.clear());
    tryterm!(term.swap_buffers());
    'run: for iter in 1.. {
        tryterm!(term.clear());
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match ch {
                'q' => break 'run,
                'p' => { game_state.running = !game_state.running; }
                _   => {},
            }
        }

        if game_state.running {
            world_state = life::tick(world_state);
        }

        display_game(&mut term, &world_state);

        let iter_label = format!("Iteration: {}", iter);
        write_text(&mut term, &iter_label, 0, -3);
        let state_label = if game_state.running { "Running (P to pause)".to_string() } else { "Paused (P to run)".to_string() };
        write_text(&mut term, &state_label, 0, -2);
        let quit_label = "Q to quit";
        write_text(&mut term, &quit_label, 0, -1);

        tryterm!(term.swap_buffers());
        sleep_ms(50);
    }
    Ok(())
}

fn display_game(term: &mut Terminal, world_state: &life::WorldState) {
    let (cols, rows) = term.size();
    for cell in world_state.alive_cells() {
        if cell.x > 0 && cell.y > 0 {
            let xpos = cell.x as usize;
            let ypos = cell.y as usize;
            if xpos < cols && ypos < rows {
                match term.get_mut(xpos, ypos) {
                    Some(term_cell) => {
                        term_cell.set_ch(ALIVE);
                    }
                    None => {}
                }
            }
        }
    }
}

fn write_text(term: &mut Terminal, text: &str, x: i32, y: i32) {
    let (_cols, _rows) = term.size();
    let cols = _cols as i32;
    let rows = _rows as i32;
    let ypos = if y < 0 { rows + y } else { y };
    // Just return without error if we can't fit the text
    if ypos >= rows || ypos < 0 {
        return;
    }
    for (idx, character) in text.chars().enumerate() {
        let xpos = x + (idx as i32);
        if xpos >= cols || xpos < 0 {
            return;
        }
        match term.get_mut(xpos as usize, ypos as usize) {
            Some(cell) => {
                cell.set_ch(character);
            }
            None => {}
        }
    }
}

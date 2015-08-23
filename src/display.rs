extern crate rustty;

use self::rustty::{Terminal,Event};
use std::thread::sleep_ms;

const FILLED: char = '\u{25A0}';

macro_rules! tryterm {
    ($expr:expr) => (match $expr {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(err) => {
            return Err(format!("Terminal error: {}", err))
        }
    })
}

pub fn display(width: usize, height: usize) -> Result<(), String> {
    let mut term = Terminal::new().unwrap();

    tryterm!(term.clear());
    tryterm!(term.swap_buffers());
    'main: for iter in 1.. {
        tryterm!(term.clear());
        let (cols, rows) = term.size();
        if cols < width {
            return Err("Width too large for terminal size".to_string());
        }
        if rows < height + 1 {
            return Err("Height too large for terminal size".to_string());
        }

        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match ch {
                'q' => break 'main,
                _   => {},
            }
        }

        match term.get_mut(10, iter) {
            Some(cell) => {
                cell.set_ch(FILLED);
            }
            None => {
                return Err("Invalid cell location".to_string());
            }
        }

        let label = format!("Iteration: {}", iter);
        try!(write_text(&mut term, &label, 0, 0));
        tryterm!(term.swap_buffers());
        sleep_ms(50);
    }
    Ok(())
}

fn write_text(term: &mut Terminal, text: &str, x: usize, y: usize) -> Result<(), String> {
    for (idx, character) in text.chars().enumerate() {
        match term.get_mut(x + idx, y) {
            Some(cell) => {
                cell.set_ch(character);
            }
            None => {
                return Err(format!("Invalid cell location: {}, {}", x + idx, y));
            }
        }
    }
    Ok(())
}

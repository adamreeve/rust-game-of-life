extern crate rustty;

use self::rustty::Terminal;

pub fn display(width: u32, height: u32) {
    let cols;
    let rows;
    {
        let term = Terminal::new().unwrap();
        let (_cols, _rows) = term.size();
        cols = _cols;
        rows = _rows;
        // Need term to go out of scope to be able to use println
    }
    println!("Game size: {} x {}", width, height);
    println!("Term size: {} x {}", cols, rows);
}

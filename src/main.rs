#[macro_use]
extern crate clap;

use clap::App;
use std::process;
use std::io::Write;

mod display;

fn run() -> Result<(), &'static str> {
    let args = App::new("Game of Life")
        .version(&crate_version!()[..])
        .about("Plays Conway's Game of Life")
        .args_from_usage(
            "-w --width=[WIDTH] 'Sets the width of the game, default=40'
             -h --height=[HEIGHT] 'Sets the height of the game, default=40'")
        .get_matches();

    let width: u32 = match args.value_of("WIDTH").unwrap_or("40").parse() {
        Ok(val) => val,
        Err(_) => {
            return Err("Invalid width specified");
        }
    };
    let height: u32 = match args.value_of("HEIGHT").unwrap_or("40").parse() {
        Ok(val) => val,
        Err(_) => {
            return Err("Invalid height specified");
        }
    };

    display::display(width, height);
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => {
            writeln!(&mut ::std::io::stderr(), "{}", err).unwrap();
            process::exit(-1);
        }
    }
}

#![feature(custom_derive)]

#[macro_use]
extern crate clap;

use clap::App;
use std::process;
use std::io::Write;

mod game;
mod life;

fn run() -> Result<(), String> {
    App::new("Game of Life")
        .version(&crate_version!()[..])
        .about("Plays Conway's Game of Life")
        .get_matches();

    let world = life::WorldState::new();
    game::run(world)
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

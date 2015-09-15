#![feature(custom_derive)]

#[macro_use]
extern crate clap;
extern crate combine;

use clap::App;
use std::process;
use std::io::Write;

mod game;
mod life;
mod import;

fn run() -> Result<(), String> {
    let args = App::new("Game of Life")
        .version(&crate_version!()[..])
        .about("Plays Conway's Game of Life")
        .args_from_usage(
            "-i --input=[INPUT_FILE] 'A starting state in Life 1.06 format'")
        .get_matches();

    let world = match args.value_of("INPUT_FILE") {
        Some(path) => match import::load_file(path) {
            Ok(world) => world,
            Err(err) => return Err(err)
        },
        None => life::WorldState::new(),
    };
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

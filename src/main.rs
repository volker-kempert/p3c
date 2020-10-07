#[macro_use]
extern crate clap;
// use std::*;
use clap::{App, Arg, SubCommand};
use p3d::cube;
use p3d::piece;


use p3d::ge_cube;

// cSpell: disable

fn main() {
    let matches = App::new("p3d")
        .version("1.0")
        .author("Volker.kempert@gmail.com>")
        .about("Cumpute 3d packing of cubes (5x5x5")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("lsbox")
                .about("print an empty box")
                .version("1.0"),
        )
        .subcommand(
            SubCommand::with_name("ge-cube")
                .about("Genetic evolution: Solve the cube packing problem")
                .version("1.0")
                .arg(
                    Arg::with_name("generations")
                        .short("g")
                        .long("generations")
                        .help("Number of generations maximal to run: default 1000")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("population")
                        .short("p")
                        .long("population")
                        .help("Number of individums of the population: default 1000")
                        .takes_value(true)
                ),
        )
        .subcommand(
            App::new("lspiece")
                .about("List pieces")
                .arg(
                    Arg::with_name("PIECE-ID")
                        .help("The Id in range 0..24")
                        .required(true),
                )
                .arg(
                    Arg::with_name("index")
                    .help("The index to determine x, y, z and rotion")
                    .takes_value(true),
                )
                .version("1.0"),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => {}
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("lsbox") {
        // print empty box
        let mybox = cube::PrintBox::new();
        println!("Empty box... {} ", mybox);

        // matches of subcommand arguments follow here
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            // println!("Printing normally...");
        }
    }
    if let Some(matches) = matches.subcommand_matches("lspiece") {
        // print a box with piece
        let id = value_t!(matches, "PIECE-ID", usize).unwrap();
        let mut mybox = cube::PrintBox::new();
        let mut mypiece = piece::Piece::new(id);
        if matches.is_present("index") {
            let index = value_t!(matches, "index", u16).unwrap();
            println!("Apply index: {}", index);
            mypiece.set_combination(Some(index));
            mypiece.set_piece();
        }
        if mypiece.fit_in_box() {
            mypiece.add_to_box(&mut mybox);
            println!("Box... {} ", mybox);
        } else {
            println!("Piece does not fit into box")
        }
    }
    if let Some(matches) = matches.subcommand_matches("ge-cube") {
        let mut generations: u64 = 1000;
        if matches.is_present("generations") {
            generations = value_t!(matches, "generations", u64).unwrap();
        }
        let mut population: usize = 1000;
        if matches.is_present("population") {
            population = value_t!(matches, "population", usize).unwrap();
        }
        ge_cube::solve_cube(generations, population);
    }
}

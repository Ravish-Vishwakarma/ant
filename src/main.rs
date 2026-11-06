use std::env;
mod commands;
mod help;
mod parse_args;
mod services;
mod types;
use commands::{add, initialize, log};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        help::show_help();
    } else {
        handel_commands(&args);
    }
}

fn handel_commands(args: &Vec<String>) {
    let argument_type = parse_args::parse_args(args);
    match argument_type {
        Some(command) => match command {
            types::CommandType::Init => {
                initialize::initilize_ant(args);
            }
            types::CommandType::Add => {
                add::ant_add(&args);
            }
            types::CommandType::Commit => {}
            types::CommandType::Status => {}
            types::CommandType::Log => {
                log::log(&args);
            }
            types::CommandType::Diff => {}
            types::CommandType::Branch => {}
            types::CommandType::Checkout => {}
        },
        None => {
            println!("{} is not a valid command, See 'ant --help'", args[1]);
        }
    }
}

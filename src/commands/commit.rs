use crate::services;
pub fn ant_commit(args: &Vec<String>) {
    println!("Inside Commit {:?}", args);
    if services::check_ant_exists() {
        if args.len() >= 4 {
            if args[2] == "-m" || args[2] == "--message" {
                println!("Okay right flag")
            } else {
                println!("{} is not a valid flag.", args[2]);
            }
        } else {
            println!("Please enter a message with -m or --message.");
        }
    } else {
        println!("Repo isn't initlized.");
    }
}

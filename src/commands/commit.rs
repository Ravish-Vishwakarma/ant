use crate::services;
use crate::types::CommitInfo;
pub fn ant_commit(args: &Vec<String>) {
    // println!("Inside Commit {:?}", args);
    if let Some(commit) = parse_commit_command(&args) {
        println!("{}", commit.message);
        println!("{}", commit.author);
    }
}

fn parse_commit_command(args: &Vec<String>) -> Option<CommitInfo> {
    if services::check_ant_exists() {
        if args.len() >= 4 {
            if args[2] == "-m" || args[2] == "--message" {
                // println!("Okay right flag");
                if args.len() >= 6 {
                    if args[4] == "--author" {
                        return Some(CommitInfo {
                            message: args[3].clone(),
                            author: args[5].clone(),
                        });
                    }
                }

                return Some(CommitInfo {
                    message: args[3].clone(),
                    author: String::from("ant default user"),
                });
            } else {
                println!("{} is not a valid flag.", args[2]);
                None
            }
        } else {
            println!("Please enter a message with -m or --message.");
            None
        }
    } else {
        println!("Repo isn't initlized.");
        None
    }
}

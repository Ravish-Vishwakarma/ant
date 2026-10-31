use crate::types::CommandType;
pub fn parse_args(arguments: &Vec<String>) -> Option<CommandType> {
    match arguments[1].as_str() {
        "init" => Some(CommandType::Init),
        "add" => Some(CommandType::Add),
        "commit" => Some(CommandType::Commit),
        "status" => Some(CommandType::Status),
        "log" => Some(CommandType::Log),
        "diff" => Some(CommandType::Diff),
        "branch" => Some(CommandType::Branch),
        "checkout" => Some(CommandType::Checkout),
        _ => None,
    }
}

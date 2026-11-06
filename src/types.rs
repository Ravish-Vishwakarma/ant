pub enum CommandType {
    Init,
    Add,
    Commit,
    Status,
    Log,
    Diff,
    Branch,
    Checkout,
}

pub struct RepoFileType {
    pub path: String,
    pub content: String,
}

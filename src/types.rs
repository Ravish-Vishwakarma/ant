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

// ---------------------------- GITOBJ ---------------------------- //
struct GitObject {
    obj_type: GitObjType,
    // content: &[u8],
}
enum GitObjType {
    Blob,
    Commit,
    Tree,
}

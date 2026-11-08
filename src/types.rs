use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};
use std::io::Write;

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
pub struct GitObject {
    object_type: GitObjType,
    content: Vec<u8>,
}

#[derive(Debug)]
pub enum GitObjType {
    Blob,
    Commit,
    Tree,
}

impl GitObject {
    pub fn hash(&self) -> String {
        let mut hasher = Sha1::new();

        let header = format!("{:?} {}\0", self.object_type, self.content.len());

        hasher.update(header.as_bytes());
        hasher.update(&self.content);
        let result = hasher.finalize();

        hex::encode(result)
    }

    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let header = format!("{:?} {}\0", self.object_type, self.content.len());
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

        encoder.write_all(header.as_bytes())?;
        encoder.write_all(&self.content)?;

        let compressed_bytes = encoder.finish()?;

        Ok(compressed_bytes)
    }
}

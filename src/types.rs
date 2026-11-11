use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};
use std::io::Read;
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
    pub object_type: GitObjType,
    pub content: Vec<u8>,
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

    pub fn deserialize(data: Vec<u8>) -> GitObject {
        let mut decoder = ZlibDecoder::new(&data[..]);
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .expect("Failed to decompress zlib data");

        let null_idx = decompressed
            .iter()
            .position(|&b| b == 0)
            .expect("Invalid git object: missing null byte");

        let header = &decompressed[..null_idx];
        let content = decompressed[null_idx + 1..].to_vec();

        let mut header_parts = header.splitn(2, |&b| b == b' ');
        let obj_type_bytes = header_parts.next().expect("Missing object type");
        let obj_type_str =
            String::from_utf8(obj_type_bytes.to_vec()).expect("Invalid UTF-8 in object type");

        let object_type = match obj_type_str.as_str() {
            "Blob" => GitObjType::Blob,
            "Commit" => GitObjType::Commit,
            "Tree" => GitObjType::Tree,
            other => panic!("Unknown git object type: {}", other),
        };

        GitObject {
            object_type,
            content,
        }
    }
}

pub struct Blob {
    pub content: Vec<u8>,
}
impl Blob {
    pub fn get_content(&self) -> &Vec<u8> {
        return &self.content;
    }
}

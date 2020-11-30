#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FileBlob {
  pub contents: Vec<u8>,
}

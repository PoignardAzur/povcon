use crate::commit::Commit;
use crate::file_blob::FileBlob;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct GlobalStore {
  pub commits: HashMap<u32, Commit>,
  pub blobs: HashMap<u32, FileBlob>,
}

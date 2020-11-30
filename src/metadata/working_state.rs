use crate::file_blob::FileBlob;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Path(pub String);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Timestamp(pub i32);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FileData {
  pub time_stamp: Timestamp,
  pub file_blob: FileBlob,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct WorkingState {
  pub files: HashMap<Path, FileData>,
}

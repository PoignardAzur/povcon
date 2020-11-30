#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Commit {
  pub title: String,
  pub descr: String,
  pub parent_id: u32,
  // TODO - HashMap<Path, Blob>
}

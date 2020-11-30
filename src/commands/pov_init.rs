pub fn pov_init(use_json: bool) -> std::io::Result<()> {
  use crate::global_metadata::GlobalMetadata;
  use crate::global_store::GlobalStore;
  use crate::working_state::WorkingState;

  use std::collections::HashMap;
  use rmp_serde::Serializer;
  use serde::Serialize;

  let metadata = GlobalMetadata {
    store: GlobalStore {
      commits: HashMap::new(),
      blobs: HashMap::new(),
    },
    working_state: WorkingState {
      files: HashMap::new(),
    },
  };
  let mut buf = Vec::new();

  metadata.serialize(&mut Serializer::new(&mut buf)).unwrap();

  use std::fs::{File, create_dir_all};
  use std::io::prelude::*;

  create_dir_all(".povcon/")?;

  let mut file = File::create(".povcon/povcon_metadata")?;
  file.write_all(&buf)?;

  if use_json {
    // TODO
    serde_json::to_writer(File::create(".povcon/povcon_metadata.json")?, &metadata).unwrap();
  }

  Ok(())
}

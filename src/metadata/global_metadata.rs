use crate::working_state::WorkingState;
use crate::global_store::GlobalStore;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct GlobalMetadata {
  pub store: GlobalStore,
  pub working_state: WorkingState,
}

use crate::core::{Function, TypeId};

pub struct Types {
  pub construct_directory_digest: Function,
  pub construct_snapshot: Function,
  pub construct_file_content: Function,
  pub construct_files_content: Function,
  pub construct_process_result: Function,
  pub address: TypeId,
  pub path_globs: TypeId,
  pub directory_digest: TypeId,
  pub snapshot: TypeId,
  pub merged_directories: TypeId,
  pub files_content: TypeId,
  pub dir: TypeId,
  pub file: TypeId,
  pub link: TypeId,
  pub process_request: TypeId,
  pub process_result: TypeId,
  pub generator: TypeId,
  pub url_to_fetch: TypeId,
  pub string: TypeId,
  pub bytes: TypeId,
}

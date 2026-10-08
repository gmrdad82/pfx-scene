#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod diagnostic;
mod edit;
mod id;
mod migrate;
mod paths;
mod project;
mod read;
mod rules;
mod scene;
mod sha256;
mod spans;
pub mod types;

pub use diagnostic::{Diagnostic, Location, Severity, code};
pub use edit::{EditError, Kind, Patch, PatchGroup, SceneEdit, Target, Value};
pub use id::{ID_BITS, ID_LENGTH, Id, IdError};
pub use project::{MAX_DEPTH, Project, check, migrate};
pub use scene::{Entry, Scene};
pub use types::{
    FORMAT, File, FileKind, MaterialsFile, PrefabFile, ProjectFile, ProjectTable, ProxiesFile,
    SceneFile,
};

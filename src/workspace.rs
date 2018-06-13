//! The `workspace` module provides the public interface into the CTF management library.

use std::error::Error;
use model::*;
use repository::Repository;

const DEFAULT_WORKSPACE: &str = ".config/.ctf";

/// Encapsulates configuration parameters for a workspace instance.
pub struct Config<T: Repository> {
    /// The location to use for persisting the workspace data.
    ///
    /// The meaning of the string depends on the backing repository.
    /// If the string points to an existing repository, the state will be loaded into the workspace,
    /// otherwise a new repository will be created.
    location: String,
    /// The repository strategy responsible for providing actual data persistence. Any type that
    /// implements the `Repository` trait can be used.
    repository: T,
}


/// Keeps track of all the data and statistics related to a given workspace.
///
/// Workspaces are independent of each other and can be managed concurrently.
pub struct Workspace {
}

impl Workspace {
    /// Creates a new workspace with the specified configuration.
    ///
    /// # Panics
    pub fn new<T: Repository>(cfg: Config<T>) -> Result<Workspace, Box<Error>> {
        Ok(Workspace {})
    }
}

// #[cfg(test)]
// mod tests {
//     use sqlite::Sqlite;
//     #[test]
//     fn create_workspace_null_config() {
//         let cfg = Config {
//             location: ".ctfbot.sqlite",
//             repository: Sqlite
//         };
//     }
// }

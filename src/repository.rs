use std::error::Error;
use model::*;

// pub trait Crud {
//     type Resource;

//     fn add(r: Resource) -> Result<Resource, Error>;
//     fn patch(
// }

/// The `Repository` trait provides the common functionality required to persist the data in a
/// workspace.  Repositories have an empty constructor and will be initialized by the library when
/// the time comes. If the requested location does not exist, the repository is responsible for
/// creating it, otherwise it should load the existing data at the location. If the data is not
/// valid, the repository must raise an error.
pub trait Repository {
    /// Empty constructor that returns an uninitialized repository.
    fn new() -> Self;

    /// Called by the library when the repository must be initialized at `location`.  If the
    /// initialization succeeds, the call returns whether a new workspace was created.
    ///
    /// # Failures
    ///
    /// Will return `Err` when the workspace could not be initialized.
    fn init(&mut self, location: &str) -> Result<bool, Box<Error>>;

    // Users
    // fn user_add(u: User) -> Result<u32, Box<Error>>;
    // fn user_patch(u: User) -> Result<bool, Box<Error>>;
    // fn user_find(id: u32) -> Option<User>; // TODO: Ref
    // fn users() -> Vec<User>;
}

#[cfg(test)]
mod tests {
}

pub mod path_behavior;
pub mod dotfile;
pub mod mapping;
pub mod command;

pub use dotfile::Dotfile;

pub use path_behavior::AppendBehavior;
pub use path_behavior::ReplaceBehavior;
pub use path_behavior::PathBehavior;

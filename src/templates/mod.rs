pub mod developers;
pub mod graphql_playground;
pub mod jumbotron;
pub mod login;
pub mod recovery;
pub mod registration;
pub mod navbar;
pub mod page;
pub mod profile;

/// Re-export everything in the static_pages module publicly.
pub mod static_pages;
pub use static_pages::*;

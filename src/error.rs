//! Error handling.

/// All major errors that can be shown to a user.
#[derive(Clone, Debug)]
pub enum TelescopeError {
    /// 404 - Page not found. Use [`TelescopeError::ResourceNotFound`] instead
    /// when possible, as it will have more info.
    PageNotFound,

    /// 404 - Resource Not Found.
    ResourceNotFound {
        /// The header of the jumbotron to be displayed.
        header: String,
        /// The message to display under the jumbotron.
        message: String
    },
}



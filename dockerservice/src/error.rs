use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Everything that can go wrong talking to a container engine.
///
/// The variants describe situations a user interface has to react to
/// differently, not the transport that produced them: nothing here mentions
/// the backend, so a second adapter maps onto the same set.
#[derive(Debug, Error)]
pub enum Error {
    /// The daemon could not be reached at all — not running, wrong socket,
    /// or the socket is not readable by this user.
    #[error("cannot reach the container daemon: {message}")]
    DaemonUnreachable { message: String },

    /// The daemon was reached and refused: the socket or the operation is
    /// not permitted for this user.
    #[error("permission denied by the container daemon: {message}")]
    PermissionDenied { message: String },

    /// The requested resource does not exist.
    #[error("not found: {message}")]
    NotFound { message: String },

    /// The resource exists but is in a state the operation does not allow,
    /// such as starting a container that is already running.
    #[error("conflicting state: {message}")]
    Conflict { message: String },

    /// The request did not complete in time.
    #[error("the request to the container daemon timed out")]
    Timeout,

    /// The daemon answered, but not with something we could understand.
    #[error("unexpected response from the container daemon: {message}")]
    Protocol { message: String },

    /// The daemon reported an error that maps to none of the above.
    #[error("the container daemon returned status {status}: {message}")]
    Api { status: u16, message: String },
}

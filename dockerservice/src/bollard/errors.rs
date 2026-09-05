use crate::Error;
use bollard::errors::Error as BollardError;
use std::io::ErrorKind;

impl From<BollardError> for Error {
    fn from(err: BollardError) -> Self {
        match err {
            // The daemon answered; the status code says how the UI should react.
            BollardError::DockerResponseServerError {
                status_code,
                message,
            } => match status_code {
                401 | 403 => Error::PermissionDenied { message },
                404 => Error::NotFound { message },
                409 => Error::Conflict { message },
                status => Error::Api { status, message },
            },

            // No answer at all — usually no daemon, or a socket we may not open.
            BollardError::IOError { err } => match err.kind() {
                ErrorKind::PermissionDenied => Error::PermissionDenied {
                    message: err.to_string(),
                },
                _ => Error::DaemonUnreachable {
                    message: err.to_string(),
                },
            },

            BollardError::RequestTimeoutError => Error::Timeout,

            // Answered, but unintelligible.
            e @ (BollardError::JsonDataError { .. }
            | BollardError::JsonSerdeError { .. }
            | BollardError::StrParseError { .. }
            | BollardError::APIVersionParseError { .. }
            | BollardError::DockerStreamError { .. }) => Error::Protocol {
                message: e.to_string(),
            },

            other => Error::DaemonUnreachable {
                message: other.to_string(),
            },
        }
    }
}

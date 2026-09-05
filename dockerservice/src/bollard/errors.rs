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

#[cfg(test)]
mod tests {
    use super::*;

    fn server(status_code: u16) -> Error {
        BollardError::DockerResponseServerError {
            status_code,
            message: "boom".to_owned(),
        }
        .into()
    }

    fn io(kind: ErrorKind) -> Error {
        BollardError::IOError {
            err: std::io::Error::new(kind, "boom"),
        }
        .into()
    }

    #[test]
    fn unauthorized_and_forbidden_are_permission_denied() {
        assert!(matches!(server(401), Error::PermissionDenied { .. }));
        assert!(matches!(server(403), Error::PermissionDenied { .. }));
    }

    #[test]
    fn missing_resource_is_not_found() {
        assert!(matches!(server(404), Error::NotFound { .. }));
    }

    #[test]
    fn conflicting_state_is_conflict() {
        assert!(matches!(server(409), Error::Conflict { .. }));
    }

    #[test]
    fn other_statuses_keep_the_code() {
        assert!(matches!(server(500), Error::Api { status: 500, .. }));
        assert!(matches!(server(418), Error::Api { status: 418, .. }));
    }

    #[test]
    fn the_daemons_message_survives() {
        let Error::NotFound { message } = server(404) else {
            panic!("expected NotFound");
        };
        assert_eq!(message, "boom");
    }

    #[test]
    fn an_unreadable_socket_is_permission_denied_not_a_missing_daemon() {
        // The difference between "Docker is not running" and "you are not in
        // the docker group" is the whole reason this mapping exists.
        assert!(matches!(
            io(ErrorKind::PermissionDenied),
            Error::PermissionDenied { .. }
        ));
    }

    #[test]
    fn other_io_failures_mean_the_daemon_is_unreachable() {
        for kind in [ErrorKind::NotFound, ErrorKind::ConnectionRefused] {
            assert!(matches!(io(kind), Error::DaemonUnreachable { .. }));
        }
    }

    #[test]
    fn timeouts_are_their_own_case() {
        let err: Error = BollardError::RequestTimeoutError.into();
        assert!(matches!(err, Error::Timeout));
    }

    #[test]
    fn unintelligible_answers_are_protocol_errors() {
        let stream: Error = BollardError::DockerStreamError {
            error: "boom".to_owned(),
        }
        .into();
        assert!(matches!(stream, Error::Protocol { .. }));

        let version: Error = BollardError::APIVersionParseError {}.into();
        assert!(matches!(version, Error::Protocol { .. }));
    }

    #[test]
    fn unknown_failures_do_not_masquerade_as_success() {
        let err: Error = BollardError::MissingSessionBuildkitError {}.into();
        assert!(matches!(err, Error::DaemonUnreachable { .. }));
    }
}

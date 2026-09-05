/// Daemon-level queries that belong to no single resource.
pub trait SystemService {
    fn version(&self) -> String;
}

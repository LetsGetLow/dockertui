/// Daemon-level queries that belong to no single resource.
pub trait SystemService: Send + Sync {
    fn version(&self) -> String;
}

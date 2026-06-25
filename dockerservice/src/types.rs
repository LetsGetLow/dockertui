use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type Lables = HashMap<String, String>;

pub type MountPointType = String;

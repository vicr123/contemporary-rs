use tracing::Level;

#[derive(Debug)]
pub struct ApplicationLogEntry {
    pub level: Level,
    pub target: String,
    pub message: String,
}

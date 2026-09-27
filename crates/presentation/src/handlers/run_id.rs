use std::time::{SystemTime, UNIX_EPOCH};

pub(super) fn new_run_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    format!("run-{}-{timestamp}", std::process::id())
}

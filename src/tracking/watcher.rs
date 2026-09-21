// File system watcher for build artifacts

use crate::utils::logger::log_to_file;
use notify::{RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher};
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct BuildWatcher {
    watcher: Arc<Mutex<RecommendedWatcher>>,
}

impl BuildWatcher {
    pub fn new(debug_logs_enabled: bool) -> Self {
        let watcher = RecommendedWatcher::new(
            move |res: Result<notify::Event, notify::Error>| match res {
                Ok(event) => {
                    if debug_logs_enabled
                        && matches!(
                            event.kind,
                            notify::EventKind::Create(_)
                                | notify::EventKind::Modify(_)
                                | notify::EventKind::Remove(_)
                        )
                    {
                        log_to_file(&format!("Build change detected: {:?}", event));
                    }
                }
                Err(e) => {
                    if debug_logs_enabled {
                        log_to_file(&format!("Watch error: {:?}", e));
                    }
                }
            },
            notify::Config::default(),
        )
        .unwrap();
        BuildWatcher {
            watcher: Arc::new(Mutex::new(watcher)),
        }
    }

    pub fn watch<P: AsRef<Path>>(&mut self, path: P) -> NotifyResult<()> {
        self.watcher
            .lock()
            .unwrap()
            .watch(path.as_ref(), RecursiveMode::Recursive)?;
        Ok(())
    }
}

use std::path::PathBuf;

use plugins_core::{InstalledApp as InstalledAppTrait, Version};

pub struct InstalledApp {
    archive_path: PathBuf,
    //archive_metadata: Archive::Metadata,
}

impl InstalledAppTrait for InstalledApp {
    fn id(&self) -> &str {
        ""
    }
    fn name(&self) -> &str {
        ""
    }
    fn description(&self) -> &str {
        ""
    }
    fn version(&self) -> Version {
        Version::parse("0.1.0").unwrap()
    }

    fn environment(&self) -> &str {
        ""
    }

    fn install_location(&self) -> &std::path::Path {
        &self.archive_path
    }
}

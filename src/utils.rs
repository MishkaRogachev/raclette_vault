
pub fn app_data_path() -> std::io::Result<std::path::PathBuf> {
    if cfg!(test) {
        Ok(std::env::temp_dir())
    } else if cfg!(debug_assertions) {
        Ok(std::env::current_dir()?)
    } else {
        if let Some(proj_dirs) = directories::ProjectDirs::from(
            "com", "raclettevault", "RacletteVault") {
            Ok(proj_dirs.data_dir().to_path_buf())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Unable to determine a proper data directory",
            ))
        }
    }
}

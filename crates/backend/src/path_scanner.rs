use {
    crate::error::Error,
    std::{
        env,
        fs,
        path::Path,
        io,
    },
};

const DATA_DIRS: [&str; 4] = ["", ".config/", ".local/share/", ".local/state/"];

const XDG_PATHS: [&str; 12] = [
    ".cache",
    ".config",
    ".local",
    ".local/share",
    ".local/state",
    ".config/mimeapps.list",
    ".config/user-dirs.dirs",
    ".config/user-dirs.locale",
    ".local/share/applications",
    ".local/share/icons",
    ".local/share/Trash",
    ".local/share/recently-used.xbel",
];

pub fn remove_empty_dirs() -> Result<(), Error> {
    for dir in &get_data_paths()? {
        if is_empty(dir)? {
            println!("{} is empty", dir.file_name().unwrap().to_str().unwrap());
            trash::delete(dir)?;
        }
    }

    Ok(())
}

pub fn get_data_paths() -> Result<Box<[Box<Path>]>, Error> {
    let home = env::var("HOME")?;

    let mut paths = Vec::new();

    for path in DATA_DIRS {
        for entry in fs::read_dir(format!("{}/{}", home, path))? {
            let entry = entry?;

            if let Some(file_name) = entry.file_name().to_str() {
                // idk dude
                if path != "" || file_name.starts_with('.') {
                    if XDG_PATHS.contains(&format!("{}{}", path, file_name).as_str()) {
                        continue;
                    }

                    paths.push(entry.path().into_boxed_path());
                }
            }
        }
    }

    Ok(paths.into_boxed_slice())
}

fn is_empty(path: &Path) -> Result<bool, Error> {
    if path.is_file() {
        return Ok(path.metadata()?.len() == 0);
    }

    if path.is_dir() {
        match fs::read_dir(path) {
            Ok(path) => {
                for entry in path {
                    let entry = entry?;

                    if !is_empty(&entry.path())? {
                        return Ok(false)
                    }
                }
            },
            Err(error) => {
                return match error.kind() {
                    io::ErrorKind::PermissionDenied => Ok(false),
                    _ => Err(Error::Io(error)),
                }
            },
        }
    }

    Ok(true)
}

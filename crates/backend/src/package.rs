use {
    crate::error::Error, freedesktop_file_parser::DesktopEntry, std::{
        ffi::OsStr, fs, path::Path
    }
};

#[derive(Debug, PartialEq)]
pub enum InstallReason {
    Explicit,
    Dependency,
}

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub install_reason: InstallReason,
    pub dependency: bool,
    pub optional_dependency: bool,
    pub provides: Box<[Box<str>]>,
    pub files: Box<[Box<Path>]>, // maybe i just shouldnt store this here idk
                                // should probably store as actual paths
                                // idk what the comment above is supposed to mean
    pub data_paths: Vec<Box<Path>>,
    pub visible_desktop_entries: Vec<DesktopEntry>,
    // should find the "main" desktop entry
    pub icon_paths: Vec<Box<Path>>,
    pub best_icon_path: Option<Box<Path>>,
}

impl Package {
    pub fn new(
        name: &str,
        install_reason: InstallReason,
        dependency: bool,
        optional_dependency: bool,
        provides: Box<[Box<str>]>,
        files: Box<[Box<Path>]>,
    ) -> Self {
        Self {
            name: name.to_string(),
            install_reason,
            dependency,
            optional_dependency,
            provides,
            files,
            data_paths: Vec::new(),
            visible_desktop_entries: Vec::new(),
            icon_paths: Vec::new(),
            best_icon_path: None,
        }
    }

    pub fn find_desktop_entries(&mut self) -> Result<(), Error> {
        for file in self.files.iter() {
            if file.extension() == Some(OsStr::new("desktop")) && file.is_file() && file.starts_with("/usr/share/applications") {
                //println!("/{}", file);

                if let Ok(desktop_file) = freedesktop_file_parser::parse(
                    fs::read_to_string(file)?.as_str()
                ) {
                    //println!("/{} isn't fucked", file);
                    let desktop_entry = desktop_file.entry;

                    if desktop_entry.no_display.is_none() || desktop_entry.no_display == Some(false) {
                        self.visible_desktop_entries.push(desktop_entry);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn find_icon_paths(&mut self) -> Result<(), Error> {
        for icon in self.visible_desktop_entries.iter()
            .filter_map(|desktop_entry| desktop_entry.icon.as_ref().map(|icon| &icon.content))
        {
            for file in self.files.iter() {
                //println!("does {:?} start with {}?", file.file_name().unwrap(), icon);
                if file.starts_with("/usr/share/icons") &&
                    file.file_name().unwrap().to_str().unwrap().starts_with(icon) {  // two unwraps in one line nice
                    const IMAGE_EXTENSIONS: [&str; 2] = [
                        "png",
                        "svg",
                    ];

                    if let Some(file_extension) = file.extension() {
                        if IMAGE_EXTENSIONS.into_iter().any(|image_extension| file_extension == image_extension) {
                            self.icon_paths.push(file.clone());
                            //println!("{:?} is possibly an icon", file);
                        }
                    }
                }
            }
        }

        let sizes = [
            "scalable",
            "512x512",
            "256x256",
            "192x192",
            "128x128",
            "96x96",
            "72x72",
            "64x64",
            "48x48",
            "36x36",
            "32x32",
            "24x24",
            "22x22",
            "20x20",
            "16x16",
        ];

        if let Some(desktop_entry) = self.visible_desktop_entries.get(0) {  // just get the first one lollllll
            if let Some(icon_name) = &desktop_entry.icon {
                for icon_path in self.icon_paths.iter().map(|path| path.to_str().unwrap()) {
                    for size in sizes {
                                                                // i think it already does this
                        if icon_path.contains(size) && icon_path.contains(&icon_name.content) {
                                                                    // idk why this works
                            self.best_icon_path = Some(Path::new(icon_path).into());
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

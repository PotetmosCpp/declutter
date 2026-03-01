use {
    std::{
        path::Path,
        fs,
        ffi::OsStr,
    },
    freedesktop_file_parser::DesktopEntry,
    crate::error::Error,
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
    pub data_paths: Vec<Box<Path>>,
    pub visible_desktop_entries: Vec<DesktopEntry>,
    pub icon_paths: Vec<Box<Path>>,
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
        }
    }

    pub fn find_desktop_entries(&mut self) -> Result<(), Error> {
        for file in self.files.iter() {
            if file.extension() == Some(OsStr::new("desktop")) && file.is_file() {
                //println!("/{}", file);

                if let Ok(desktop_file) = freedesktop_file_parser::parse(
                    fs::read_to_string(file)?.as_str()
                ) {
                    //println!("/{} isn't fucked", file);
                    let desktop_entry = desktop_file.entry;

                    if desktop_entry.no_display.is_none() || desktop_entry.no_display == Some(false) {
                        self.visible_desktop_entries.push(desktop_entry);
                    }
                } else {
                    //println!("/{} is fucked", file);
                }
            } else {
                //println!("{:?}", file);
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
                if file.file_name().unwrap().to_str().unwrap().starts_with(icon) {  // two unwraps in one line nice
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
                } else {
                    //println!("nooooooooo");
                }
            }
        }
        

        Ok(())
    }
}

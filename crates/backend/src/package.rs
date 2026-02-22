use {
    std::{
        path::Path,
        fs,
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
    pub files: Box<[Box<str>]>, // maybe i just shouldnt store this here idk
    pub data_paths: Vec<Box<Path>>,
    pub desktop_entries: Vec<DesktopEntry>,
}

impl Package {
    pub fn new(
        name: &str,
        install_reason: InstallReason,
        dependency: bool,
        optional_dependency: bool,
        provides: Box<[Box<str>]>,
        files: Box<[Box<str>]>,
    ) -> Self {
        Self {
            name: name.to_string(),
            install_reason,
            dependency,
            optional_dependency,
            provides,
            files,
            data_paths: Vec::new(),
            desktop_entries: Vec::new(),
        }
    }

    pub fn find_desktop_entries(&mut self) -> Result<(), Error> {
        for file in self.files.iter() {
            if file.ends_with(".desktop") {
                //println!("/{}", file);

                if let Ok(desktop_file) = freedesktop_file_parser::parse(
                    fs::read_to_string(Path::new(&format!("/{}", file)))?.as_str()
                ) {
                    //println!("/{} isn't fucked", file);
                    self.desktop_entries.push(desktop_file.entry);
                } else {
                    //println!("/{} is fucked", file);
                }
            }
        }

        Ok(())
    }
}

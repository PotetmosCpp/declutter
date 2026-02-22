use {
    std::{
        ops::{
            Deref,
            DerefMut,
        },
        path::Path,
        iter,
    },
    alpm::{
        Alpm,
        PackageReason,
    },
    crate::{
        error::Error,
        package::{
            Package,
            InstallReason,
        },
        path_scanner,
    },
};

pub struct Packages(Box<[Package]>);

impl Packages {
    pub fn get() -> Result<Self, Error> {
        let alpm = Alpm::new("/", "/var/lib/pacman")?;
        let packages = alpm.localdb().pkgs();

        Ok(Packages(
            packages.iter()
                .map(|package| {
                    Package::new(
                        package.name(),
                        match package.reason() {
                            PackageReason::Explicit => InstallReason::Explicit,
                            PackageReason::Depend => InstallReason::Dependency,
                        },
                        !package.required_by().is_empty(),
                        !package.optional_for().is_empty(),
                        package.provides().iter()
                            .map(|provides| Box::from(provides.name()))
                            .collect(),
                        package.files().files().iter()
                            .map(|file| Box::from(str::from_utf8(file.name()).expect("this shouldnt happen")))
                            .collect(),
                    )
                })
                .collect()
        ))
    }

    pub fn explicit(&self) -> Box<[&Package]> {
        self.iter()
            .filter(|package| package.install_reason == InstallReason::Explicit)
            .collect()
    }

    pub fn unneeded(&self) -> Box<[&Package]> {
        self.iter()
            .filter(|package|
                package.install_reason == InstallReason::Dependency &&
                !package.dependency &&
                !package.optional_dependency
            )
            .collect()
    }

    pub fn optional(&self) -> Box<[&Package]> {
        self.iter()
            .filter(|package|
                package.install_reason == InstallReason::Dependency &&
                !package.dependency &&
                package.optional_dependency
            )
            .collect()
    }

    pub fn find_data_paths(&mut self) -> Result<(), Error> {
        // should use a hashset here
        let data_dirs = path_scanner::get_data_paths()?;

        for package in self.iter_mut() {
            for data_dir in data_dirs.iter() {
                if iter::once(package.name.as_str())
                    .chain(package.provides.iter().map(|provides| provides.as_ref()))
                    .any(|string| string == data_dir.file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_lowercase()
                    ) {
                    package.data_paths.push(data_dir.clone());
                }
            }
        }

        Ok(())
    }

    pub fn get_unused_data_dirs(&self) -> Result<Box<[Box<Path>]>, Error> {
        // should use hashset here too
        let package_data_paths: Box<[Box<Path>]> = self.iter()
            .flat_map(|package| package.data_paths.clone()) // dont know if this clone is needed
            .collect();

        Ok(path_scanner::get_data_paths()?
            .into_iter()
            .filter(|data_path| !package_data_paths.contains(data_path))
            .collect())
    }

    pub fn find_desktop_entries(&mut self) -> Result<(), Error> {
        for package in self.iter_mut() {
            package.find_desktop_entries()?;
        }

        Ok(())
    }
}

impl Deref for Packages {
    type Target = Box<[Package]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Packages {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

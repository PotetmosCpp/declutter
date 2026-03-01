use backend::packages::Packages;

fn main() {
    let mut packages = Packages::get().unwrap();

    packages.find_data_paths().unwrap();
    packages.find_desktop_entries().unwrap();

    packages.find_icon_paths().unwrap();

    let package = &packages.explicit()[1];
    println!("{:#?}", package);

    //println!("Explicit: {:#?}", packages.explicit());
    println!("Explicit:");
    packages.explicit().iter()
        .for_each(|package| println!("{}", package.name));
    println!("Unneeded: {:#?}", packages.unneeded());
    println!("Optional: {:#?}", packages.optional());

    backend::remove_empty_dirs().unwrap();

    println!("Unused data dirs: {:#?}", packages.get_unused_data_dirs().unwrap());
}

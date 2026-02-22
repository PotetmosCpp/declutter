mod app;

//use backend::packages::Packages;

use {
    cosmic::app::settings::Settings,
    backend::packages::Packages,
};

use app::App;

fn main() {
    let mut packages = Packages::get().unwrap();

    packages.find_data_paths().unwrap();
    packages.find_desktop_entries().unwrap();

    //println!("Explicit: {:#?}", packages.explicit());
    println!("Explicit:");
    packages.explicit().iter()
        .for_each(|package| println!("{}", package.name));
    println!("Unneeded: {:#?}", packages.unneeded());
    println!("Optional: {:#?}", packages.optional());

    backend::remove_empty_dirs().unwrap();

    println!("Unused data dirs: {:#?}", packages.get_unused_data_dirs().unwrap());

    /*let settings = Settings::default();
    cosmic::app::run::<App>(settings, ()).unwrap();*/
}

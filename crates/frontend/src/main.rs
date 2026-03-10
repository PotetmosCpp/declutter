use eframe::{App, NativeOptions, egui::{self, CentralPanel, Image, ScrollArea, Vec2, include_image}};

use backend::packages::Packages;

fn main() -> eframe::Result {
    let mut packages = Packages::get().unwrap();

    packages.find_data_paths().unwrap();
    packages.find_desktop_entries().unwrap();

    packages.find_icon_paths().unwrap();

    //println!("Explicit: {:#?}", packages.explicit());
    println!("Explicit:");
    packages.explicit().iter()
        .for_each(|package| println!("{}", package.name));
    println!("Unneeded: {:#?}", packages.unneeded());
    println!("Optional: {:#?}", packages.optional());

    backend::remove_empty_dirs().unwrap();

    println!("Unused data dirs: {:#?}", packages.get_unused_data_dirs().unwrap());

    let options = NativeOptions::default();

    let mut explicit: Vec<Package> = packages.explicit().iter()
        .map(|package| Package::new(
                &package.name,
                package.icon_paths.get(0).map(|icon_path| icon_path.to_str().unwrap()),
        ))
        .collect();
        // should be sorted by if they have an icon or no

    explicit.sort_by_key(|x| x.icon_path.clone().is_none());


    eframe::run_native(
        "declutter",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(Declutter {
                explicit
            }))
        }),
    )
}

struct Package {
    name: Box<str>,
    icon_path: Option<Box<str>>,
}

impl Package {
    fn new(name: &str, icon_path: Option<&str>) -> Self {
        Self {
            name: name.into(),
            icon_path: icon_path.map(Box::from),
        }
    }
}

struct Declutter {
    explicit: Vec<Package>,
}

impl App for Declutter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for package in &self.explicit {
                        ui.allocate_ui(Vec2::new(100.0, 150.0), |ui| {
                            ui.vertical_centered(|ui| {
                                if let Some(icon_path) = &package.icon_path {
                                    ui.add(Image::new(
                                        // test icon
                                        format!("file://{}", icon_path),
                                    ));
                                } else {
                                    ui.add(Image::new(
                                        // test icon
                                        include_image!("/usr/share/icons/Cosmic/scalable/apps/application-default.svg"),
                                    ));
                                }

                                ui.heading(&package.name);
                            });
                        });
                    }
                });
            });
        });
    }
}

use std::path::{Path, PathBuf};

use eframe::{App, NativeOptions, egui::{self, CentralPanel, Image, RichText, ScrollArea, Vec2, ViewportBuilder, include_image}};

use backend::packages::Packages;

fn main() -> eframe::Result {
    // this code isnt supposed to be good btw i just wanted something that works

    let mut packages = Packages::get().unwrap();

    packages.find_data_paths().unwrap();
    packages.find_desktop_entries().unwrap();

    packages.find_icon_paths().unwrap();

    //println!("Explicit: {:#?}", packages.explicit());
    //println!("Explicit:");
    //packages.explicit().iter()
        //.for_each(|package| println!("{}", package.name));
    //println!("Unneeded: {:#?}", packages.unneeded());
    //println!("Optional: {:#?}", packages.optional());

    // should be a button for this
    backend::remove_empty_dirs().unwrap();

    //println!("Unused data dirs: {:#?}", packages.get_unused_data_dirs().unwrap());

    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_app_id("Declutter"),
        ..Default::default()
    };

    let mut explicit: Vec<Package> = packages.explicit().iter()
        .map(|package| Package::new(
            &package.name,
            package.best_icon_path.as_deref().and_then(Path::to_str),
        ))
        .collect();
    explicit.sort_by_key(|x| x.icon_path.clone().is_none());

    // lollllllllllllllllllz repeoteitontionperjepreiroetieropnrne
    let mut unneeded: Vec<Package> = packages.unneeded().iter()
        .map(|package| Package::new(
            &package.name,
            package.best_icon_path.as_deref().and_then(Path::to_str),
        ))
        .collect();
    unneeded.sort_by_key(|x| x.icon_path.clone().is_none());

    let mut optional: Vec<Package> = packages.optional().iter()
        .map(|package| Package::new(
            &package.name,
            package.best_icon_path.as_deref().and_then(Path::to_str),
        ))
        .collect();
    optional.sort_by_key(|x| x.icon_path.clone().is_none());

    let untracked_dirs = packages.get_unused_data_dirs().unwrap().iter().map(|dir| dir.to_path_buf()).collect();


    eframe::run_native(
        "Declutter",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(Declutter {
                explicit,
                unneeded,
                optional,
                untracked_dirs,
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
    unneeded: Vec<Package>,
    optional: Vec<Package>,
    untracked_dirs: Vec<PathBuf>,
}

impl App for Declutter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.label(RichText::new("Explicit").size(50.0));
                ui.horizontal_wrapped(|ui| {
                    for package in &self.explicit {
                        ui.allocate_ui(Vec2::new(100.0, 150.0), |ui| {
                            ui.vertical_centered(|ui| {
                                if let Some(icon_path) = &package.icon_path {
                                    ui.add(Image::new(
                                        format!("file://{}", icon_path),
                                    ));
                                } else {
                                    ui.add(Image::new(
                                        // this shouldnt be done
                                        include_image!("/usr/share/icons/Cosmic/scalable/apps/application-default.svg"),
                                    ));
                                }

                                ui.heading(&package.name);
                            });
                        });
                    }
                });

                // here too loll
                ui.label(RichText::new("Uneeded").size(50.0));
                ui.horizontal_wrapped(|ui| {
                    for package in &self.unneeded {
                        ui.allocate_ui(Vec2::new(100.0, 150.0), |ui| {
                            ui.vertical_centered(|ui| {
                                if let Some(icon_path) = &package.icon_path {
                                    ui.add(Image::new(
                                        format!("file://{}", icon_path),
                                    ));
                                } else {
                                    ui.add(Image::new(
                                        include_image!("/usr/share/icons/Cosmic/scalable/apps/application-default.svg"),
                                    ));
                                }

                                ui.heading(&package.name);
                            });
                        });
                    }
                });

                ui.label(RichText::new("Optional").size(50.0));
                ui.horizontal_wrapped(|ui| {
                    for package in &self.optional {
                        ui.allocate_ui(Vec2::new(100.0, 150.0), |ui| {
                            ui.vertical_centered(|ui| {
                                if let Some(icon_path) = &package.icon_path {
                                    ui.add(Image::new(
                                        format!("file://{}", icon_path),
                                    ));
                                } else {
                                    ui.add(Image::new(
                                        include_image!("/usr/share/icons/Cosmic/scalable/apps/application-default.svg"),
                                    ));
                                }

                                ui.heading(&package.name);
                            });
                        });
                    }
                });

                ui.label(RichText::new("Untracked folders").size(50.0));
                ui.horizontal_wrapped(|ui| {
                    for untracked_dir in &self.untracked_dirs {
                        ui.allocate_ui(Vec2::new(100.0, 150.0), |ui| {
                            ui.vertical_centered(|ui| {
                                ui.image(include_image!("/usr/share/icons/Cosmic/scalable/mimetypes/inode-directory.svg"));

                                ui.heading(&untracked_dir.to_str().unwrap()[10..]);
                            });
                        });
                    }
                });
            });
        });
    }
}

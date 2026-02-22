// todo maybe make gui for wparu
// braised beef is brrrrrrrrrrrrrrrrrilllliiiiant
// brilliant idea btw i use arch btw use the table on the arch wiki for the
// things the xdg support article where it has comments and notes for stuff
// that doesnt xdg

use {
    cosmic::{
        prelude::*,
        Core,
        Application,
        widget,
        iced,
        app::Task,
    },
    backend::packages::Packages,
};

pub struct App {
    core: Core,
    count: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    Clicked,
}

impl Application for App {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "app id probably";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _: Self::Flags) -> (Self, Task<Self::Message>) {
        let app = App {
            core,
            count: 2.0,
        };

        (app, Task::none())
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let mut row = widget::row();

        let packages = Packages::get().unwrap();

        for package in packages.iter() {
            let mut icon = None;

            for desktop_entry in package.desktop_entries.iter() {
                if let Some(desktop_icon) = &desktop_entry.icon {
                    icon = Some(desktop_icon);
                }
            }

            if let Some(icon) = icon {
                let button = widget::button::standard(package.name.clone())
                    .height(100)
                    .font_size(50)
                    .on_press(Message::Clicked);

                row = row.push(button);
            }
        }

        widget::container(widget::scrollable(row))
            .center(iced::Length::Fill)
            .into()
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::Clicked => self.count *= 2.0,
        }

        Task::none()
    }
}

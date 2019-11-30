use vgtk::{ext::*, gtk, run, Component, UpdateAction, VNode};
use vgtk::lib::{gtk::*, gio::ApplicationFlags};

pub mod entry;
pub mod saved;
mod checklist;

use entry::Entry;
use saved::SavedEntries;

#[derive(Default)]
pub struct App {
    entries: SavedEntries,
}

#[derive(Clone, Debug)]
pub enum AppMessage {
    Exit,
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_props: Self::Properties) -> Self {
        let entries = SavedEntries::new()
            .with_entry(Entry::new("FooBar".to_string(), "Bar".to_string(), true))
            .with_entry(Entry::new("FooBar2".to_string(), "Bar".to_string(), true))
            .with_entry(Entry::new("FooBar3".to_string(), "Bar".to_string(), true))
            .with_entry(Entry::new("FooBar4".to_string(), "Bar".to_string(), true));
        Self {
            entries,
        }
    }

    fn update(&mut self, message: AppMessage) -> UpdateAction<Self> {
        match message {
            AppMessage::Exit => {
                vgtk::quit();
                UpdateAction::None
            }
        }
    }

    fn view(&self) -> VNode<App> {
        gtk! {
            <Application::new_unwrap(Some("com.github.gnome_canteens"), ApplicationFlags::empty())>
                <Window>
                <HeaderBar title="Gnome Canteens" show_close_button=true>
                    <Button HeaderBar::pack_type=PackType::Start image="gtk-go-back"/>
                </HeaderBar>
                <Stack transition_duration=2 transition_type=StackTransitionType::SlideLeft>
                    {
                        self.entries.render()
                    }
                </Stack>
                </Window>
            </Application>
        }
    }
}

fn main() {
    std::process::exit(run::<App>());
}

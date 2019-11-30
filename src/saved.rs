use vgtk::{ext::*, gtk, run, Component, UpdateAction, VNode};
use vgtk::lib::{gtk::*, gio::ApplicationFlags};

use crate::entry::Entry;
use crate::App;

#[derive(Default)]
pub struct SavedEntries {
    entries: Vec<Entry>,
}

impl SavedEntries {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn set_entries(&mut self, entries: Vec<Entry>) {
        self.entries = entries;
    }

    pub fn with_entry(mut self, entry: Entry) -> Self {
        self.entries.push(entry);
        self
    }

    pub fn with_entries(self, entries: Vec<Entry>) -> Self {
        Self {
            entries,
            ..self
        }
    }

    pub fn render(&self) -> VNode<App> {
        gtk! {
            <Viewport>
                <ListBox>
                    {
                        self.entries.clone().into_iter().map(|entry| gtk!{
                            <ListBoxRow>
                                <Box>
                                    { entry.render() }
                                    <Button image="gtk-go-forward" relief=ReliefStyle::None always_show_image=true Box::pack_type=PackType::End />
                                </Box>
                            </ListBoxRow>
                        })
                    }
                </ListBox>
            </Viewport>
        }
    }
}

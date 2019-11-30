use vgtk::{gtk, VNode};
use vgtk::lib::gtk::*;

use crate::App;

#[derive(Default, Clone)]
pub struct Entry {
    name: String,
    location: String,
    open: bool,
}

impl Entry {
    pub fn new(name: String, location: String, open: bool) -> Self {
        Self {
            name,
            location,
            open,
        }
    }

    pub fn open_to_string(&self) -> String {
        match self.open {
            true => "Open".to_string(),
            false => "Closed".to_string(),
        }
    }

    pub fn render(&self) -> VNode<App> {
        gtk! {
            <Box orientation=Orientation::Vertical>
                <Label label=self.name.clone()/>
                <Box orientation=Orientation::Vertical>
                    <Label label=self.location.clone()/>
                    <Label label=self.open_to_string()/>
                </Box>
            </Box>
        }
    }
}

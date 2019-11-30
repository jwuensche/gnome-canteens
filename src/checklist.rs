use vgtk::{gtk, VNode};
use vgtk::lib::gtk::*;

use crate::entry::Entry;
use crate::App;

pub struct CheckList {
    entry: Entry,
}

impl CheckList {
    pub fn new(name: String, location: String, open: bool) -> Self {
        Self {
            entry: Entry::new(name, location, open)
        }
    }

    pub fn render(&self) -> VNode<App> {
        gtk! {
            <Box orientation=Orientation::Horizontal>
            {self.entry.render()}
            <CheckButton/>
                </Box>
        }
    }
}

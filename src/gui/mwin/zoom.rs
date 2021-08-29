use gtk::prelude::{ButtonExt, ContainerExt, EntryExt, ToggleButtonExt, WidgetExt};

use super::operation::{ope_set_zoom_fit, ope_set_zoom_level};

const SCROLL_VIEW_MAX_ZOOM_FACTOR: f32 = 20.0;
const _SCROLL_VIEW_MIN_ZOOM_FACTOR: f32 = 0.02;

const ZOOM_LEVELS: [f32; 9] = [0.125, 0.25, 0.5, 0.666, 1.0, 2.0, 4.0, 8.0, 16.0];

pub(crate) struct MyZoom {
    entry: gtk::Entry,
    popover_menu_box: gtk::Box,
    popover_menu_item_zoom_fit: gtk::CheckButton,
}

impl MyZoom {
    pub fn new(
        entry: gtk::Entry,
        popover_menu_box: gtk::Box,
        popover_menu_item_zoom_fit: gtk::CheckButton,
    ) -> Self {
        let a = Self {
            entry,
            popover_menu_box,
            popover_menu_item_zoom_fit,
        };
        a.setup_menu_items();
        a
    }
    //
    pub fn set_zoom_fit(&self) {
        //self.menu_item_zoom_fit.set_active(true);
    }
    pub fn set_zoom_level(&self, _level: f32) {
        self.popover_menu_item_zoom_fit.set_active(false);
    }
    pub fn update_zoom_entry(&self, level: f32) {
        let s = format_zoom_value(level);
        self.entry.set_text(&s);
    }
    //
    fn setup_menu_items(&self) {
        self.popover_menu_item_zoom_fit.connect_active_notify(
            move |check_button: &gtk::CheckButton| {
                if check_button.is_active() {
                    gui_trace!("popover_menu_item_zoom_fit.connect_active_notify()");
                    ope_set_zoom_fit();
                }
            },
        );
        //
        for level in &ZOOM_LEVELS {
            let level = *level;
            if level > SCROLL_VIEW_MAX_ZOOM_FACTOR {
                break;
            }
            //
            let name = format_zoom_value(level);
            let item = gtk::ModelButton::builder().text(name.as_str()).build();
            item.connect_clicked(move |_| {
                gui_trace!("item.connect_clicked:{}", level);
                ope_set_zoom_level(level);
            });
            item.show();
            self.popover_menu_box.add(&item);
        }
    }
}

fn format_zoom_value(value: f32) -> String {
    let zoom_percent = ((value * 1000.0 + 0.5).floor()) * 0.1;
    format!("{:.1}%", zoom_percent)
}

//
pub fn next_zoom_in_level(current_level: f32) -> f32 {
    for level in &ZOOM_LEVELS {
        let level = *level;
        if current_level < level {
            return level;
        }
    }
    current_level
}
pub fn next_zoom_out_level(current_level: f32) -> f32 {
    for level in ZOOM_LEVELS.iter().rev() {
        let level = *level;
        if current_level > level {
            return level;
        }
    }
    current_level
}

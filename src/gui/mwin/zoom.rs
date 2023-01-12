use glib::ToVariant;
use gtk::prelude::{ActionableExt, ButtonExt, ContainerExt, EntryExt, ToggleButtonExt, WidgetExt};

use super::acti;

const SCROLL_VIEW_MAX_ZOOM_FACTOR: f32 = 20.0;
const _SCROLL_VIEW_MIN_ZOOM_FACTOR: f32 = 0.02;

const ZOOM_LEVELS: [f32; 9] = [0.125, 0.25, 0.5, 0.666, 1.0, 2.0, 4.0, 8.0, 16.0];

pub(crate) struct MyZoom {
    entry: gtk::Entry,
    popover_menu_box: gtk::Box,
    popover_menu_item_zoom_fit: gtk::CheckButton,
    button_zoom_fit: gtk::ToggleButton,
    flg_zoom_fit: bool,
}

impl MyZoom {
    pub fn new(
        entry: gtk::Entry,
        popover_menu_box: gtk::Box,
        popover_menu_item_zoom_fit: gtk::CheckButton,
        button_zoom_fit: gtk::ToggleButton,
    ) -> Self {
        let a = Self {
            entry,
            popover_menu_box,
            popover_menu_item_zoom_fit,
            button_zoom_fit,
            flg_zoom_fit: true,
        };
        a.setup_menu_items();
        a
    }
    //
    pub fn is_zoom_fit(&self) -> bool {
        self.flg_zoom_fit
    }
    //
    pub fn set_zoom_fit(&mut self) {
        self.popover_menu_item_zoom_fit
            .set_action_target_value(Some(&true.to_variant()));
        self.button_zoom_fit
            .set_action_target_value(Some(&true.to_variant()));
        self.popover_menu_item_zoom_fit.set_active(true);
        self.flg_zoom_fit = true;
    }
    pub fn unset_zoom_fit(&mut self) {
        self.popover_menu_item_zoom_fit
            .set_action_target_value(Some(&false.to_variant()));
        self.button_zoom_fit
            .set_action_target_value(Some(&false.to_variant()));
        self.popover_menu_item_zoom_fit.set_active(false);
        self.flg_zoom_fit = false;
    }
    pub fn update_zoom_entry(&self, level: f32) {
        let s = format_zoom_value(level);
        self.entry.set_text(&s);
    }
    //
    fn setup_menu_items(&self) {
        self.popover_menu_item_zoom_fit
            .set_action_target_value(Some(&true.to_variant()));
        self.button_zoom_fit
            .set_action_target_value(Some(&true.to_variant()));
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
                acti::ope_set_zoom_level(level);
            });
            item.show();
            self.popover_menu_box.add(&item);
        }
    }
}

fn format_zoom_value(value: f32) -> String {
    let zoom_percent = ((value * 1000.0 + 0.5).floor()) * 0.1;
    format!("{zoom_percent:.1}%")
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

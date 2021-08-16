use gtk::prelude::{
    ButtonExt, CheckMenuItemExt, ContainerExt, EntryExt, GtkMenuExt, GtkMenuItemExt, MenuShellExt,
    WidgetExt,
};
use gtk::EntryIconPosition;

use std::cell::RefCell;
use std::rc::Rc;

use super::operation::{ope_set_zoom_fit, ope_set_zoom_in, ope_set_zoom_level, ope_set_zoom_out};
use super::MyData;
use super::UI_GLOBAL;

const SCROLL_VIEW_MAX_ZOOM_FACTOR: f32 = 20.0;
const _SCROLL_VIEW_MIN_ZOOM_FACTOR: f32 = 0.02;

const ZOOM_LEVELS: [f32; 9] = [0.125, 0.25, 0.5, 0.666, 1.0, 2.0, 4.0, 8.0, 16.0];

pub(crate) struct MyZoom {
    in_btn: gtk::Button,
    out_btn: gtk::Button,
    entry: gtk::Entry,
    menu: gtk::Menu,
    menu_item_zoom_fit: gtk::CheckMenuItem,
}

impl MyZoom {
    pub fn new(
        in_btn: gtk::Button,
        out_btn: gtk::Button,
        entry: gtk::Entry,
        menu: gtk::Menu,
        menu_item_zoom_fit: gtk::CheckMenuItem,
    ) -> Self {
        let a = Self {
            in_btn,
            out_btn,
            entry,
            menu,
            menu_item_zoom_fit,
        };
        a.setup_menu_items();
        a
    }
    //
    pub fn set_zoom_fit(&self) {
        //self.menu_item_zoom_fit.set_active(true);
    }
    pub fn set_zoom_level(&self, _level: f32) {
        self.menu_item_zoom_fit.set_active(false);
    }
    pub fn update_zoom_entry(&self, level: f32) {
        let s = format_zoom_value(level);
        self.entry.set_text(&s);
    }
    //
    fn setup_menu_items(&self) {
        self.menu_item_zoom_fit.connect_active_notify(
            move |check_menu_item: &gtk::CheckMenuItem| {
                if check_menu_item.is_active() {
                    gui_trace!("menu_item_zoom_fit.connect_active_notify()");
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
            let item = gtk::MenuItem::with_label(name.as_str());
            item.connect_activate(move |_| {
                gui_trace!("item.connect_activate:{}", level);
                ope_set_zoom_level(level);
            });
            item.show();
            self.menu.append(&item);
        }
        self.menu.check_resize();
    }
    //
    pub fn setup_connect(&self) {
        UI_GLOBAL.with(|global| {
            if let Some((ref my_data, _)) = *global.borrow() {
                self.setup_connect_0(my_data);
            }
        });
    }
    fn setup_connect_0(&self, _my_data: &Rc<RefCell<MyData>>) {
        let zoom_menu = self.menu.clone();
        self.entry.connect_icon_press(
            move |_entry, icon_pos: EntryIconPosition, event: &gdk::Event| {
                if icon_pos == EntryIconPosition::Secondary {
                    let btn_num = event.button().unwrap();
                    // only left button
                    if btn_num == 1 {
                        zoom_menu.popup_at_pointer(Some(event));
                    }
                }
            },
        );
        self.in_btn.connect_clicked(move |_btn: &gtk::Button| {
            gui_trace!("in_btn.connect_clicked");
            ope_set_zoom_in();
        });
        self.out_btn.connect_clicked(move |_btn: &gtk::Button| {
            gui_trace!("out_btn.connect_clicked");
            ope_set_zoom_out();
        });
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

use gio::prelude::ActionMapExt;
use gio::{SimpleAction, SimpleActionGroup};
use glib::object::IsA;
use glib::variant::{StaticVariantType, Variant};
use glib::ToVariant;
use gtk::prelude::WidgetExt;
use gtk::Widget;

pub(crate) fn insert_mwin_action_group<T: IsA<Widget>>(wdg: &T) {
    let ag = SimpleActionGroup::new();
    ag.add_action(&create_mwin_action("open", |_ac, _var| {
        gui_trace!("ACTION: mwin.open");
        ope_open_file_chooser_dialog();
    }));
    ag.add_action(&create_mwin_action("reload", |_ac, _var| {
        gui_trace!("ACTION: mwin.reload");
        ope_reload_file();
    }));
    //
    ag.add_action(&create_mwin_action("zoom_in", |_ac, _var| {
        gui_trace!("ACTION: mwin.zoom_in");
        ope_set_zoom_in();
    }));
    ag.add_action(&create_mwin_action("zoom_out", |_ac, _var| {
        gui_trace!("ACTION: mwin.zoom_out");
        ope_set_zoom_out();
    }));
    ag.add_action(&create_mwin_action_state_bool("zoom_fit", |_ac, _var| {
        gui_trace!("ACTION: mwin.zoom_fit: {:?}", _var);
        ope_set_zoom_fit();
    }));
    //
    wdg.insert_action_group("mwin", Some(&ag));
}

fn create_mwin_action<F: Fn(&SimpleAction, Option<&Variant>) + 'static>(
    name: &str,
    f: F,
) -> SimpleAction {
    let ac = SimpleAction::new(name, None);
    let _ = ac.connect_activate(f);
    ac
}

fn create_mwin_action_state_bool<F: Fn(&SimpleAction, Option<&Variant>) + 'static>(
    name: &str,
    f: F,
) -> SimpleAction {
    let ty_ = bool::static_variant_type();
    let va_ = true.to_variant();
    let ac = SimpleAction::new_stateful(name, Some(&ty_), &va_);
    let _ = ac.connect_activate(f);
    ac
}

//----------------------------------------------------------------------
use super::image_area::{open_uri_for_image_file, spawn_render_image};
use super::zoom::{next_zoom_in_level, next_zoom_out_level};
use super::UI_MWIN_GLOBAL;
use crate::gui::fcdia::open_file_chooser;
use crate::gui::guii::Size2Di;

//
pub(crate) fn ope_setup_scroll_window_content_wh(alc_wh: Size2Di) {
    UI_MWIN_GLOBAL.with(|global| {
        if let Some((ref my_data, _)) = *global.borrow() {
            let a_my_data = my_data.borrow();
            a_my_data.im.set_max_content_wh(alc_wh);
        }
    });
}

//
pub(crate) fn ope_set_zoom_level(level: f32) {
    UI_MWIN_GLOBAL.with(|global| {
        if let Some((ref my_data, _)) = *global.borrow() {
            let is_zoom_ok = {
                let mut a_my_data = my_data.borrow_mut();
                let is_zoom_ok = level <= a_my_data.im.available_zoom_level();
                if is_zoom_ok {
                    a_my_data.im.set_zoom_level(level);
                    let _ = glib::idle_add_once(move || {
                        //gui_trace!("glib::idle_add(): ope_zoom_fit_unset_active()");
                        ope_zoom_fit_unset_active();
                    });
                }
                is_zoom_ok
            };
            if is_zoom_ok {
                spawn_render_image(my_data.clone());
            }
        }
    });
}

//
fn ope_set_zoom_fit() {
    UI_MWIN_GLOBAL.with(|global| {
        if let Some((ref my_data, _)) = *global.borrow() {
            {
                let a_my_data = my_data.borrow();
                if a_my_data.zoom.is_zoom_fit() {
                    return;
                }
            }
            {
                let mut a_my_data = my_data.borrow_mut();
                a_my_data.zoom.set_zoom_fit();
                a_my_data.im.set_zoom_fit();
            }
            spawn_render_image(my_data.clone());
        }
    });
}

//
fn ope_set_zoom_in() {
    UI_MWIN_GLOBAL.with(|global| {
        if let Some((ref my_data, _)) = *global.borrow() {
            let is_zoom_ok = {
                let mut a_my_data = my_data.borrow_mut();
                let current_level = if let Some(level) = a_my_data.im.zoom_level() {
                    level
                } else {
                    a_my_data.im.zoom_fit_level()
                };
                let next_level = next_zoom_in_level(current_level);
                let is_zoom_ok = next_level <= a_my_data.im.available_zoom_level();
                if is_zoom_ok {
                    a_my_data.im.set_zoom_level(next_level);
                    let _ = glib::idle_add_once(move || {
                        //gui_trace!("glib::idle_add(): ope_zoom_fit_unset_active()");
                        ope_zoom_fit_unset_active();
                    });
                }
                is_zoom_ok
            };
            if is_zoom_ok {
                spawn_render_image(my_data.clone());
            }
        }
    });
}

//
fn ope_set_zoom_out() {
    UI_MWIN_GLOBAL.with(|global| {
        if let Some((ref my_data, _)) = *global.borrow() {
            let is_zoom_ok = {
                let mut a_my_data = my_data.borrow_mut();
                let current_level = if let Some(level) = a_my_data.im.zoom_level() {
                    level
                } else {
                    a_my_data.im.zoom_fit_level()
                };
                let next_level = next_zoom_out_level(current_level);
                //let is_zoom_ok = next_level <= a_my_data.im.available_zoom_level();
                let is_zoom_ok = true;
                if is_zoom_ok {
                    a_my_data.im.set_zoom_level(next_level);
                    let _ = glib::idle_add_once(move || {
                        //gui_trace!("glib::idle_add(): ope_zoom_fit_unset_active()");
                        ope_zoom_fit_unset_active();
                    });
                }
                is_zoom_ok
            };
            if is_zoom_ok {
                spawn_render_image(my_data.clone());
            }
        }
    });
}

fn ope_zoom_fit_unset_active() {
    UI_MWIN_GLOBAL.with(|global| {
        if let Some((ref my_data, _)) = *global.borrow() {
            {
                let mut a_my_data = my_data.borrow_mut();
                a_my_data.zoom.unset_zoom_fit();
            }
        }
    });
}

//
pub(crate) fn ope_update_zoom_entry() {
    UI_MWIN_GLOBAL.with(|global| {
        if let Some((ref my_data, _)) = *global.borrow() {
            {
                let a_my_data = my_data.borrow();
                let level = a_my_data.im.zoom_fit_level();
                a_my_data.zoom.update_zoom_entry(level);
            }
        }
    });
}

//
pub(crate) fn ope_open_uri_for_image_file(uri_str: &str) {
    if uri_str.is_empty() {
        return;
    }
    open_uri_for_image_file(uri_str);
}

//
fn ope_open_file_chooser_dialog() {
    open_file_chooser();
}
//
fn ope_reload_file() {
    UI_MWIN_GLOBAL.with(|global| {
        if let Some((ref my_data, _)) = *global.borrow() {
            let uri_s = {
                let a_my_data = my_data.borrow();
                a_my_data.uri_s.clone()
            };
            let _ = glib::idle_add_once(move || {
                open_uri_for_image_file(&uri_s);
            });
        }
    });
}

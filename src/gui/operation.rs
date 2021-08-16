use super::guii::Size2Di;
use super::image_area::spawn_render_image;
use super::zoom::{next_zoom_in_level, next_zoom_out_level};
use super::UI_GLOBAL;

//
pub(crate) fn ope_setup_scroll_window_content_wh(alc_wh: Size2Di) {
    UI_GLOBAL.with(|global| {
        if let Some((ref my_data, _)) = *global.borrow() {
            let a_my_data = my_data.borrow();
            a_my_data.im.set_max_content_wh(alc_wh);
        }
    });
}

//
pub(crate) fn ope_set_zoom_fit() {
    UI_GLOBAL.with(|global| {
        if let Some((ref my_data, _)) = *global.borrow() {
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
pub(crate) fn ope_set_zoom_level(level: f32) {
    UI_GLOBAL.with(|global| {
        if let Some((ref my_data, _)) = *global.borrow() {
            let is_zoom_ok = {
                let mut a_my_data = my_data.borrow_mut();
                let is_zoom_ok = level <= a_my_data.im.available_zoom_level();
                if is_zoom_ok {
                    a_my_data.zoom.set_zoom_level(level);
                    a_my_data.im.set_zoom_level(level);
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
pub(crate) fn ope_set_zoom_in() {
    UI_GLOBAL.with(|global| {
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
                    a_my_data.zoom.set_zoom_level(next_level);
                    a_my_data.im.set_zoom_level(next_level);
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
pub(crate) fn ope_set_zoom_out() {
    UI_GLOBAL.with(|global| {
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
                    a_my_data.zoom.set_zoom_level(next_level);
                    a_my_data.im.set_zoom_level(next_level);
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
pub(crate) fn ope_update_zoom_entry() {
    UI_GLOBAL.with(|global| {
        if let Some((ref my_data, _)) = *global.borrow() {
            {
                let a_my_data = my_data.borrow();
                let level = a_my_data.im.zoom_fit_level();
                a_my_data.zoom.update_zoom_entry(level);
            }
        }
    });
}

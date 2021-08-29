use gio::prelude::ActionMapExt;
use gio::{SimpleAction, SimpleActionGroup};
use glib::object::IsA;
use glib::variant::Variant;
use gtk::prelude::WidgetExt;
use gtk::Widget;

use super::operation;

pub(crate) fn insert_mwin_action_group<T: IsA<Widget>>(wdg: &T) {
    let ag = SimpleActionGroup::new();
    ag.add_action(&create_mwin_action("open", |_ac, _var| {
        gui_trace!("ACTION: mwin.open");
        operation::ope_open_file_chooser_dialog();
    }));
    ag.add_action(&create_mwin_action("reload", |_ac, _var| {
        gui_trace!("ACTION: mwin.reload");
    }));
    //
    ag.add_action(&create_mwin_action("zoom_in", |_ac, _var| {
        gui_trace!("ACTION: mwin.zoom_in");
        operation::ope_set_zoom_in();
    }));
    ag.add_action(&create_mwin_action("zoom_out", |_ac, _var| {
        gui_trace!("ACTION: mwin.zoom_out");
        operation::ope_set_zoom_out();
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
    //ac.set_enabled(false);
    ac
}

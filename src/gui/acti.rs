use gio::prelude::ActionMapExt;
use gio::{SimpleAction, SimpleActionGroup};
use glib::object::IsA;
use glib::variant::Variant;
use gtk::prelude::WidgetExt;
use gtk::Widget;

pub(crate) fn insert_app_action_group<T: IsA<Widget>>(wdg: &T) {
    let ag = SimpleActionGroup::new();
    ag.add_action(&create_app_action("help", |_ac, _var| {
        gui_trace!("ACTION: app.help");
        ope_open_help_dialog();
    }));
    ag.add_action(&create_app_action("about", |_ac, _var| {
        gui_trace!("ACTION: app.about");
        ope_open_about_dialog();
    }));
    wdg.insert_action_group("app", Some(&ag));
}

fn create_app_action<F: Fn(&SimpleAction, Option<&Variant>) + 'static>(
    name: &str,
    f: F,
) -> SimpleAction {
    let ac = SimpleAction::new(name, None);
    let _ = ac.connect_activate(f);
    //ac.set_enabled(true);
    ac
}

//----------------------------------------------------------------------
use crate::gui::abdia::open_about;
fn ope_open_about_dialog() {
    open_about();
}

use crate::gui::mwin::open_help;
fn ope_open_help_dialog() {
    open_help();
}

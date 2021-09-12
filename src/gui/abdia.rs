use gtk::prelude::{AboutDialogExt, BuilderExtManual, DialogExt, WidgetExt};

use gtk::Builder as GtkBuilder;

use std::cell::RefCell;
use std::rc::Rc;

const ID_MAIN_ABOUT: &str = "MainAbout";

// gtk & thread_local
// https://gitlab.com/susurrus/gattii/-/blob/master/src/bin/gattii.rs
thread_local!(
    static UI_AB_DLG_GLOBAL: RefCell<Option<(Rc<RefCell<MyAbout>>,i32)>> = RefCell::new(None)
);

pub(crate) fn init(builder: GtkBuilder) {
    let my_about = Rc::new(RefCell::new(MyAbout::new(builder)));
    UI_AB_DLG_GLOBAL.with(move |global| {
        *global.borrow_mut() = Some((my_about, 0));
    });
}

pub(crate) struct MyAbout {
    ab: gtk::AboutDialog,
}
impl MyAbout {
    pub fn new(builder: GtkBuilder) -> Self {
        let ab: gtk::AboutDialog = builder.object(ID_MAIN_ABOUT).unwrap();
        ab.set_version(Some(env!("CARGO_PKG_VERSION")));
        //
        Self { ab }
    }
    pub fn run(&self) {
        let _rt = self.ab.run();
        //
        gui_trace!("MyAbout::run(): {:?}", _rt);
        self.ab.hide();
    }
}

pub(crate) fn open_about() {
    UI_AB_DLG_GLOBAL.with(|global| {
        if let Some((ref my_about, _)) = *global.borrow() {
            {
                let a_my_about = my_about.borrow();
                a_my_about.run();
            }
        }
    });
}

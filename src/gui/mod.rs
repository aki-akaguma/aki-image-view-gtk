//
// https://www.ruimo.com/fromId?fromId=130
//

use gio::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::prelude::{BuilderExt, BuilderExtManual, GtkApplicationExt, GtkWindowExt, WidgetExt};

use gdk_pixbuf::Pixbuf as GdkPixbuf;
use glib::Bytes as GlibBytes;
use gtk::Builder as GtkBuilder;

use std::cell::RefCell;
use std::rc::Rc;

mod image_area;
use image_area::MyImageArea;

use crate::conf::conf_file::ConfigFile;

pub const WINDOW_DEFAULT_WIDTH: i32 = 640;
pub const WINDOW_DEFAULT_HEIGHT: i32 = 500;

pub(crate) struct MyData {
    conf_file: Rc<RefCell<ConfigFile>>,
    //
    builder: GtkBuilder,
    im: MyImageArea,
    sp: gtk::Spinner,
    bytes: Option<GlibBytes>,
    pixbuf: Option<GdkPixbuf>,
}
impl MyData {
    fn new(
        conf_file: Rc<RefCell<ConfigFile>>,
        builder: GtkBuilder,
        da: gtk::DrawingArea,
        sp: gtk::Spinner,
    ) -> Self {
        Self {
            conf_file,
            //
            builder,
            im: MyImageArea::new(da),
            sp,
            bytes: None,
            pixbuf: None,
        }
    }
}

// gtk & thread_local
// https://gitlab.com/susurrus/gattii/-/blob/master/src/bin/gattii.rs
thread_local!(
    static UI_GLOBAL: RefCell<Option<(Rc<RefCell<MyData>>,i32)>> = RefCell::new(None)
);

fn build_ui(application: &gtk::Application, conf_file: Rc<RefCell<ConfigFile>>) {
    //let builder = gtk::Builder::from_file("ui/ImVm.glade");
    let builder = gtk::Builder::from_string(include_str!("../../ui/ImVm.glade"));
    builder.set_application(application);
    //
    let window: gtk::ApplicationWindow = builder.object("MainWin").unwrap();
    let da: gtk::DrawingArea = builder.object("drawing_area_main").unwrap();
    let sp: gtk::Spinner = builder.object("spinner_main").unwrap();
    window.set_default_size(WINDOW_DEFAULT_WIDTH, WINDOW_DEFAULT_HEIGHT);
    window.set_size_request(WINDOW_DEFAULT_WIDTH / 4, WINDOW_DEFAULT_HEIGHT / 4);
    window.show_all();
    //
    application.add_window(&window);
    //
    {
        let c_conf_file = conf_file.borrow();
        if c_conf_file.is_ok() {
            let prof = &c_conf_file.conf.default;
            window.move_(prof.geometry_x, prof.geometry_y);
            window.resize(prof.geometry_w, prof.geometry_h);
            window.set_decorated(prof.decorated);
            if prof.sticky {
                window.stick();
            } else {
                window.unstick();
            }
            window.set_keep_above(prof.above);
            window.set_keep_below(prof.below);
        }
    }
    //
    let my_data = Rc::new(RefCell::new(MyData::new(conf_file, builder, da, sp)));
    //
    UI_GLOBAL.with(move |global| {
        *global.borrow_mut() = Some((my_data, 0));
    });
    //
    UI_GLOBAL.with(|global| {
        if let Some((ref my_data, _)) = *global.borrow() {
            let a_my_data = my_data.borrow();
            a_my_data.im.setup_connect();
        }
    });
    //
    window.connect_delete_event(move |win, _| {
        UI_GLOBAL.with(move |global| {
            if let Some((ref my_data, _)) = *global.borrow() {
                let a_my_data = my_data.borrow_mut();
                let v_conf_file = &mut a_my_data.conf_file.borrow_mut();
                //
                let gdk_win = win.window().unwrap();
                let gdk_state = gdk_win.state();
                //
                let mut prof = &mut v_conf_file.conf.default;
                if win.is_maximized() {
                    // nothing todo
                } else {
                    let (x, y) = win.position();
                    let (w, h) = win.size();
                    prof.geometry_x = x;
                    prof.geometry_y = y;
                    prof.geometry_w = w;
                    prof.geometry_h = h;
                }
                //
                prof.decorated = win.is_decorated();
                prof.sticky = gdk_state.contains(gdk::WindowState::STICKY);
                // [BUG] window state has not actived ABOVE and BELOW
                prof.above = gdk_state.contains(gdk::WindowState::ABOVE);
                //prof.above = c_win_setting.is_above();
                prof.below = gdk_state.contains(gdk::WindowState::BELOW);
                //eprintln!("{:b}", gdk_state.bits());
                //
                //prof.transparent = c_win_setting.is_transparent();
                //prof.opaque_back = c_win_setting.is_opaque_back();
                //
                //
                let res = v_conf_file.save_to_config_file();
                if res.is_err() {
                    let err = res.err().unwrap();
                    eprintln!("{}", err);
                    eprintln!("{:?}", err);
                }
            }
        });
        gtk::Inhibit(false)
    });
}

pub fn gui_main(conf_file: Rc<RefCell<ConfigFile>>) {
    let app = gtk::Application::builder()
        .application_id("com.github.aki-akaguma.aki-image-view-gtk")
        .build();
    //
    app.connect_activate(move |app| {
        let conf_file = conf_file.clone();
        build_ui(app, conf_file);
    });
    app.connect_shutdown(|_app| {
        UI_GLOBAL.with(move |global| {
            *global.borrow_mut() = None;
        });
    });
    //
    app.run();
}

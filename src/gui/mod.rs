//
// https://www.ruimo.com/fromId?fromId=130
//

use gio::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::prelude::{BuilderExt, BuilderExtManual, GtkApplicationExt, GtkWindowExt, WidgetExt};

use gtk::Builder as GtkBuilder;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::Sender;

macro_rules! gui_trace {
    () => ({
        #[cfg(feature = "gui_trace")]
        {
            {
                use std::io::Write;
                //
                let stderr = std::io::stderr();
                let mut handle = stderr.lock();
                let _ = handle.write_fmt(format_args!("\n"));
            }
        }
    });
    ($fmt:tt) => ({
        #[cfg(feature = "gui_trace")]
        {
            {
                use std::io::Write;
                //
                let stderr = std::io::stderr();
                let mut handle = stderr.lock();
                static mut COUNT: u64 = 0;
                unsafe { COUNT += 1 };
                let _ = handle.write_fmt(format_args!("{:04} ", unsafe { COUNT }));
                let _ = handle.write_fmt(format_args!($fmt));
                let _ = handle.write_fmt(format_args!(" [{}:{}]\n", file!(), line!()));
            }
        }
    });
    ($fmt:tt, $($arg:tt)*) => ({
        #[cfg(feature = "gui_trace")]
        {
            {
                use std::io::Write;
                //
                let stderr = std::io::stderr();
                let mut handle = stderr.lock();
                static mut COUNT: u64 = 0;
                unsafe { COUNT += 1 };
                let _ = handle.write_fmt(format_args!("{:04} ", unsafe { COUNT }));
                let _ = handle.write_fmt(format_args!($fmt, $($arg)*));
                let _ = handle.write_fmt(format_args!(" [{}:{}]\n", file!(), line!()));
            }
        }
    });
}

mod guii;
mod image_area;
mod operation;
mod render_thr;
mod zoom;

use crate::conf::conf_file::ConfigFile;

pub const WINDOW_DEFAULT_WIDTH: i32 = 640;
pub const WINDOW_DEFAULT_HEIGHT: i32 = 500;

const ID_MAIN_WINDOW: &str = "MainWin";
const ID_MAIN_DRAWING_AREA: &str = "drawing_area_main";
const ID_MAIN_SCROLLED_WINDOW: &str = "scrolled_window_main";
const ID_MAIN_VIEWPORT: &str = "viewport_main";
const ID_MAIN_SPINNER: &str = "spinner_main";
//const ID_MENU_MAIN: &str = "popup_menu_main";
const ID_MENU_ZOOM: &str = "popup_menu_zoom";
const ID_MENU_ITEM_ZOOM_FIT: &str = "popup_menu_item_zoom_fit";

#[cfg(feature = "debian_build")]
macro_rules! ui_dir {
    () => {
        concat!("/usr/share/", env!("CARGO_PKG_NAME"), "/ui/")
    };
    ($fnm: expr) => {
        concat!(ui_dir!(), $fnm)
    };
}

#[cfg(not(feature = "debian_build"))]
macro_rules! ui_dir {
    () => {
        "../../ui/"
    };
    ($fnm: expr) => {
        concat!(ui_dir!(), $fnm)
    };
}

macro_rules! main_glade_name {
    () => {
        "ImVm.glade"
    };
}
macro_rules! menu_glade_name {
    () => {
        "Menu.glade"
    };
}

pub(crate) struct MyData {
    conf_file: Rc<RefCell<ConfigFile>>,
    //
    tx: Sender<render_thr::RenderThreadMsg>,
    //
    _builder: GtkBuilder,
    im: image_area::MyImageArea,
    zoom: zoom::MyZoom,
    sp: gtk::Spinner,
}
impl MyData {
    fn new(
        conf_file: Rc<RefCell<ConfigFile>>,
        tx: Sender<render_thr::RenderThreadMsg>,
        builder: GtkBuilder,
        da: gtk::DrawingArea,
        sp: gtk::Spinner,
    ) -> Self {
        let da_parent: gtk::ScrolledWindow = builder.object(ID_MAIN_SCROLLED_WINDOW).unwrap();
        let da_viewport: gtk::Viewport = builder.object(ID_MAIN_VIEWPORT).unwrap();
        let zoom_menu: gtk::Menu = builder.object(ID_MENU_ZOOM).unwrap();
        let zoom_menu_item_zoom_fit: gtk::CheckMenuItem =
            builder.object(ID_MENU_ITEM_ZOOM_FIT).unwrap();
        let zoom_in_btn: gtk::Button = builder.object("button_zoom_in").unwrap();
        let zoom_out_btn: gtk::Button = builder.object("button_zoom_out").unwrap();
        let zoom_entry: gtk::Entry = builder.object("entry_zoom").unwrap();
        //
        Self {
            conf_file,
            tx,
            //
            _builder: builder,
            im: image_area::MyImageArea::new(da_parent, da_viewport, da),
            zoom: zoom::MyZoom::new(
                zoom_in_btn,
                zoom_out_btn,
                zoom_entry,
                zoom_menu,
                zoom_menu_item_zoom_fit,
            ),
            sp,
        }
    }
}

// gtk & thread_local
// https://gitlab.com/susurrus/gattii/-/blob/master/src/bin/gattii.rs
thread_local!(
    static UI_GLOBAL: RefCell<Option<(Rc<RefCell<MyData>>,i32)>> = RefCell::new(None)
);

//
fn build_ui(
    application: &gtk::Application,
    tx: Sender<render_thr::RenderThreadMsg>,
    conf_file: Rc<RefCell<ConfigFile>>,
) {
    let builder = {
        #[cfg(feature = "debian_build")]
        {
            let builder = gtk::Builder::from_file(ui_dir!(main_glade_name!()));
            builder.add_from_file(ui_dir!(menu_glade_name!())).unwrap();
            builder
        }
        #[cfg(not(feature = "debian_build"))]
        {
            let builder = gtk::Builder::from_string(include_str!(ui_dir!(main_glade_name!())));
            builder
                .add_from_string(include_str!(ui_dir!(menu_glade_name!())))
                .unwrap();
            builder
        }
    };
    //
    builder.set_application(application);
    //
    let window: gtk::ApplicationWindow = builder.object(ID_MAIN_WINDOW).unwrap();
    let da: gtk::DrawingArea = builder.object(ID_MAIN_DRAWING_AREA).unwrap();
    let sp: gtk::Spinner = builder.object(ID_MAIN_SPINNER).unwrap();
    //let menu_main: gtk::Menu = builder.object(ID_MENU_MAIN).unwrap();
    //
    window.set_default_size(WINDOW_DEFAULT_WIDTH, WINDOW_DEFAULT_HEIGHT);
    window.set_size_request(WINDOW_DEFAULT_WIDTH / 4, WINDOW_DEFAULT_HEIGHT / 4);
    window.show_all();
    //
    application.add_window(&window);
    //
    {
        let c_conf_file = conf_file.borrow();
        if c_conf_file.is_ok() {
            /*
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
            */
        }
    }
    //
    let my_data = Rc::new(RefCell::new(MyData::new(conf_file, tx, builder, da, sp)));
    UI_GLOBAL.with(move |global| {
        *global.borrow_mut() = Some((my_data, 0));
    });
    //
    UI_GLOBAL.with(|global| {
        if let Some((ref my_data, _)) = *global.borrow() {
            let a_my_data = my_data.borrow();
            a_my_data.im.setup_connect();
            a_my_data.zoom.setup_connect();
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
    let (handle, tx) = render_thr::start_render_thread();
    //
    let app = gtk::Application::builder()
        .application_id("com.github.aki-akaguma.aki-image-view-gtk")
        .build();
    //
    let tx_thr = tx.clone();
    app.connect_activate(move |app| {
        let conf_file = conf_file.clone();
        build_ui(app, tx_thr.clone(), conf_file);
    });
    app.connect_shutdown(|_app| {
        UI_GLOBAL.with(move |global| {
            *global.borrow_mut() = None;
        });
    });
    //
    app.run();
    //
    tx.send(render_thr::RenderThreadMsg::Quit).unwrap();
    handle.join().unwrap();
}

//
// https://www.ruimo.com/fromId?fromId=130
//
use crate::conf::conf_file::ConfigFile;
use crate::gui::abdia;
use crate::gui::fcdia;

use gtk::prelude::{BuilderExtManual, GtkApplicationExt, GtkWindowExt, WidgetExt};

use gtk::Builder as GtkBuilder;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::Sender;

pub mod acti;
pub mod image_area;
pub mod render_thr;
pub mod zoom;

pub const WINDOW_DEFAULT_WIDTH: i32 = 820;
pub const WINDOW_DEFAULT_HEIGHT: i32 = 500;

const ID_MAIN_WINDOW: &str = "MainWin";
const ID_MAIN_DRAWING_AREA: &str = "drawing_area_main";
const ID_MAIN_SCROLLED_WINDOW: &str = "scrolled_window_main";
const ID_MAIN_VIEWPORT: &str = "viewport_main";
const ID_MAIN_SPINNER: &str = "spinner_main";
const ID_POPOVER_MENU_ZOOM_BOX: &str = "popover_menu_zoom_box";
const ID_POPOVER_MENU_ZOOM_ITEM_ZOOM_FIT: &str = "popover_menu_zoom_item_zoom_fit";
const ID_BUTTON_ZOOM_FIT: &str = "button_zoom_fit";

// gtk & thread_local
// https://gitlab.com/susurrus/gattii/-/blob/master/src/bin/gattii.rs
thread_local!(
    static UI_MWIN_GLOBAL: RefCell<Option<(Rc<RefCell<MyMainWin>>,i32)>> = RefCell::new(None)
);

pub(crate) struct MyMainWin {
    conf_file: Rc<RefCell<ConfigFile>>,
    tx: Sender<render_thr::RenderThreadMsg>,
    uri_s: String,
    //
    im: image_area::MyImageArea,
    zoom: zoom::MyZoom,
    sp: gtk::Spinner,
}
impl MyMainWin {
    fn new(
        conf_file: Rc<RefCell<ConfigFile>>,
        tx: Sender<render_thr::RenderThreadMsg>,
        builder: GtkBuilder,
        da: gtk::DrawingArea,
        sp: gtk::Spinner,
    ) -> Self {
        let da_parent: gtk::ScrolledWindow = builder.object(ID_MAIN_SCROLLED_WINDOW).unwrap();
        let da_viewport: gtk::Viewport = builder.object(ID_MAIN_VIEWPORT).unwrap();
        //
        let button_zoom_fit: gtk::ToggleButton = builder.object(ID_BUTTON_ZOOM_FIT).unwrap();
        let zoom_popover_menu_box: gtk::Box = builder.object(ID_POPOVER_MENU_ZOOM_BOX).unwrap();
        let zoom_popover_menu_item_zoom_fit: gtk::CheckButton =
            builder.object(ID_POPOVER_MENU_ZOOM_ITEM_ZOOM_FIT).unwrap();
        //
        let zoom_entry: gtk::Entry = builder.object("entry_zoom").unwrap();
        //
        Self {
            conf_file,
            uri_s: "".into(),
            tx,
            //
            im: image_area::MyImageArea::new(da_parent, da_viewport, da),
            zoom: zoom::MyZoom::new(
                zoom_entry,
                zoom_popover_menu_box,
                zoom_popover_menu_item_zoom_fit,
                button_zoom_fit,
            ),
            sp,
        }
    }
}

//
fn build_ui(
    builder: gtk::Builder,
    application: &gtk::Application,
    tx: Sender<render_thr::RenderThreadMsg>,
    conf_file: Rc<RefCell<ConfigFile>>,
) {
    //
    let window: gtk::ApplicationWindow = builder.object(ID_MAIN_WINDOW).unwrap();
    let da: gtk::DrawingArea = builder.object(ID_MAIN_DRAWING_AREA).unwrap();
    let sp: gtk::Spinner = builder.object(ID_MAIN_SPINNER).unwrap();
    //
    window.set_default_size(WINDOW_DEFAULT_WIDTH, WINDOW_DEFAULT_HEIGHT);
    window.set_size_request(WINDOW_DEFAULT_WIDTH / 4, WINDOW_DEFAULT_HEIGHT / 4);
    window.show_all();
    //
    application.add_window(&window);
    //
    {
        let builder = builder.clone();
        fcdia::init(builder);
    }
    {
        let builder = builder.clone();
        abdia::init(builder);
    }
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
    let my_data = Rc::new(RefCell::new(MyMainWin::new(conf_file, tx, builder, da, sp)));
    UI_MWIN_GLOBAL.with(move |global| {
        *global.borrow_mut() = Some((my_data, 0));
    });
    //
    UI_MWIN_GLOBAL.with(|global| {
        if let Some((ref my_data, _)) = *global.borrow() {
            let a_my_data = my_data.borrow();
            a_my_data.im.setup_connect();
        }
    });
    //
    window.connect_delete_event(move |win, _| {
        UI_MWIN_GLOBAL.with(move |global| {
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
    //
    super::acti::insert_app_action_group(&window);
    acti::insert_mwin_action_group(&window);
}

//
pub(crate) fn app_on_activate(
    builder: gtk::Builder,
    app: &gtk::Application,
    tx_thr: Sender<render_thr::RenderThreadMsg>,
    conf_file: Rc<RefCell<ConfigFile>>,
    img_path: String,
) {
    build_ui(builder, app, tx_thr, conf_file);
    if !img_path.is_empty() {
        let uri = format!("file:///{}", img_path);
        acti::ope_open_uri_for_image_file(uri.as_str());
    }
}

//
pub(crate) fn app_on_shutdown() {
    UI_MWIN_GLOBAL.with(move |global| {
        *global.borrow_mut() = None;
    });
}

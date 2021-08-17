use gdk::prelude::GdkContextExt;
use gio::prelude::FileExt;
use gtk::prelude::{ScrolledWindowExt, SpinnerExt, WidgetExt, WidgetExtManual};

use glib::clone;
use glib::signal::Inhibit;

use gdk_pixbuf::Pixbuf as GdkPixbuf;
use glib::Bytes as GlibBytes;

use std::cell::RefCell;
use std::rc::Rc;

use super::guii::Size2Di;
use super::operation::{ope_setup_scroll_window_content_wh, ope_update_zoom_entry};
use super::render_thr::RenderThreadMsg;
use super::MyData;
use super::UI_GLOBAL;

const MAX_PIXELS: i32 = 128 * 1024 * 1024 / 4;

pub(crate) struct MyImageArea {
    scrw: gtk::ScrolledWindow,
    vipo: gtk::Viewport,
    da: gtk::DrawingArea,
    //
    has_image_info: bool,
    orig_wh: Size2Di,
    render_wh: Size2Di,
    bytes: Option<GlibBytes>,
    pixbuf: Option<GdkPixbuf>,
    zoom_level: Option<f32>,
}

impl MyImageArea {
    pub fn new(scrw: gtk::ScrolledWindow, vipo: gtk::Viewport, da: gtk::DrawingArea) -> Self {
        Self {
            scrw,
            vipo,
            da,
            //
            has_image_info: false,
            orig_wh: Size2Di::new(0, 0),
            render_wh: Size2Di::new(0, 0),
            bytes: None,
            pixbuf: None,
            zoom_level: None,
        }
    }
    pub fn setup_connect(&self) {
        UI_GLOBAL.with(|global| {
            if let Some((ref my_data, _)) = *global.borrow() {
                setup_connect(&self.scrw, &self.da, my_data);
            }
        });
    }
    pub fn queue_draw(&self) {
        self.da.queue_draw();
    }
    pub fn scrw_width_height(&self) -> Size2Di {
        (self.scrw.allocated_width(), self.scrw.allocated_height()).into()
    }
    pub fn scrw_max_content_width_height(&self) -> Size2Di {
        let max_con_w = self.scrw.max_content_width();
        let max_con_h = self.scrw.max_content_height();
        (max_con_w, max_con_h).into()
    }
    pub fn vipo_width_height(&self) -> Size2Di {
        (self.vipo.allocated_width(), self.vipo.allocated_height()).into()
    }
    pub fn da_width_height(&self) -> Size2Di {
        (self.da.allocated_width(), self.da.allocated_height()).into()
    }
    pub fn requested_width_height(&self) -> Size2Di {
        (self.da.width_request(), self.da.height_request()).into()
    }
    pub fn zoom_fit_level(&self) -> f32 {
        let da_wh = self.da_width_height();
        let level_w = da_wh.w() as f32 / self.orig_wh.w() as f32;
        let level_h = da_wh.h() as f32 / self.orig_wh.h() as f32;
        if level_w >= level_h {
            level_w
        } else {
            level_h
        }
    }
    pub fn zoomed_width_height(&self) -> Size2Di {
        if let Some(zoom_val) = self.zoom_level {
            let zoomed_w = self.orig_wh.w() as f32 * zoom_val;
            let zoomed_h = self.orig_wh.h() as f32 * zoom_val;
            (zoomed_w, zoomed_h).into()
        } else {
            let scrw_wh = self.scrw_width_height();
            let scrw_w = scrw_wh.w() as f32;
            let scrw_h = scrw_wh.h() as f32;
            let new_w = scrw_h * self.orig_wh.w() as f32 / self.orig_wh.h() as f32;
            let new_h = scrw_w * self.orig_wh.h() as f32 / self.orig_wh.w() as f32;
            if new_w <= scrw_w {
                (new_w, scrw_h).into()
            } else if new_h <= scrw_h {
                (scrw_w, new_h).into()
            } else {
                gui_trace!(
                    "im.zoomed_width_height(): {}:{}",
                    Size2Di::from((scrw_w, scrw_h)),
                    Size2Di::from((new_w, new_h))
                );
                (scrw_w, scrw_h).into()
            }
        }
    }
    pub fn available_zoom_level(&self) -> f32 {
        let wxh = self.orig_wh.w() * self.orig_wh.h();
        MAX_PIXELS as f32 / wxh as f32
    }
    pub fn zoom_level(&self) -> Option<f32> {
        self.zoom_level
    }
    pub fn set_zoom_level(&mut self, level: f32) {
        self.zoom_level = Some(level);
    }
    pub fn set_zoom_fit(&mut self) {
        self.zoom_level = None;
    }
    pub fn is_zoom_fit(&self) -> bool {
        self.zoom_level.is_none()
    }
    pub fn set_max_content_wh(&self, alc_wh: Size2Di) {
        let max_con_wh = self.scrw_max_content_width_height();
        let _un = if max_con_wh != alc_wh {
            self.scrw.set_max_content_width(alc_wh.w());
            self.scrw.set_max_content_height(alc_wh.h());
            ""
        } else {
            "unset"
        };
        gui_trace!("set_max_content_wh(): {} {}", _un, alc_wh);
        //
        if self.is_zoom_fit() {
            let da_wh = self.da_width_height();
            let zoomed_wh = self.zoomed_width_height();
            let _un = if da_wh != zoomed_wh {
                self.da.set_size_request(zoomed_wh.w(), zoomed_wh.h());
                ""
            } else {
                "unset"
            };
            gui_trace!("set_max_content_wh(): da: {} {}", _un, zoomed_wh);
        }
    }
    fn set_bytes_and_clear(&mut self, bytes: GlibBytes) {
        self.bytes = Some(bytes);
        self.has_image_info = false;
        self.orig_wh = Size2Di::new(0, 0);
        self.render_wh = Size2Di::new(0, 0);
    }
}

pub(crate) fn setup_connect(
    scrw: &gtk::ScrolledWindow,
    da: &gtk::DrawingArea,
    my_data: &Rc<RefCell<MyData>>,
) {
    //
    let targets = vec![gtk::TargetEntry::new(
        "text/uri-list",
        gtk::TargetFlags::OTHER_APP,
        0,
    )];
    da.drag_dest_set(gtk::DestDefaults::ALL, &targets, gdk::DragAction::COPY);
    da.connect_drag_data_received(clone!(@strong my_data =>
        move |_w, _, _, _, d, _, _| {
        let uris = d.uris();
        if !uris.is_empty() {
            open_uri_for_image_file(&uris[0]);
            /*
            // uri (smb:// ***) also correspond
            let file = gio::File::for_uri(&uris[0]);
            file.load_contents_async::<gio::Cancellable,_>(None, move |r|{
                match r {
                    Ok((bytes_vec_u8, _opt_etag_out)) => {
                        //gui_trace!("etag_out: {}", _opt_etag_out.to_string());
                        UI_GLOBAL.with(|global| {
                            if let Some((ref my_data, _)) = *global.borrow() {
                                {
                                    let mut a_my_data = my_data.borrow_mut();
                                    let bytes = GlibBytes::from_owned(bytes_vec_u8);
                                    a_my_data.im.set_bytes_and_clear(bytes);
                                }
                                spawn_render_image(my_data.clone());
                            }
                        });
                    }
                    Err(err) => {
                        eprintln!("LOAD ERROR: {}", err.to_string());
                    }
                }
            });
            */
        }
    }));
    //
    da.connect_size_allocate(clone!(@strong my_data => move |_widget, _alc| {
        gui_trace!("da.connect_size_allocate(): {}x{}", _alc.width, _alc.height);
        check_and_do_zoom_fit(my_data.clone());
    }));
    scrw.connect_size_allocate(clone!(@strong my_data => move |_widget, _alc| {
        let a_my_data = my_data.borrow();
        if a_my_data.im.is_zoom_fit() {
            let scrw_wh: Size2Di = a_my_data.im.scrw_width_height();
            let vipo_wh: Size2Di = a_my_data.im.vipo_width_height();
            if scrw_wh != vipo_wh {
                gui_trace!("parent.connect_size_allocate(): {} != {}", scrw_wh, vipo_wh);
                let _ = glib::idle_add_once(move || {
                    //gui_trace!("glib::idle_add(): ope_setup_scroll_window_content_wh()");
                    ope_setup_scroll_window_content_wh(scrw_wh);
                });
            }
        }
    }));
    //
    da.connect_draw(clone!(@strong my_data => move |widget, cr| {
        da_on_draw(widget, cr, &my_data);
        Inhibit(false)
    }));
}

fn da_on_draw(widget: &gtk::DrawingArea, cr: &cairo::Context, my_data: &Rc<RefCell<MyData>>) {
    let (clip_x, clip_y, clip_w, clip_h) = if let Ok((x1, y1, x2, y2)) = cr.clip_extents() {
        (x1 as i32, y1 as i32, (x2 - x1) as i32, (y2 - y1) as i32)
    } else {
        (0, 0, 0, 0)
    };
    //
    // Show images with black background
    // Calculation of expansion by Matrix can be applied to PDF and SVG
    let da_iw = widget.allocated_width();
    let da_ih = widget.allocated_height();
    let da_w = da_iw as f64;
    let da_h = da_ih as f64;
    cr.set_source_rgb(1.0, 1.0, 1.0);
    cr.rectangle(0.0, 0.0, da_w, da_h);
    cr.fill().unwrap();
    //
    let a_my_data = my_data.borrow();
    if let Some(pixbuf) = &a_my_data.im.pixbuf {
        let vp_iw = a_my_data.im.vipo.allocated_width();
        let vp_ih = a_my_data.im.vipo.allocated_height();
        let pb_iw = pixbuf.width();
        let pb_ih = pixbuf.height();
        if pb_iw <= vp_iw && pb_ih <= vp_ih {
            //gui_trace!("da_on_drow()::full {}x{},{}x{}", da_iw, da_ih, vp_iw, vp_ih);
            if da_iw != pb_iw || da_ih != pb_ih {
                let n = if da_iw != pb_iw {
                    da_iw as f64 / pb_iw as f64
                } else {
                    da_ih as f64 / pb_ih as f64
                };
                #[rustfmt::skip]
                cr.transform(cairo::Matrix {
                    xx: n, yx: 0.0, xy: 0.0, yy: n, x0: 0.0, y0: 0.0
                });
                //gui_trace!("da_on_drow()::full scale {:.3}::{}x{},{}x{}", n, da_iw, da_ih, pb_iw, pb_ih);
            } else {
                #[rustfmt::skip]
                cr.transform(cairo::Matrix {
                    xx: 1.0, yx: 0.0, xy: 0.0, yy: 1.0, x0: 0.0, y0: 0.0
                });
                //gui_trace!("da_on_drow()::full {}x{},{}x{}", da_iw, da_ih, pb_iw, pb_ih);
            }
            cr.set_source_pixbuf(pixbuf, 0.0, 0.0);
        } else {
            let subpb_x = clip_x;
            let subpb_y = clip_y;
            let subpb_w = clip_w.min(pb_iw);
            let subpb_h = clip_h.min(pb_ih);
            //
            let subpb_w = if !(subpb_x >= 0 && subpb_x + subpb_w <= pb_iw) {
                pb_iw - subpb_x
            } else {
                subpb_w
            };
            let subpb_h = if !(subpb_y >= 0 && subpb_y + subpb_h <= pb_ih) {
                pb_ih - subpb_y
            } else {
                subpb_h
            };
            if subpb_w > 0 && subpb_h > 0 {
                //gui_trace!("da_on_drow()::new_subpixbuf() {}x{},{}x{}", subpb_x, subpb_y, subpb_w, subpb_h);
                let subpixbuf = pixbuf
                    .new_subpixbuf(subpb_x, subpb_y, subpb_w, subpb_h)
                    .unwrap();
                cr.set_source_pixbuf(&subpixbuf, subpb_x as f64, subpb_y as f64);
            }
        }
        //
        cr.paint().unwrap();
        //gui_trace!("da_on_drow(): paint done.");
    }
}

fn check_and_do_zoom_fit(my_data: Rc<RefCell<MyData>>) {
    {
        let a_my_data = my_data.borrow();
        if a_my_data.im.is_zoom_fit() {
            let scrw_wh = a_my_data.im.scrw_width_height();
            let da_wh = a_my_data.im.da_width_height();
            if !(da_wh.w() <= scrw_wh.w() && da_wh.h() <= scrw_wh.h()) {
                gui_trace!("check_and_do_zoom_fit(): {} <= {}", scrw_wh, da_wh);
                let _ = glib::idle_add_once(move || {
                    ope_setup_scroll_window_content_wh(scrw_wh);
                });
                return;
            }
        }
    }
    spawn_render_image(my_data);
}

pub(crate) fn spawn_render_image(my_data: Rc<RefCell<MyData>>) {
    let has_image_info = {
        let a_my_data = my_data.borrow();
        a_my_data.im.has_image_info
    };
    if !has_image_info {
        let a_my_data = my_data.borrow();
        if let Some(bytes) = &a_my_data.im.bytes {
            let bytes = bytes.clone();
            std::thread::spawn(move || {
                setup_image_info(&bytes);
            });
        }
    } else {
        spawn_render_image_0(my_data);
    }
}

fn setup_image_info(bytes: &GlibBytes) {
    let (width, height) = {
        let input_stream = gio::MemoryInputStream::from_bytes(bytes);
        #[rustfmt::skip]
        let r = GdkPixbuf::from_stream::<_, gio::Cancellable>(&input_stream, None);
        match r {
            Ok(pb) => (pb.width(), pb.height()),
            Err(err) => {
                eprintln!("LOAD ERROR: {}", err.to_string());
                gui_trace!("setup_image_info(): {}", err.to_string());
                return;
            }
        }
    };
    //
    glib::idle_add_once(move || {
        UI_GLOBAL.with(|global| {
            if let Some((ref my_data, _)) = *global.borrow() {
                {
                    let mut a_my_data = my_data.borrow_mut();
                    a_my_data.im.orig_wh = (width, height).into();
                    a_my_data.im.has_image_info = true;
                }
                spawn_render_image_0(my_data.clone());
            }
        });
    });
}

fn spawn_render_image_0(my_data: Rc<RefCell<MyData>>) {
    let mut a_my_data = my_data.borrow_mut();
    if let Some(bytes) = &a_my_data.im.bytes {
        let zoomed_wh = a_my_data.im.zoomed_width_height();
        if zoomed_wh == a_my_data.im.render_wh {
            return;
        }
        let req_wh = a_my_data.im.requested_width_height();
        if req_wh != zoomed_wh {
            gui_trace!("spawn_render_image_0(): request: {}", zoomed_wh);
            a_my_data.im.da.set_width_request(zoomed_wh.w());
            a_my_data.im.da.set_height_request(zoomed_wh.h());
            return;
        }
        //
        gui_trace!("spawn_render_image_0(): {}", zoomed_wh);
        a_my_data.sp.start();
        a_my_data.sp.set_visible(true);
        let bytes = bytes.clone();
        a_my_data.im.render_wh = zoomed_wh;
        a_my_data
            .tx
            .send(RenderThreadMsg::Render(bytes, zoomed_wh))
            .unwrap();
    }
}

pub(crate) fn render_image_on_thread(bytes: &GlibBytes, iwh: Size2Di) {
    // In the case of SVG, it is rendered here.
    let pixbuf = {
        let input_stream = gio::MemoryInputStream::from_bytes(bytes);
        #[rustfmt::skip]
        let r = GdkPixbuf::from_stream_at_scale::<_, gio::Cancellable>(
            &input_stream, iwh.w(), -1, true, None);
        match r {
            Ok(pb) => pb,
            Err(err) => {
                eprintln!("LOAD ERROR: {}", err.to_string());
                gui_trace!("render_image_on_thread(): {}", err.to_string());
                return;
            }
        }
    };
    //
    let bytes = match pixbuf.read_pixel_bytes() {
        Some(bytes) => bytes,
        None => {
            eprintln!("LOAD ERROR: pixbuf.read_pixel_bytes()");
            gui_trace!("render_image_on_thread(): pixbuf.read_pixel_bytes()");
            return;
        }
    };
    let colorspace = pixbuf.colorspace();
    let has_alpha = pixbuf.has_alpha();
    let bits_per_sample = pixbuf.bits_per_sample();
    let width = pixbuf.width();
    let height = pixbuf.height();
    let rowstride = pixbuf.rowstride();
    gui_trace!("render_image_on_thread(): {} => {}x{}", iwh, width, height);
    //
    glib::source::idle_add_once(move || {
        // feature = "v2_32"
        #[rustfmt::skip]
        let pixbuf = GdkPixbuf::from_bytes(
            &bytes, colorspace, has_alpha, bits_per_sample, width, height, rowstride);
        UI_GLOBAL.with(|global| {
            if let Some((ref my_data, _)) = *global.borrow() {
                let mut a_my_data = my_data.borrow_mut();
                a_my_data.im.pixbuf = Some(pixbuf);
                a_my_data.sp.stop();
                a_my_data.sp.set_visible(false);
                a_my_data.im.queue_draw();
            }
        });
        //
        glib::source::idle_add_once(move || {
            ope_update_zoom_entry();
        });
    });
}

pub(crate) fn open_uri_for_image_file(uri_str: &str) {
    if uri_str.is_empty() {
        return;
    }
    // uri (smb:// ***) also correspond
    gui_trace!("open_uri_for_image_file(): '{}'", uri_str);
    let file = gio::File::for_uri(uri_str);
    file.load_contents_async::<gio::Cancellable,_>(None, move |r|{
        match r {
            Ok((bytes_vec_u8, _opt_etag_out)) => {
                //gui_trace!("etag_out: {}", _opt_etag_out.to_string());
                UI_GLOBAL.with(|global| {
                    if let Some((ref my_data, _)) = *global.borrow() {
                        {
                            let mut a_my_data = my_data.borrow_mut();
                            let bytes = GlibBytes::from_owned(bytes_vec_u8);
                            a_my_data.im.set_bytes_and_clear(bytes);
                        }
                        spawn_render_image(my_data.clone());
                    }
                });
            }
            Err(err) => {
                eprintln!("LOAD ERROR: {}", err.to_string());
            }
        }
    });
}

use gdk::prelude::GdkContextExt;
use gio::prelude::{Continue, FileExt};
use gtk::prelude::{SpinnerExt, WidgetExt, WidgetExtManual};

use glib::clone;
use glib::signal::Inhibit;

use gdk_pixbuf::Pixbuf as GdkPixbuf;
use glib::Bytes as GlibBytes;

use std::cell::RefCell;
use std::rc::Rc;

use super::MyData;
use super::UI_GLOBAL;

pub(crate) struct MyImageArea {
    da: gtk::DrawingArea,
}

impl MyImageArea {
    pub fn new(da: gtk::DrawingArea) -> Self {
        Self { da }
    }
    pub fn allocated_width_height(&self) -> (i32, i32) {
        (self.da.allocated_width(), self.da.allocated_height())
    }
    pub fn queue_draw(&self) {
        self.da.queue_draw();
    }
    pub fn setup_connect(&self) {
        UI_GLOBAL.with(|global| {
            if let Some((ref my_data, _)) = *global.borrow() {
                //let mut a_my_data = my_data.borrow_mut();
                setup_connect(&self.da, my_data);
            }
        });
    }
}

pub(crate) fn setup_connect(da: &gtk::DrawingArea, my_data: &Rc<RefCell<MyData>>) {
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
            // uri (smb:// ***) also correspond
            let file = gio::File::for_uri(&uris[0]);
            file.load_contents_async::<gio::Cancellable,_>(None, move |r|{
                match r {
                    Ok((bytes_vec_u8, _opt_etag_out)) => {
                        //eprintln!("etag_out: {}", _opt_etag_out.to_string());
                        UI_GLOBAL.with(|global| {
                            if let Some((ref my_data, _)) = *global.borrow() {
                                {
                                    let mut a_my_data = my_data.borrow_mut();
                                    a_my_data.bytes = Some(GlibBytes::from_owned(bytes_vec_u8));
                                }
                                spawn_load_image(my_data.clone());
                            }
                        });
                    }
                    Err(err) => {
                        eprintln!("LOAD ERROR: {}", err.to_string());
                    }
                }
            });
        }
    }));
    da.connect_size_allocate(clone!(@strong my_data => move |_widget, _alc| {
        spawn_load_image(my_data.clone());
    }));
    da.connect_draw(clone!(@strong my_data => move |widget, cr| {
        // Show images with black background
        // Calculation of expansion by Matrix can be applied to PDF and SVG
        //
        let iw = widget.allocated_width();
        let ih = widget.allocated_height();
        let aw = iw as f64;
        let ah = ih as f64;
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.rectangle(0.0, 0.0, aw, ah);
        cr.fill().unwrap();
        //
        let my_data = my_data.borrow();
        if let Some(pixbuf) = &my_data.pixbuf {
            let w = pixbuf.width() as f64;
            let h = pixbuf.height() as f64;
            // matrix calculation to match the smaller
            if aw * h > ah * w {
                let n = ah/h;
                #[rustfmt::skip]
                cr.transform(cairo::Matrix {
                    xx: n, yx: 0.0, xy: 0.0, yy: n, x0: (aw - w*n)/2.0, y0: 0.0
                });
            } else {
                let n = aw/w;
                #[rustfmt::skip]
                cr.transform(cairo::Matrix {
                    xx: n, yx: 0.0, xy: 0.0, yy: n, x0: n, y0: (ah - h*n)/2.0
                });
            }
            //
            cr.set_source_pixbuf(&pixbuf, 0.0, 0.0);
            cr.paint().unwrap();
        }
        //
        Inhibit(false)
    }));
}

fn spawn_load_image(my_data: Rc<RefCell<MyData>>) {
    let a_my_data = my_data.borrow();
    let (iw, ih) = a_my_data.im.allocated_width_height();
    if let Some(bytes) = &a_my_data.bytes {
        a_my_data.sp.start();
        a_my_data.sp.set_visible(true);
        let bytes = bytes.clone();
        std::thread::spawn(move || {
            load_image_on_thread(&bytes, iw, ih);
        });
    }
}

fn load_image_on_thread(bytes: &GlibBytes, iw: i32, ih: i32) {
    // In the case of SVG, it is rendered here.
    let pixbuf = {
        let input_stream = gio::MemoryInputStream::from_bytes(bytes);
        #[rustfmt::skip]
        let r = GdkPixbuf::from_stream_at_scale::<_, gio::Cancellable>(
            &input_stream, iw, ih, true, None);
        match r {
            Ok(pb) => pb,
            Err(err) => {
                eprintln!("LOAD ERROR: {}", err.to_string());
                return;
            }
        }
    };
    //
    let bytes = match pixbuf.read_pixel_bytes() {
        Some(bytes) => bytes,
        None => {
            eprintln!("LOAD ERROR: pixbuf.read_pixel_bytes()");
            return;
        }
    };
    let colorspace = pixbuf.colorspace();
    let has_alpha = pixbuf.has_alpha();
    let bits_per_sample = pixbuf.bits_per_sample();
    let width = pixbuf.width();
    let height = pixbuf.height();
    let rowstride = pixbuf.rowstride();
    //
    glib::timeout_add(std::time::Duration::from_millis(30), move || {
        // feature = "v2_32"
        #[rustfmt::skip]
        let pixbuf = GdkPixbuf::from_bytes(
            &bytes, colorspace, has_alpha, bits_per_sample, width, height, rowstride);
        UI_GLOBAL.with(|global| {
            if let Some((ref my_data, _)) = *global.borrow() {
                let mut a_my_data = my_data.borrow_mut();
                a_my_data.pixbuf = Some(pixbuf);
                a_my_data.sp.stop();
                a_my_data.sp.set_visible(false);
                a_my_data.im.queue_draw();
            }
        });
        Continue(false)
    });
}

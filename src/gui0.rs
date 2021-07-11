//
// https://www.ruimo.com/fromId?fromId=130
//

use gdk::prelude::GdkContextExt;
use gio::prelude::{ApplicationExt, ApplicationExtManual, Continue, FileExt};
use glib::clone;
use glib::signal::Inhibit;
use gtk::prelude::{
    ContainerExt, GtkWindowExt, OverlayExt, SpinnerExt, WidgetExt, WidgetExtManual,
};

use std::cell::RefCell;
use std::rc::Rc;

struct MyData {
    da: gtk::DrawingArea,
    sp: gtk::Spinner,
    bytes: Option<glib::Bytes>,
    pixbuf: Option<gdk_pixbuf::Pixbuf>,
}

// gtk & thread_local
// https://gitlab.com/susurrus/gattii/-/blob/master/src/bin/gattii.rs
thread_local!(
    static UI_GLOBAL: RefCell<Option<(Rc<RefCell<MyData>>,i32)>> = RefCell::new(None)
);

fn load_image(my_data: Rc<RefCell<MyData>>) {
    let a_my_data = my_data.borrow();
    let (iw, ih) = (
        a_my_data.da.allocated_width(),
        a_my_data.da.allocated_height(),
    );
    if let Some(bytes) = &a_my_data.bytes {
        a_my_data.sp.start();
        a_my_data.sp.set_visible(true);
        let bytes = bytes.clone();
        std::thread::spawn(move || {
            load_image_on_thread(&bytes, iw, ih);
        });
    }
}

fn load_image_on_thread(bytes: &glib::Bytes, iw: i32, ih: i32) {
    // In the case of SVG, it is rendered here.
    let pixbuf = {
        let input_stream = gio::MemoryInputStream::from_bytes(bytes);
        #[rustfmt::skip]
        let r = gdk_pixbuf::Pixbuf::from_stream_at_scale::<_, gio::Cancellable>(
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
        let pixbuf = gdk_pixbuf::Pixbuf::from_bytes(
            &bytes, colorspace, has_alpha, bits_per_sample, width, height, rowstride);
        UI_GLOBAL.with(|global| {
            if let Some((ref my_data, _)) = *global.borrow() {
                let mut a_my_data = my_data.borrow_mut();
                a_my_data.pixbuf = Some(pixbuf);
                a_my_data.sp.stop();
                a_my_data.sp.set_visible(false);
                a_my_data.da.queue_draw();
            }
        });
        Continue(false)
    });
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("DnD Image Viewer");
    //window.set_border_width(10);
    //window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(400, 300);
    //
    let da = gtk::DrawingArea::new();
    let sp = gtk::SpinnerBuilder::new().no_show_all(true).build();
    //
    let my_data = Rc::new(RefCell::new(MyData {
        da: da.clone(),
        sp: sp.clone(),
        bytes: None,
        pixbuf: None,
    }));
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
            // uri (smb://***) also correspond
            let file = gio::File::for_uri(&uris[0]);
            let vec_u8 = match file.load_contents::<gio::Cancellable>(None) {
                Ok((bytes, _etag_out)) => {
                    //eprintln!("etag_out: {}", _etag_out.to_string());
                    bytes
                }
                Err(err) => {
                    eprintln!("LOAD ERROR: {}", err.to_string());
                    return;
                }
            };
            {
                let mut a_my_data = my_data.borrow_mut();
                a_my_data.bytes = Some(glib::Bytes::from_owned(vec_u8));
            }
            load_image(my_data.clone());
        }
    }));
    da.connect_size_allocate(clone!(@strong my_data => move |_widget, _alc| {
        load_image(my_data.clone());
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
    //
    let ov = gtk::Overlay::new();
    ov.add(&da);
    ov.add_overlay(&sp);
    //
    window.add(&ov);
    window.show_all();
    //
    UI_GLOBAL.with(move |global| {
        *global.borrow_mut() = Some((my_data, 0));
    });
}

pub fn gui_main() {
    let app = gtk::Application::builder()
        .application_id("com.github.aki-akaguma.aki-image-view-gtk")
        .build();
    //
    app.connect_activate(|app| {
        build_ui(app);
    });
    app.connect_shutdown(|_app| {
        UI_GLOBAL.with(move |global| {
            *global.borrow_mut() = None;
        });
    });
    //
    app.run();
}

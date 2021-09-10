//
// https://www.ruimo.com/fromId?fromId=130
//
use crate::conf::conf_file::ConfigFile;
use crate::conf::CmdOptConf;

use gio::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::prelude::BuilderExt;

use std::cell::RefCell;
use std::rc::Rc;

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
    ($fmt:expr) => ({
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
    ($fmt:expr, $($arg:tt)*) => ({
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

//
mod acti;
mod dia;
mod guii;
mod mwin;

pub fn gui_main(conf: &CmdOptConf, conf_file: Rc<RefCell<ConfigFile>>) {
    let img_path: String = if !conf.arg_params.is_empty() {
        conf.arg_params[0].clone()
    } else {
        "".to_string()
    };
    let (handle, tx) = mwin::render_thr::start_render_thread();
    //
    let app = gtk::Application::builder()
        .application_id("com.github.aki-akaguma.aki-image-view-gtk")
        .build();
    //
    let tx_thr = tx.clone();
    app.connect_activate(move |app| {
        //
        let builder = create_gtk_builder();
        //
        builder.set_application(app);
        mwin::app_on_activate(
            builder,
            app,
            tx_thr.clone(),
            conf_file.clone(),
            img_path.clone(),
        );
    });
    app.connect_shutdown(|_app| {
        mwin::app_on_shutdown();
    });
    //
    //app.run();
    app.run_with_args::<&str>(&[]);
    //
    tx.send(mwin::render_thr::RenderThreadMsg::Quit).unwrap();
    handle.join().unwrap();
}

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
        "Mwin.glade"
    };
}
macro_rules! dialog_glade_name {
    () => {
        "Fcdia.glade"
    };
}

#[cfg(feature = "debian_build")]
fn create_gtk_builder() -> gtk::Builder {
    use gtk::prelude::BuilderExtManual;
    let builder = gtk::Builder::from_file(ui_dir!(main_glade_name!()));
    builder
        .add_from_file(ui_dir!(dialog_glade_name!()))
        .unwrap();
    builder
}

#[cfg(not(feature = "debian_build"))]
fn create_gtk_builder() -> gtk::Builder {
    let builder = gtk::Builder::from_string(include_str!(ui_dir!(main_glade_name!())));
    builder
        .add_from_string(include_str!(ui_dir!(dialog_glade_name!())))
        .unwrap();
    builder
}

use crate::gui::mwin::operation::ope_open_uri_for_image_file;

use gtk::prelude::{BuilderExtManual, DialogExt, FileChooserExt, WidgetExt};

use gtk::Builder as GtkBuilder;

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

const ID_MAIN_FILE_CHOOSER: &str = "MainFileChooser";

// gtk & thread_local
// https://gitlab.com/susurrus/gattii/-/blob/master/src/bin/gattii.rs
thread_local!(
    static UI_DLG_GLOBAL: RefCell<Option<(Rc<RefCell<MyFileChooser>>,i32)>> = RefCell::new(None)
);

pub(crate) fn init(builder: GtkBuilder) {
    let my_fich = Rc::new(RefCell::new(MyFileChooser::new(builder)));
    UI_DLG_GLOBAL.with(move |global| {
        *global.borrow_mut() = Some((my_fich, 0));
    });
}

pub(crate) struct MyFileChooser {
    fc: gtk::FileChooserDialog,
    last_dir: Option<PathBuf>,
    last_filename: Option<PathBuf>,
}
impl MyFileChooser {
    pub fn new(builder: GtkBuilder) -> Self {
        let fc: gtk::FileChooserDialog = builder.object(ID_MAIN_FILE_CHOOSER).unwrap();
        //
        Self {
            fc,
            last_dir: None,
            last_filename: None,
        }
    }
    pub fn run_open_file(&mut self) -> Option<String> {
        if let Some(ref filename) = self.last_filename {
            let b = self.fc.select_filename(filename);
            if !b {
                if let Some(ref dir) = self.last_dir {
                    self.fc.set_current_folder(dir);
                }
            }
        }
        //
        let rt = self.fc.run();
        //
        let opt_filename = if rt == gtk::ResponseType::Ok {
            if let Some(dir_path_buf) = self.fc.current_folder() {
                self.last_dir = Some(dir_path_buf);
            }
            //
            let filename = self.fc.filename();
            if let Some(ref path_buf) = filename {
                self.last_filename = Some(path_buf.clone());
            }
            gui_trace!("run_open_file() choose: {:?}", filename);
            filename
        } else {
            gui_trace!("run_open_file(): {:?}", rt);
            None
        };
        self.fc.hide();
        opt_filename.map(|path| path.to_string_lossy().to_string())
    }
}

pub(crate) fn open_file_chooser() {
    UI_DLG_GLOBAL.with(|global| {
        if let Some((ref my_fich, _)) = *global.borrow() {
            {
                let mut a_my_fich = my_fich.borrow_mut();
                if let Some(filename) = a_my_fich.run_open_file() {
                    let uri = format!("file:///{}", filename);
                    ope_open_uri_for_image_file(uri.as_str());
                }
            }
        }
    });
}

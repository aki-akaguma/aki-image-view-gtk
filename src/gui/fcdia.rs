use crate::gui::guii::Size2Di;
use crate::gui::mwin::acti;

use gdk_pixbuf::Pixbuf;
use gtk::prelude::{BuilderExtManual, DialogExt, FileChooserExt, ImageExt, WidgetExt};

use gtk::Builder as GtkBuilder;

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

const ID_MAIN_FILE_CHOOSER: &str = "MainFileChooser";
const ID_PREV_IMAGE: &str = "prev_image1";

// gtk & thread_local
// https://gitlab.com/susurrus/gattii/-/blob/master/src/bin/gattii.rs
thread_local!(
    static UI_FC_DLG_GLOBAL: RefCell<Option<(Rc<RefCell<MyFileChooser>>,i32)>> = RefCell::new(None)
);

pub(crate) fn init(builder: GtkBuilder) {
    let my_fich = Rc::new(RefCell::new(MyFileChooser::new(builder)));
    UI_FC_DLG_GLOBAL.with(move |global| {
        *global.borrow_mut() = Some((my_fich, 0));
    });
}

pub(crate) struct MyFileChooser {
    fc: gtk::FileChooserDialog,
    last_dir: Option<PathBuf>,
    last_filename: Option<PathBuf>,
    _img: gtk::Image,
}
impl MyFileChooser {
    pub fn new(builder: GtkBuilder) -> Self {
        let fc: gtk::FileChooserDialog = builder.object(ID_MAIN_FILE_CHOOSER).unwrap();
        let img: gtk::Image = builder.object(ID_PREV_IMAGE).unwrap();
        {
            let img = img.clone();
            fc.connect_selection_changed(move |fc| {
                fc_on_selection_changed(fc, &img);
            });
        }
        //
        Self {
            fc,
            last_dir: None,
            last_filename: None,
            _img: img,
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

fn fc_on_selection_changed(fc: &gtk::FileChooserDialog, img: &gtk::Image) {
    macro_rules! _fn_nm {
        () => {
            "fc.connect_selection_changed()"
        };
    }
    let parent = img.parent().unwrap();
    let parent_wh: Size2Di = (parent.allocated_width(), parent.allocated_height()).into();
    let ok = if let Some(ref path_buf) = fc.preview_filename() {
        let ok = match Pixbuf::from_file_at_size(path_buf, parent_wh.w(), -1) {
            Ok(pixbuf) => {
                if pixbuf.height() <= parent_wh.h() {
                    //gui_trace!(concat!(_fn_nm!(), ": {:?}: {}"), path_buf, parent_wh);
                    img.set_pixbuf(Some(&pixbuf));
                    true
                } else {
                    false
                }
            }
            Err(_err) => {
                #[rustfmt::skip]
                gui_trace!(concat!(_fn_nm!(), ": Pixbuf::from_file_at_size(): {}"), _err);
                false
            }
        };
        if ok {
            true
        } else {
            match Pixbuf::from_file_at_size(path_buf, -1, parent_wh.h()) {
                Ok(pixbuf) => {
                    if pixbuf.width() <= parent_wh.w() {
                        //gui_trace!(concat!(_fn_nm!(), ": {:?}: {}"), path_buf, parent_wh);
                        img.set_pixbuf(Some(&pixbuf));
                        true
                    } else {
                        false
                    }
                }
                Err(_err) => {
                    #[rustfmt::skip]
                    gui_trace!(concat!(_fn_nm!(), ": Pixbuf::from_file_at_size(): {}"), _err);
                    false
                }
            }
        }
    } else {
        false
    };
    if !ok {
        //#[rustfmt::skip]
        //gui_trace!(concat!(_fn_nm!(), ": icon: {:?}: {}"), img.icon_name(), parent_wh);
        img.set_icon_name(None);
    }
}

pub(crate) fn open_file_chooser() {
    UI_FC_DLG_GLOBAL.with(|global| {
        if let Some((ref my_fich, _)) = *global.borrow() {
            {
                let mut a_my_fich = my_fich.borrow_mut();
                if let Some(filename) = a_my_fich.run_open_file() {
                    let uri = format!("file:///{filename}");
                    acti::ope_open_uri_for_image_file(uri.as_str());
                }
            }
        }
    });
}

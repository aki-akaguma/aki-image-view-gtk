use gtk::prelude::{DialogExt, FileChooserExt, WidgetExt};

pub(crate) struct MyFileChooser {
    fc: gtk::FileChooserDialog,
}
impl MyFileChooser {
    pub fn new(fc: gtk::FileChooserDialog) -> Self {
        Self { fc }
    }
    pub fn run_open_file(&self) -> Option<String> {
        let rt = self.fc.run();
        let opt_filename = if rt == gtk::ResponseType::Ok {
            let filename = self.fc.filename();
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

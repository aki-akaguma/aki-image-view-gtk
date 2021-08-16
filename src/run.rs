use crate::conf::CmdOptConf;
use std::cell::RefCell;
use std::rc::Rc;

const CONFIG_FILE_NAME: &str = concat!(env!("CARGO_PKG_NAME"), ".conf");

pub fn run(conf: &CmdOptConf) -> anyhow::Result<()> {
    //println!("{:?}", conf);
    //
    let conf_file = {
        let s = match conf.opt_config {
            Some(ref s) => s.clone(),
            None => {
                let s = match std::env::var("HOME") {
                    Ok(s) => s + "/.config/",
                    Err(_) => "/etc".to_string(),
                };
                s + CONFIG_FILE_NAME
            }
        };
        let c_path = std::path::PathBuf::from(s);
        let conf_file = crate::conf::conf_file::ConfigFile::load_from_config_file(c_path);
        if conf_file.is_err() {
            if let Err(ref err) = conf_file.err {
                eprintln!("ConfigFile: {}", err.to_string());
                eprintln!("ConfigFile: {:?}", err);
            }
        }
        Rc::new(RefCell::new(conf_file))
    };
    //
    crate::gui::gui_main(conf_file);
    //
    //
    Ok(())
}

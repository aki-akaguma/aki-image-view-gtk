use crate::util::OptUcXParam;
use flood_tide::OptParseErrors;
pub use parse::CmdOptConf;

pub mod conf_file;
mod parse;

impl CmdOptConf {
    /*
    pub fn base_dir(&self) -> String {
        for o in self.opt_uc_x.iter() {
            if let OptXParam::BaseDir(s) = o {
                return s.clone();
            }
        }
        String::new()
    }
    */
    pub fn is_opt_uc_x_help(&self) -> bool {
        for o in self.opt_uc_x.iter() {
            if let OptUcXParam::Help = o {
                return true;
            }
        }
        false
    }
    pub fn is_opt_uc_x_package_version_info(&self) -> bool {
        for o in self.opt_uc_x.iter() {
            if let OptUcXParam::PackageVersionInfo = o {
                return true;
            }
        }
        false
    }
}

pub fn parse_cmdopts() -> Result<CmdOptConf, OptParseErrors> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let _program = env_args.remove(0);
    let program = env!("CARGO_PKG_NAME");
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse::parse_cmdopts(program, &env_args)
}

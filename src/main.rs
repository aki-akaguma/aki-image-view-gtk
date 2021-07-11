/*
 * gtk+ sample program.
 *
 * cairo clock is wall clock. it is drawn by cairo library.
 *
 * https://gitlab.gnome.org/GNOME/gtkmm-documentation/tree/master/examples/book/drawingarea/clock
*/

use flood_tide::HelpVersion;

mod conf;
mod gui;
mod run;
mod util;

const TRY_HELP_MSG: &str = "Try --help for help.";

fn main() {
    memx_cdy::memx_init(); // fast mem operation.
                           //
    let conf = match conf::parse_cmdopts() {
        Ok(conf) => conf,
        Err(errs) => {
            for err in errs.iter().take(1) {
                if err.is_help() || err.is_version() {
                    println!("{}", err);
                    std::process::exit(0);
                }
            }
            eprintln!("{}\n{}", errs, TRY_HELP_MSG);
            std::process::exit(1);
        }
    };
    //
    match run::run(&conf) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}: {}", conf.prog_name, err);
            std::process::exit(1);
        }
    };
    //
    std::process::exit(0);
}

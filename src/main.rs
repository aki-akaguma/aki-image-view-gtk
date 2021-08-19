/*!
image view gtk gui application.

`aki-image-view-gtk` command is gtk image viewer.

* minimum support gtk v3_24
* minimum support rustc 1.51.0 (2fd73fabe 2021-03-23)

# Command help

```text
aki-image-view-gtk --help
```

```text
Usage:
  aki-image-view-gtk [options]

image view gtk gui application

Options:
  -c, --config <file>   config file path

  -H, --help        display this help and exit
  -V, --version     display version information and exit
  -X <x-options>    x options. try -X help
```

## Quick install

1. you can install this into cargo bin path:

```text
cargo install aki-image-view-gtk
```

2. you can build debian package:

```text
cargo deb
```

and install **.deb** into your local repository of debian package.
*/

use flood_tide::HelpVersion;

mod conf;
mod gui;
mod run;
mod util;

const TRY_HELP_MSG: &str = "Try --help for help.";

fn main() {
    // fast mem operation.
    //memx_cdy::memx_init();
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

use anyhow::{Ok, Result};
use core_loader::Core;
use log::error;

const HELP: &str = "\
retrovert-console
USAGE:
  retrovert-console [ARGS]
  --core        PATH    Override path to core library";

fn main() -> Result<()> {
    Core::init_logging()?;

    let mut pargs = pico_args::Arguments::from_env();

    dbg!(&pargs);

    // Help has a higher priority and should be handled separately.
    //if pargs.contains(["-h", "--help"]) {
    //    print!("{}", HELP);
    //    return Ok(());
    //}

    // override core location
    let core_over: Option<String> = pargs.opt_value_from_str("--core")?;

    let loaded_core = match Core::load_core(&core_over) {
        Result::Ok(lib) => lib,
        Err(e) => {
            error!("Unable to load corelib: {}", e);
            return Err(e);
        }
    };

    let core = match Core::new(&loaded_core) {
        Result::Ok(c) => c,
        Err(e) => {
            error!("Unable to create core {}", e);
            print!("{}", HELP);
            return Err(e);
        }
    };

    if pargs.contains(["-h", "--help"]) {
        println!("{}", HELP);
        (core.core_show_args)();
        return Ok(());
    }

    let core_instance = (core.core_create_func)();
    (core.core_update_func)(core_instance);
    (core.core_destroy_func)(core_instance, false);

    Ok(())
}

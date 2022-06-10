use anyhow::{Ok, Result};
use core_loader::Core;
use log::error;
use pancurses::{Input};
use cfixed_string::CFixedString;

const HELP: &str = "\
retrovert-console
USAGE:
  retrovert-console [ARGS]
  --core        PATH    Override path to core library";

fn main() -> Result<()> {
    Core::init_logging()?;

    let mut pargs = pico_args::Arguments::from_env();

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

    /*
    let window = pancurses::initscr();
    window.printw("Type things, press delete to quit\n");
    window.keypad(true);
    pancurses::noecho();

    loop {
        window.refresh();
        match window.getch() {
            Some(Input::Character(c)) => { window.addch(c); },
            Some(Input::KeyBackspace) => break,
            Some(input) => { window.addstr(&format!("{:?}", input)); },
            None => ()
        }
    
        (core.core_update_func)(core_instance);
    }

    pancurses::endwin();
     */

     while let Some(play_url) = pargs.opt_value_from_str::<_, String>("--play")? {
        let name_c = CFixedString::from_str(&play_url);
        (core.core_load_url)(core_instance, name_c.as_ptr() as _);
     }


    loop {
        (core.core_update_func)(core_instance);
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    (core.core_destroy_func)(core_instance, false);

    Ok(())
}

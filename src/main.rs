use anyhow::Result;
use core_loader::Core;
use log::{debug, error, info, warn};

fn main() -> Result<()> {
    Core::init_logging()?;

    error!("Bright red error");
    info!("This only appears in the log file");
    debug!("This level is currently not enabled for any logger");

    let loaded_core = Core::load_core()?;
    let mut core = Core::new(&loaded_core)?;

    let core_instance = (core.core_create_func)();
    (core.core_update_func)(core_instance);
    (core.core_destroy_func)(core_instance, false);

    Ok(())
}

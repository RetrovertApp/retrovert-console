use anyhow::Result;
use core_loader::Core;
use log::{debug, error, info, warn};

fn main() -> Result<()> {
    Core::init_logging()?;

    if let Err(e) = Core::init_data_directory() {
        error!("Unable to init directories: error {:?}", e);
        return Err(e);
    }

    let loaded_core = match Core::load_core() {
        Ok(lib) => lib,
        Err(e) => {
            error!("Unable to load corelib: {}", e);
            return Err(e);
        }
    };

    let core = match Core::new(&loaded_core) {
        Ok(c) => c,
        Err(e) => {
            error!("Unable to create core {}", e);
            return Err(e);
        }
    };

    let core_instance = (core.core_create_func)();
    (core.core_update_func)(core_instance);
    (core.core_destroy_func)(core_instance, false);

    Ok(())
}

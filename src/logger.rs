use std::{collections::HashMap, io};

use lum_log::{
    Builder, Config, defaults,
    log::{LevelFilter, SetLoggerError},
};

pub fn setup_logger() -> Result<(), SetLoggerError> {
    let mut colors = HashMap::new();
    colors.insert(LevelFilter::Info, "Green".into());
    colors.insert(LevelFilter::Error, "Red".into());
    colors.insert(LevelFilter::Warn, "Yellow".into());
    colors.insert(LevelFilter::Debug, "Purple".into());
    colors.insert(LevelFilter::Trace, "Blue".into());

    let config = Config {
        colors,
        min_log_level: LevelFilter::Info,
    };

    let module_levels = [];

    Builder::new(defaults::format())
        .config(&config)
        .chain(io::stdout())
        .is_debug_build(cfg!(debug_assertions))
        .module_levels(&module_levels)
        .apply()
}

use std::io::Write;

pub use log::{debug, error, info, trace, warn};

pub fn init() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {:.3} | {}",
                chrono::Local::now().format("%Y/%m/%d %H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .filter(None, log::LevelFilter::Debug)
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        init();
    }
}

use std::fmt::Display;
use std::io::{Write, stderr};
use std::sync::atomic::{AtomicBool, Ordering};

static ERROR_FLAG: AtomicBool = AtomicBool::new(false);

pub fn error_flag() -> bool {
    ERROR_FLAG.load(Ordering::Relaxed)
}

pub fn report<Msg: Display>(msg: Msg) {
    ERROR_FLAG.store(true, Ordering::Relaxed);

    let mut std_error = stderr();
    writeln!(std_error, "[ERROR] {msg}").expect("Can not report");
    std_error.flush().expect("Can not flush");
}

#[macro_export]
macro_rules! formatted_error {
    ($msg:expr) => {
        error_system::report(format!($msg))
    };
}

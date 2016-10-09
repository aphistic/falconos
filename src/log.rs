use spin::Mutex;

use serial;

pub static SERIAL_LOG: Mutex<serial::Serial> = Mutex::new(serial::Serial::new(0x3f8));

macro_rules! logln {
    ($fmt:expr) => (log!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (log!(concat!($fmt, "\n"), $($arg)*));
}
macro_rules! log {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        $crate::log::SERIAL_LOG.lock().write_fmt(format_args!($($arg)*)).unwrap();
    });
}
#[macro_export]
macro_rules! clog {
    ($lvl:expr, $($arg:tt)+) => ({
        log!(target: "agent", $lvl, $($arg)*);
    });
}

#[macro_export]
macro_rules! cerror {
    ($($arg:tt)*) => (
        clog!($crate::logger::Level::Error, $($arg)*)
    );
}

#[macro_export]
macro_rules! cwarn {
    ($($arg:tt)*) => (
        clog!($crate::logger::Level::Warn, $($arg)*)
    );
}

#[macro_export]
macro_rules! cinfo {
    ($($arg:tt)*) => (
        clog!($crate::logger::Level::Info, $($arg)*)
    );
}

#[macro_export]
macro_rules! cdebug {
    ($($arg:tt)*) => (
        clog!($crate::logger::Level::Debug, $($arg)*)
    );
}

#[macro_export]
macro_rules! ctrace {
    ($($arg:tt)*) => (
        clog!($crate::logger::Level::Trace, $($arg)*)
    );
}

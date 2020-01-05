#[macro_export]
macro_rules! clog {
    ($lvl:expr, $($arg:tt)+) => ({
        log::log!(target: "dashboard-server", $lvl, $($arg)*);
    });
}

#[macro_export]
macro_rules! cerror {
    ($($arg:tt)*) => (
        $crate::clog!($crate::logger::Level::Error, $($arg)*)
    );
}

#[macro_export]
macro_rules! cwarn {
    ($($arg:tt)*) => (
        $crate::clog!($crate::logger::Level::Warn, $($arg)*)
    );
}

#[macro_export]
macro_rules! cinfo {
    ($($arg:tt)*) => (
        $crate::clog!($crate::logger::Level::Info, $($arg)*)
    );
}

#[macro_export]
macro_rules! cdebug {
    ($($arg:tt)*) => (
        $crate::clog!($crate::logger::Level::Debug, $($arg)*)
    );
}

#[macro_export]
macro_rules! ctrace {
    ($($arg:tt)*) => (
        $crate::clog!($crate::logger::Level::Trace, $($arg)*)
    );
}

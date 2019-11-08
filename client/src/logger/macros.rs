#[macro_export]
macro_rules! log_target {
    (PROCESS) => {
        "client-process"
    };
    (MAIN) => {
        "client-main"
    };
    (WEB) => {
        "client-web"
    };
    (HARDWARE) => {
        "client-hardware"
    };
}

#[macro_export]
macro_rules! clog {
    ($target:ident, $lvl:expr, $($arg:tt)+) => ({
        log!(target: log_target!($target), $lvl, $($arg)*);
    });
}

#[macro_export]
macro_rules! cerror {
    ($target:ident, $($arg:tt)*) => (
        clog!($target, $crate::logger::Level::Error, $($arg)*)
    );
}

#[macro_export]
macro_rules! cwarn {
    ($target:ident, $($arg:tt)*) => (
        clog!($target, $crate::logger::Level::Warn, $($arg)*)
    );
}

#[macro_export]
macro_rules! cinfo {
    ($target:ident, $($arg:tt)*) => (
        clog!($target, $crate::logger::Level::Info, $($arg)*)
    );
}

#[macro_export]
macro_rules! cdebug {
    ($target:ident, $($arg:tt)*) => (
        clog!($target, $crate::logger::Level::Debug, $($arg)*)
    );
}

#[macro_export]
macro_rules! ctrace {
    ($target:ident, $($arg:tt)*) => (
        clog!($target, $crate::logger::Level::Trace, $($arg)*)
    );
}

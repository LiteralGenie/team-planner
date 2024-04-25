pub mod api;

macro_rules! log {
    ($($t:tt)*) => (crate::console::api::log(&format_args!($($t)*).to_string()))
}

pub(crate) use log;

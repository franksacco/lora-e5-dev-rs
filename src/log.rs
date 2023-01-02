#[cfg(debug_assertions)]
macro_rules! log {
    ($s:expr) => {
        cortex_m_semihosting::hprintln!($s)
    };
}

#[cfg(not(debug_assertions))]
macro_rules! log {
    ($s:expr) => {};
}

pub(crate) use log;

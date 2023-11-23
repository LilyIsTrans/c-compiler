#[cfg(not(unicode))]
pub type BasicText = [u8];

#[cfg(unicode)]
pub type BasicText = str;

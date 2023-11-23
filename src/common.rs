#[cfg(not(feature = "unicode"))]
pub type BasicText = [u8];

#[cfg(feature = "unicode")]
pub type BasicText = str;

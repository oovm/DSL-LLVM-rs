#[cfg(not(test))]
pub use self::derive::{Parser, Rule};
#[cfg(test)]
pub use self::result::{Parser, Rule};
#[allow(unused_attributes)]
mod derive;
#[allow(unused_attributes)]
mod result;

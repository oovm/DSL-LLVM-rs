#[cfg(not(test))]
pub use self::derive::{Parser, Rule};
#[cfg(test)]
pub use self::result::{Parser, Rule};

mod derive;
mod result;

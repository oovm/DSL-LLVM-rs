#[derive(Debug, Clone)]
pub enum AST {
    /// - `null`: It doesn't look like anything to me
    Null,
    /// - `undefined`: It just like a dream
    Undefined,

    /// - `String`
    String(),
    /// - `Boolean`
    Boolean(),
    /// - `Number`
    Number(),
    /// - `Integer`
    Integer(),
}

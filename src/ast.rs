#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    /// - `null`: It doesn't look like anything to me
    Null,
    /// - `undefined`: It just like a dream
    Undefined,

    /// - `String`: A rust standard string
    String(String),
    /// - `Boolean`: true of false, this is a problem
    Boolean(bool),
    /// `Number`: A 64-bit floating point number, just like js
    Decimal(f64),
    /// - `Integer` : A 64-bit integer, not like js
    Integer(i64),

    /// - `Symbol`
    Symbol(String),
    /// - `Function`: ???
    Function(),

    Stack(Vec<AST>),
}

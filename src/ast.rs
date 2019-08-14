#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    /// - `null`: It doesn't look like anything to me
    Null,
    /// - `undefined`: It just likes a dream
    Undefined,

    /// - `String`: A rust standard string
    String(String),
    /// - `Boolean`: true of false, this is a problem
    Boolean(bool),
    /// `Number`: A 64-bit floating point number, just like js
    Decimal(f64),
    /// - `Integer` : A 64-bit integer, not like js
    Integer(i64),

    /// - `Symbol`: A string used as id
    Symbol(String),
    /// - `Function`: ???
    Function(),

    Infix(String, Box<AST>, Box<AST>),
    Prefix(String, Box<AST>),
    Postfix(String, Box<AST>),

    Array(Vec<AST>),
    Hashmap(Vec<(AST, AST)>),

    /// - `Stack`: ???
    Stack(Vec<AST>), // not slice!
}

impl From<&str> for AST {
    fn from(s: &str) -> Self {
        AST::String(s.to_string())
    }
}

impl From<i64> for AST {
    fn from(i: i64) -> Self {
        AST::Integer(i)
    }
}

impl From<f64> for AST {
    fn from(i: f64) -> Self {
        AST::Decimal(i)
    }
}

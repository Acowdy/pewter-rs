/// A compilation unit, usually a single file.
#[derive(Debug)]
pub struct Compunit {
    pub name: String,
    pub defs: Vec<Def>,
}

/// A top level definition.
#[derive(Debug)]
pub struct Def(pub String, pub Literal);

/// A literal.
#[derive(Debug)]
pub enum Literal {
    Int(i32),
}

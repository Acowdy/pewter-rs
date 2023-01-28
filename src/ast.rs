#[derive(Debug)]
pub struct Decl(pub String, pub i32);

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub decls: Vec<Decl>,
}

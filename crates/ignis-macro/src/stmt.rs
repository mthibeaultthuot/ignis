use syn::Stmt;

pub struct Statement {
    stmts: Vec<Stmt>,
}

impl Statement {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self { stmts }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Stmt> {
        self.stmts.iter()
    }
}

use syn::ItemFn;

use crate::stmt::Statement;

pub struct Parser {
    stmt: Statement,
}

impl Parser {
    pub fn from_item(func: ItemFn) -> Self {
        //for blck in func.block.stmts {
        //    println!("{:?}", blck);
        //    println!("_________");
        //}
        Self {
            stmt: Statement::new(func.block.stmts),
        }
    }

    pub fn parse(&self) {
        self.stmt.iter().for_each(|item| println!("{:?}", item));
    }
}

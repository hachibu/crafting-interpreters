use syntax::*;

#[derive(Clone, Copy, Debug)]
pub struct SourceMap {
    pub offset: usize
}

impl SourceMap {
    pub fn new(offset: usize) -> SourceMap {
        SourceMap { offset }
    }
}

pub enum Stmt {
    Expr(Box<Expr>, SourceMap),
    Print(Box<Expr>, SourceMap),
    Var(String, Option<Box<Expr>>, SourceMap)
}

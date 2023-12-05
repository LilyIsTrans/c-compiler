use crate::tokens::*;

struct Block<'a>(Vec<Statement<'a>>);

enum Expression<'a> {
    Constant(Literal<'a>),
}

enum Statement<'a> {
    Return(Expression<'a>),
}

struct FunctionDeclaration<'a> {
    name: Identifier<'a>,
    body: Block<'a>,
}

struct Program<'a> {
    funcs: Vec<FunctionDeclaration<'a>>,
}

pub type Integer = i64;
pub type Label = Integer;
pub type Name = String;

/* Enums */

#[derive(Debug, Clone)]
pub enum Block {
    AssignmentStmt(AssignmentStmt),
    SkipStmt(SkipStmt),
    Condition(Condition),
}

#[derive(Debug, Clone)]
pub enum Statement {
    AssignmentStmt(AssignmentStmt),
    SkipStmt(SkipStmt),
    SequenceStmt(SequenceStmt),
    IfElseStmt(IfElseStmt),
    WhileStmt(WhileStmt),
}

#[derive(Debug, Clone)]
pub enum Expression {
    // Arithmetic expressions
    VarExp(VarExp),
    NumExp(NumExp),
    AddExp(AddExp),
    SubExp(SubExp),
    MulExp(MulExp),
    DivExp(DivExp),

    // Boolean expressions
    CTrue(),
    CFalse(),
    NotExp(NotExp),
    AndExp(AndExp),
    OrExp(OrExp),

    EqExp(EqExp),
    GTExp(GTExp),
    LTExp(LTExp),
    GEqExp(GEqExp),
    LEqExp(LEqExp),
}
#[derive(Debug, Clone)]
pub enum ArithmeticExpression {
    VarExp(VarExp),
    NumExp(NumExp),
    AddExp(AddExp),
    SubExp(SubExp),
    MulExp(MulExp),
    DivExp(DivExp),
}

#[derive(Debug, Clone)]
pub enum BooleanExpression {
    CTrue(CTrue),
    CFalse(CFalse),
    NotExp(NotExp),
    AndExp(AndExp),
    OrExp(OrExp),

    EqExp(EqExp),
    GTExp(GTExp),
    LTExp(LTExp),
    GEqExp(GEqExp),
    LEqExp(LEqExp),
}

#[derive(Debug, Clone)] /* Statements */
pub struct AssignmentStmt {
    pub name: Name,
    pub exp: Box<Expression>,
    pub label: Label,
}

#[derive(Debug, Clone)]
pub struct SkipStmt {
    pub label: Label,
}

#[derive(Debug, Clone)]
pub struct SequenceStmt {
    pub s1: Box<Statement>,
    pub s2: Box<Statement>,
}

#[derive(Debug, Clone)]
pub struct IfElseStmt {
    pub condition: Condition,
    pub then_stmt: Box<Statement>,
    pub else_stmt: Box<Statement>,
}

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub condition: Condition,
    pub stmt: Box<Statement>,
}

#[derive(Debug, Clone)]
pub struct Condition {
    pub exp: BooleanExpression,
    pub label: Label,
}

/* Arithmetic expressio
ns */
#[derive(Debug, Clone)]
pub struct VarExp {
    pub name: Name,
}

#[derive(Debug, Clone)]
pub struct NumExp {
    pub value: Integer,
}

#[derive(Debug, Clone)]
pub struct AddExp {
    pub left: Box<ArithmeticExpression>,
    pub right: Box<ArithmeticExpression>,
}

#[derive(Debug, Clone)]
pub struct SubExp {
    pub left: Box<ArithmeticExpression>,
    pub right: Box<ArithmeticExpression>,
}

#[derive(Debug, Clone)]
pub struct MulExp {
    pub left: Box<ArithmeticExpression>,
    pub right: Box<ArithmeticExpression>,
}

#[derive(Debug, Clone)]
pub struct DivExp {
    pub left: Box<ArithmeticExpression>,
    pub right: Box<ArithmeticExpression>,
}

/* Boolean expressions */

#[derive(Debug, Clone)]
pub struct CTrue {}

#[derive(Debug, Clone)]
pub struct CFalse {}

#[derive(Debug, Clone)]
pub struct NotExp {
    pub exp: Box<BooleanExpression>,
}

#[derive(Debug, Clone)]
pub struct AndExp {
    pub left: Box<BooleanExpression>,
    pub right: Box<BooleanExpression>,
}

#[derive(Debug, Clone)]
pub struct OrExp {
    pub left: Box<BooleanExpression>,
    pub right: Box<BooleanExpression>,
}

/* Relational expressions */

#[derive(Debug, Clone)]
pub struct EqExp {
    pub left: ArithmeticExpression,
    pub right: ArithmeticExpression,
}

#[derive(Debug, Clone)]
pub struct GTExp {
    pub left: ArithmeticExpression,
    pub right: ArithmeticExpression,
}

#[derive(Debug, Clone)]
pub struct LTExp {
    pub left: ArithmeticExpression,
    pub right: ArithmeticExpression,
}

#[derive(Debug, Clone)]
pub struct GEqExp {
    pub left: ArithmeticExpression,
    pub right: ArithmeticExpression,
}

#[derive(Debug, Clone)]
pub struct LEqExp {
    pub left: ArithmeticExpression,
    pub right: ArithmeticExpression,
}

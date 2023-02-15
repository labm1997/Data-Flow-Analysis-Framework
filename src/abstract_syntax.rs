pub type Integer = i64;
pub type Label = Integer;
pub type Name = String;
pub type Program = Statement;
pub const UNDEF: Label = -1;

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
    ArithmeticExpression(Box<ArithmeticExpression>),
    BooleanExpression(Box<BooleanExpression>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    pub exp: Box<BooleanExpression>,
    pub label: Label,
}

/* Arithmetic expressio
ns */
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VarExp {
    pub name: Name,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NumExp {
    pub value: Integer,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AddExp {
    pub left: Box<ArithmeticExpression>,
    pub right: Box<ArithmeticExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubExp {
    pub left: Box<ArithmeticExpression>,
    pub right: Box<ArithmeticExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MulExp {
    pub left: Box<ArithmeticExpression>,
    pub right: Box<ArithmeticExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    pub left: Box<ArithmeticExpression>,
    pub right: Box<ArithmeticExpression>,
}

#[derive(Debug, Clone)]
pub struct GTExp {
    pub left: Box<ArithmeticExpression>,
    pub right: Box<ArithmeticExpression>,
}

#[derive(Debug, Clone)]
pub struct LTExp {
    pub left: Box<ArithmeticExpression>,
    pub right: Box<ArithmeticExpression>,
}

#[derive(Debug, Clone)]
pub struct GEqExp {
    pub left: Box<ArithmeticExpression>,
    pub right: Box<ArithmeticExpression>,
}

#[derive(Debug, Clone)]
pub struct LEqExp {
    pub left: Box<ArithmeticExpression>,
    pub right: Box<ArithmeticExpression>,
}

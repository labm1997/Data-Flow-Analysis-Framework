use crate::abstract_syntax::{Block, Statement};

pub fn blocks(stmt: Box<Statement>) -> Vec<Block> {
    return match *stmt {
        Statement::AssignmentStmt(data) => Vec::from([Block::AssignmentStmt(data)]),
        Statement::SkipStmt(data) => Vec::from([Block::SkipStmt(data)]),
        Statement::SequenceStmt(data) => [blocks(data.s1), blocks(data.s2)].concat(),
        Statement::IfElseStmt(data) => [
            Vec::from([Block::Condition(data.condition)]),
            blocks(data.then_stmt),
            blocks(data.else_stmt),
        ]
        .concat(),
        Statement::WhileStmt(data) => [
            Vec::from([Block::Condition(data.condition)]),
            blocks(data.stmt),
        ]
        .concat(),
    };
}

// impl Statement for AssignmentStmt {
//     fn get_blocks(&self) -> Vec<Box<dyn Block>> {
//         let a = Vec::from([Box::new(AssignmentStmt {
//             name: self.name,
//             exp: Box::new(self.exp),
//             label: self.label,
//         })]);
//         return Vec::new();
//     }
// }
// impl Statement for SkipStmt {
//     fn get_blocks(&self) -> Vec<Box<dyn Block>> {
//         return Vec::new();
//     }
// }
// impl Statement for SequenceStmt {
//     fn get_blocks(&self) -> Vec<Box<dyn Block>> {
//         let mut blocks = Vec::new();
//         for block in self.s1.get_blocks() {
//             blocks.push(block);
//         }
//         for block in self.s2.get_blocks() {
//             blocks.push(block);
//         }
//         return blocks;
//     }
// }
// impl Statement for IfElseStmt {
//     fn get_blocks(&self) -> Vec<Box<dyn Block>> {
//         return Vec::new();
//     }
// }
// impl Statement for WhileStmt {
//     fn get_blocks(&self) -> Vec<Box<dyn Block>> {
//         return Vec::new();
//     }
// }

// fn blocks(stmt: Statement): Set[Block] = stmt match {
//   case AssignmentStmt(name, exp, label) => Set(AssignmentStmt(name, exp, label))
//   case SkipStmt(label) => Set(SkipStmt(label))
//   case SequenceStmt(s1, s2) => blocks(s1) union blocks(s2)
//   case IfElseStmt(condition, thenStmt, elseStmt) => Set(condition) union blocks(thenStmt) union blocks(elseStmt)
//   case WhileStmt(condition, stmt) => Set(condition) union blocks (stmt)
// }

// def assignments(stmt: Statement): Set[AssignmentStmt] = stmt match {
//   case AssignmentStmt(name, exp, label) => Set(AssignmentStmt(name, exp, label))
//   case SkipStmt(label) => Set()
//   case SequenceStmt(s1, s2) => assignments(s1) union assignments(s2)
//   case IfElseStmt(condition, thenStmt, elseStmt) => assignments(thenStmt) union assignments(elseStmt)
//   case WhileStmt(condition, stmt) => assignments(stmt)
// }

// /* computes the free variables of a program */
// def fv(exp: BooleanExpression): Set[Name] = exp match {
//   case CTrue => Set()
//   case CFalse => Set()
//   case NotExp(exp) => fv(exp)
//   case AndExp(left, right) => fv(left) union fv(right)
//   case OrExp(left, right) => fv(left) union fv(right)
//   case EqExp(left, right) => fv(left) union fv(right)
//   case GTExp(left, right) => fv(left) union fv(right)
//   case LTExp(left, right) => fv(left) union fv(right)
//   case GEqExp(left, right) => fv(left) union fv(right)
//   case LEqExp(left, right) => fv(left) union fv(right)
// }

// def fv(exp: ArithmeticExpression): Set[Name] = exp match {
//   case VarExp(name) => Set(name)
//   case NumExp(value) => Set()
//   case AddExp(left, right) => fv(left) union fv(right)
//   case SubExp(left, right) => fv(left) union fv(right)
//   case MulExp(left, right) => fv(left) union fv(right)
//   case DivExp(left, right) => fv(left) union fv(right)
// }

// def fv(stmt: Statement): Set[Name] = stmt match {
//   case AssignmentStmt(name, exp, label) => fv(exp)
//   case SkipStmt(label) => Set()
//   case SequenceStmt(s1, s2) => fv(s1) union fv(s2)
//   case IfElseStmt(Condition(c, _), thenStmt, elseStmt) => fv(c) union fv(thenStmt) union fv(elseStmt)
//   case WhileStmt(Condition(c, _), stmt) => fv(c) union fv(stmt)
// }

// def complexExpressions(exp: BooleanExpression): Set[ArithmeticExpression] = exp match {
//   case CTrue => Set()
//   case CFalse => Set()
//   case NotExp(exp) => complexExpressions(exp)
//   case AndExp(left, right) => complexExpressions(left) union complexExpressions(right)
//   case OrExp(left, right) => complexExpressions(left) union complexExpressions(right)
//   case EqExp(left, right) => complexExpressions(left) union complexExpressions(right)
//   case GTExp(left, right) => complexExpressions(left) union complexExpressions(right)
//   case LTExp(left, right) => complexExpressions(left) union complexExpressions(right)
//   case GEqExp(left, right) => complexExpressions(left) union complexExpressions(right)
//   case LEqExp(left, right) => complexExpressions(left) union complexExpressions(right)
// }
// def complexExpressions(exp: ArithmeticExpression): Set[ArithmeticExpression] = exp match {
//   case VarExp(name) => Set()
//   case NumExp(value) => Set()
//   case AddExp(left, right) => Set(AddExp(left, right)) union complexExpressions(left) union complexExpressions(right)
//   case SubExp(left, right) => Set(AddExp(left, right)) union complexExpressions(left) union complexExpressions(right)
//   case MulExp(left, right) => Set(AddExp(left, right)) union complexExpressions(left) union complexExpressions(right)
//   case DivExp(left, right) => Set(AddExp(left, right)) union complexExpressions(left) union complexExpressions(right)
// }

// def complexExpressions(c: Condition): Set[ArithmeticExpression] = complexExpressions(c.exp)

// def complexExpressions(stmt: Statement): Set[ArithmeticExpression] = stmt match {
//   case AssignmentStmt(name, exp, label) => complexExpressions(exp)
//   case SkipStmt(label) => Set()
//   case SequenceStmt(s1, s2) => complexExpressions(s1) union complexExpressions(s2)
//   case IfElseStmt(condition, s1, s2) => complexExpressions(condition) union complexExpressions(s1) union complexExpressions(s2)
//   case WhileStmt(condition, stmt) => complexExpressions(condition) union complexExpressions(stmt)
// }

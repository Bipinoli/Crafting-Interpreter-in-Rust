use crate::parser::expression::ExpressionVisitor;
use crate::parser::expression::{Binary, Grouping, Literal, Unary};

pub struct AstPrinterVisitor {}

impl ExpressionVisitor for AstPrinterVisitor {
    fn for_unary(&self, visitable: &Unary) {}
    fn for_binary(&self, visitable: &Binary) {}
    fn for_literal(&self, visitable: &Literal) {}
    fn for_grouping(&self, visitable: &Grouping) {}
}

use std::collections::VecDeque;

use crate::expr::Expr;

#[derive(Debug, PartialEq)]
pub(crate) struct Monkey {
    pub id: usize,
    pub items: VecDeque<u64>,
    pub operation: Expr,
    pub test_divisible_by: u64,
    pub if_true: u64,
    pub if_false: u64,
}

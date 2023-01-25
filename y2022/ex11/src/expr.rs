#[derive(Debug, PartialEq)]
pub(crate) struct Const(pub(crate) u64);

#[derive(Debug, PartialEq)]
pub(crate) enum Var {
    Old,
    New,
}

#[derive(Debug, PartialEq)]
pub(crate) enum VarConst {
    Var(Var),
    Const(Const),
}

#[derive(Debug, PartialEq)]
pub(crate) enum Op {
    Plus,
    Mult,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Expr {
    lhs: VarConst,
    op: Op,
    rhs: VarConst,
}

impl Expr {
    pub(crate) fn new(lhs: VarConst, op: Op, rhs: VarConst) -> Self {
        Self { lhs, op, rhs }
    }

    pub(crate) fn eval(&self, old: u64, new: u64) -> u64 {
        let lhs = match self.lhs {
            VarConst::Var(Var::Old) => old,
            VarConst::Var(Var::New) => new,
            VarConst::Const(Const(c)) => c,
        };
        let rhs = match self.rhs {
            VarConst::Var(Var::Old) => old,
            VarConst::Var(Var::New) => new,
            VarConst::Const(Const(c)) => c,
        };
        match self.op {
            Op::Plus => lhs + rhs,
            Op::Mult => lhs * rhs,
        }
    }
}

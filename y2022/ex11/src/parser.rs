use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map_res, opt},
    multi::separated_list1,
    IResult,
};

use crate::{
    expr::{Const, Expr, Op, Var, VarConst},
    monkey::Monkey,
};

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse::<usize>().map(|u| u as u64))(input)
}

fn parse_monkey_number(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("Monkey ")(input)?;
    let (input, id) = parse_usize(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, id))
}

fn parse_starting_items(input: &str) -> IResult<&str, VecDeque<u64>> {
    let (input, _) = tag("  Starting items: ")(input)?;
    let (input, items) = separated_list1(tag(", "), parse_u64)(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, VecDeque::from(items)))
}

fn parse_var(input: &str) -> IResult<&str, VarConst> {
    let (input, var) = alt((tag("old"), tag("new")))(input)?;
    match var {
        "old" => Ok((input, VarConst::Var(Var::Old))),
        "new" => Ok((input, VarConst::Var(Var::New))),
        _ => unreachable!(),
    }
}

fn parse_const(input: &str) -> IResult<&str, VarConst> {
    let (input, value) = parse_u64(input)?;
    Ok((input, VarConst::Const(Const(value))))
}

fn parse_var_const(input: &str) -> IResult<&str, VarConst> {
    let (input, var) = alt((parse_var, parse_const))(input)?;
    Ok((input, var))
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    let (input, op) = alt((tag("+"), tag("*")))(input)?;
    match op {
        "+" => Ok((input, Op::Plus)),
        "*" => Ok((input, Op::Mult)),
        _ => unreachable!(),
    }
}

fn parse_operation(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("  Operation: new = ")(input)?;
    let (input, lhs) = parse_var_const(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, op) = parse_op(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, rhs) = parse_var_const(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, Expr::new(lhs, op, rhs)))
}

fn parse_test(input: &str) -> IResult<&str, u64> {
    let (input, _) = tag("  Test: divisible by ")(input)?;
    let (input, value) = parse_u64(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, value))
}

fn parse_if_true(input: &str) -> IResult<&str, u64> {
    let (input, _) = tag("    If true: throw to monkey ")(input)?;
    let (input, value) = parse_u64(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, value))
}

fn parse_if_false(input: &str) -> IResult<&str, u64> {
    let (input, _) = tag("    If false: throw to monkey ")(input)?;
    let (input, value) = parse_u64(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, value))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, id) = parse_monkey_number(input)?;
    let (input, starting_items) = parse_starting_items(input)?;
    let (input, operation) = parse_operation(input)?;
    let (input, test_divisible_by) = parse_test(input)?;
    let (input, if_true) = parse_if_true(input)?;
    let (input, if_false) = parse_if_false(input)?;
    Ok((
        input,
        Monkey {
            id,
            items: starting_items,
            operation,
            test_divisible_by,
            if_true,
            if_false,
        },
    ))
}

pub(crate) fn parse_input(input: &str) -> impl Iterator<Item = Monkey> + '_ {
    input.split("\n\n").map(|s| parse_monkey(s).unwrap().1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_monkey_number() {
        let input = "Monkey 0:\n";
        let (_, id) = parse_monkey_number(input).unwrap();
        assert_eq!(id, 0);
    }

    #[test]
    fn test_parse_starting_items() {
        let input = "  Starting items: 79, 98\n";
        let (_, id) = parse_starting_items(input).unwrap();
        assert_eq!(id, vec![79, 98]);
    }

    #[test]
    fn test_parse_operation() {
        let input = "  Operation: new = old * 19\n";
        let (_, op) = parse_operation(input).unwrap();
        assert_eq!(
            op,
            Expr::new(
                VarConst::Var(Var::Old),
                Op::Mult,
                VarConst::Const(Const(19))
            )
        );
    }

    #[test]
    fn test_parse_test() {
        let input = "  Test: divisible by 23\n";
        let (_, divisible_by) = parse_test(input).unwrap();
        assert_eq!(divisible_by, 23);
    }

    #[test]
    fn test_parse_if_true() {
        let input = "    If true: throw to monkey 2\n";
        let (_, id) = parse_if_true(input).unwrap();
        assert_eq!(id, 2);
    }

    #[test]
    fn test_parse_if_false() {
        let input = "    If false: throw to monkey 3";
        let (_, id) = parse_if_false(input).unwrap();
        assert_eq!(id, 3);
    }

    #[test]
    fn test_parse_monkey() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
";

        let (_, monkey) = parse_monkey(input).unwrap();
        assert_eq!(
            monkey,
            Monkey {
                id: 0,
                items: VecDeque::from(vec![79, 98]),
                operation: Expr::new(
                    VarConst::Var(Var::Old),
                    Op::Mult,
                    VarConst::Const(Const(19))
                ),
                test_divisible_by: 23,
                if_true: 2,
                if_false: 3,
            }
        );
    }

    #[test]
    fn test_parse_input() {
        let input = "Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3";

        let monkeys: Vec<Monkey> = parse_input(input).collect();
        assert_eq!(
            monkeys,
            vec![
                Monkey {
                    id: 1,
                    items: VecDeque::from(vec![54, 65, 75, 74]),
                    operation: Expr::new(
                        VarConst::Var(Var::Old),
                        Op::Plus,
                        VarConst::Const(Const(6))
                    ),
                    test_divisible_by: 19,
                    if_true: 2,
                    if_false: 0,
                },
                Monkey {
                    id: 2,
                    items: VecDeque::from(vec![79, 60, 97]),
                    operation: Expr::new(
                        VarConst::Var(Var::Old),
                        Op::Mult,
                        VarConst::Var(Var::Old)
                    ),
                    test_divisible_by: 13,
                    if_true: 1,
                    if_false: 3,
                },
            ]
        );
    }
}

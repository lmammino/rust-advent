use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::{alpha1, u64},
    combinator::{complete, opt},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt::Display,
    ops::RangeInclusive,
};

const MIN_RANGE: u64 = 1;
const MAX_RANGE: u64 = 4000;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Condition {
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

impl Condition {
    fn opposite(&self) -> Self {
        match self {
            // > -> <=
            Self::Greater => Self::LessEqual,
            // < -> >=
            Self::Less => Self::GreaterEqual,
            // >= -> <
            Self::GreaterEqual => Self::Less,
            // <= -> >
            Self::LessEqual => Self::Greater,
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Greater => write!(f, ">"),
            Self::Less => write!(f, "<"),
            Self::GreaterEqual => write!(f, ">="),
            Self::LessEqual => write!(f, "<="),
        }
    }
}

impl From<char> for Condition {
    fn from(c: char) -> Self {
        match c {
            '>' => Self::Greater,
            '<' => Self::Less,
            _ => panic!("Invalid condition"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr<'a> {
    variable: &'a str,
    condition: Condition,
    value: u64,
}

impl<'a> Expr<'a> {
    pub fn reverse(&self) -> Self {
        Self {
            variable: self.variable,
            condition: self.condition.opposite(),
            value: self.value,
        }
    }
}

impl std::fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.variable, self.condition, self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target<'a> {
    Accepted,
    Rejected,
    Label(&'a str),
}

impl<'a> Target<'a> {
    fn new(label: &'a str) -> Self {
        match label {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            _ => Self::Label(label),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule<'a> {
    WithExpr(Expr<'a>, Target<'a>),
    NoExpr(Target<'a>),
}

impl<'a> Rule<'a> {
    pub fn eval(&self, variables: &Variables<'a>) -> Option<Target<'a>> {
        match self {
            Self::WithExpr(expr, target) => {
                let value = variables.0.get(expr.variable).unwrap();
                match expr.condition {
                    Condition::Greater => {
                        if value > &expr.value {
                            Some(target.clone())
                        } else {
                            None
                        }
                    }
                    Condition::Less => {
                        if value < &expr.value {
                            Some(target.clone())
                        } else {
                            None
                        }
                    }
                    _ => panic!("Invalid condition. This shouldn't happen in part 1"),
                }
            }
            Self::NoExpr(target) => Some(target.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workflow<'a> {
    pub label: &'a str,
    pub rules: Vec<Rule<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variables<'a>(pub HashMap<&'a str, u64>);

fn parse_variable(input: &str) -> IResult<&str, &str> {
    let (input, variable) = alt((tag("x"), tag("m"), tag("a"), tag("s")))(input)?;
    Ok((input, variable))
}

fn parse_label(input: &str) -> IResult<&str, &str> {
    let (input, label) = alpha1(input)?;
    Ok((input, label))
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    let (input, variable) = parse_variable(input)?;
    let (input, condition) = alt((char('>'), char('<')))(input)?;
    let (input, value) = u64(input)?;
    Ok((
        input,
        Expr {
            variable,
            condition: condition.into(),
            value,
        },
    ))
}

fn parse_target(input: &str) -> IResult<&str, Target<'_>> {
    let (input, target) = alt((tag("R"), tag("A"), alpha1))(input)?;
    Ok((input, Target::new(target)))
}

fn parse_rule(input: &str) -> IResult<&str, Rule<'_>> {
    // "a<2006:qkq" (rule with expr) or "rfg" (rule no expr)
    let (input, expr) = opt(tuple((parse_expr, tag(":"))))(input)?;
    let (input, target) = parse_target(input)?;
    match expr {
        Some((expr, _)) => Ok((input, Rule::WithExpr(expr, target))),
        _ => Ok((input, Rule::NoExpr(target))),
    }
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow<'_>> {
    // px{a<2006:qkq,m>2090:A,rfg}
    let (input, label) = parse_label(input)?;
    let (input, (_, rules, _)) = complete(tuple((
        char('{'),
        separated_list1(char(','), parse_rule),
        char('}'),
    )))(input)?;
    Ok((input, Workflow { label, rules }))
}

fn parse_variable_assignment(input: &str) -> IResult<&str, (&str, u64)> {
    let (input, variable) = parse_variable(input)?;
    let (input, _) = char('=')(input)?;
    let (input, value) = u64(input)?;
    Ok((input, (variable, value)))
}

fn parse_variables(input: &str) -> IResult<&str, Variables<'_>> {
    // {x=489,m=624,a=126,s=557}
    let (input, (_, assignments, _)) = tuple((
        char('{'),
        separated_list1(char(','), parse_variable_assignment),
        char('}'),
    ))(input)?;
    let variables: HashMap<&str, u64> = assignments.into_iter().collect();
    Ok((input, Variables(variables)))
}

pub fn parse_input(input: &str) -> (HashMap<&str, Workflow>, impl Iterator<Item = Variables<'_>>) {
    let (raw_workflows, raw_variables) = input.split_once("\n\n").unwrap();
    let workflows: HashMap<&str, Workflow> = raw_workflows
        .lines()
        .map(|line| parse_workflow(line).unwrap().1)
        .map(|workflow| (workflow.label, workflow))
        .collect();
    let variables = raw_variables
        .lines()
        .map(|line| parse_variables(line).unwrap().1);

    (workflows, variables)
}

fn range_intersect(
    r1: &RangeInclusive<u64>,
    r2: &RangeInclusive<u64>,
) -> Option<RangeInclusive<u64>> {
    let start = max(r1.start(), r2.start());
    let end = min(r1.end(), r2.end());
    if start > end {
        None
    } else {
        Some(*start..=*end)
    }
}

pub fn num_solutions(exprs: &Vec<Expr>) -> u64 {
    let mut min_max_map: HashMap<&str, RangeInclusive<u64>> = vec![
        ("x", MIN_RANGE..=MAX_RANGE),
        ("m", MIN_RANGE..=MAX_RANGE),
        ("a", MIN_RANGE..=MAX_RANGE),
        ("s", MIN_RANGE..=MAX_RANGE),
    ]
    .into_iter()
    .collect();

    for expr in exprs {
        let range = min_max_map.get_mut(expr.variable).unwrap();
        let new_range = match expr.condition {
            Condition::Greater => (expr.value + 1)..=MAX_RANGE,
            Condition::GreaterEqual => expr.value..=MAX_RANGE,
            Condition::Less => MIN_RANGE..=(expr.value - 1),
            Condition::LessEqual => MIN_RANGE..=expr.value,
        };
        if let Some(intersection) = range_intersect(range, &new_range) {
            *range = intersection; // reduce the current range to the intersection
        } else {
            return 0; // no solutions
        }
    }

    min_max_map
        .values()
        .map(|range| range.clone().count() as u64)
        .product()
}

#[cfg(test)]
mod test {
    use super::*;
    const EXAMPLE_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_parse_workflow() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}";
        let (_, workflow) = parse_workflow(input).unwrap();
        assert_eq!(
            workflow,
            Workflow {
                label: "px",
                rules: vec![
                    Rule::WithExpr(
                        Expr {
                            variable: "a",
                            condition: Condition::Less,
                            value: 2006
                        },
                        Target::Label("qkq")
                    ),
                    Rule::WithExpr(
                        Expr {
                            variable: "m",
                            condition: Condition::Greater,
                            value: 2090
                        },
                        Target::Accepted
                    ),
                    Rule::NoExpr(Target::Label("rfg"))
                ]
            }
        )
    }

    #[test]
    fn test_parse_input() {
        let (workflows, variables) = parse_input(EXAMPLE_INPUT);
        assert_eq!(workflows.len(), 11);
        assert_eq!(variables.count(), 5);
    }

    #[test]
    fn test_num_solutions() {
        use Condition::*;
        // s>=1351 s<=2770 m<1801 m>838
        // -> x=[1..4000], m=[839..1800], a=[1..4000], s=[1351..2770]
        // -> 4000 * 962 * 4000 * 1420 = 21_856_640_000_000
        let exprs = vec![
            Expr {
                variable: "s",
                condition: GreaterEqual,
                value: 1351,
            },
            Expr {
                variable: "s",
                condition: LessEqual,
                value: 2770,
            },
            Expr {
                variable: "m",
                condition: Less,
                value: 1801,
            },
            Expr {
                variable: "m",
                condition: Greater,
                value: 838,
            },
        ];
        assert_eq!(num_solutions(&exprs), 21_856_640_000_000);
    }
}

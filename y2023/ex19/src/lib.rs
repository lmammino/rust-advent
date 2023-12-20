use models::{num_solutions, parse_input, Expr, Rule, Target};
mod models;

pub fn part1(input: &str) -> u64 {
    let (workflows, variables) = parse_input(input);

    variables
        .filter(|variables| {
            let initial_workflow = workflows.get("in").unwrap();
            let mut current_workflow = initial_workflow;
            'apply_workflows: loop {
                for rule in current_workflow.rules.iter() {
                    let target = rule.eval(variables);
                    match target {
                        Some(Target::Label(label)) => {
                            current_workflow = workflows.get(label).unwrap();
                            continue 'apply_workflows;
                        }
                        Some(Target::Accepted) => return true,
                        Some(Target::Rejected) => return false,
                        None => continue,
                    }
                }
            }
        })
        .map(|variables| variables.0.values().sum::<u64>())
        .sum()
}

struct WorkflowTraversalState<'a> {
    current_workflow: &'a str,
    current_rule: usize,
    expressions: Vec<Expr<'a>>,
}

pub fn part2(input: &str) -> u64 {
    // IDEA traverse the workflow as a tree and maps out all the paths that lead to an accepted.
    // In every step we accumulate expressions. All the accepted paths will be later evaluated.
    let (workflows, _) = parse_input(input);

    let mut queue: Vec<WorkflowTraversalState> = vec![WorkflowTraversalState {
        current_workflow: "in",
        current_rule: 0,
        expressions: vec![],
    }];

    let mut accepted_paths: Vec<Vec<Expr<'_>>> = vec![];
    while let Some(state) = queue.pop() {
        let current_workflow = workflows.get(state.current_workflow).unwrap();
        let current_rule = current_workflow.rules.get(state.current_rule).unwrap();
        match current_rule {
            Rule::WithExpr(expr, target) => {
                match target {
                    Target::Accepted => {
                        // found a leaf node. Push the current expr to the stack of expressions and save the current path.
                        let mut expressions = state.expressions.clone();
                        expressions.push(expr.clone());
                        accepted_paths.push(expressions);
                    }
                    Target::Rejected => {
                        // invalid path, do nothing
                    }
                    Target::Label(label) => {
                        // intermediate node, push the current expr to the stack of expressions and continue traversing
                        let mut expressions = state.expressions.clone();
                        expressions.push(expr.clone());
                        queue.push(WorkflowTraversalState {
                            current_workflow: label,
                            current_rule: 0,
                            expressions,
                        });
                    }
                }

                // check if there is a following rule in the current workflow
                if let Some(_rule) = current_workflow.rules.get(state.current_rule + 1) {
                    // invert the current expression and create a new state for every rule
                    let inverted_expr = expr.reverse();
                    let mut expressions = state.expressions.clone();
                    expressions.push(inverted_expr);
                    queue.push(WorkflowTraversalState {
                        current_workflow: state.current_workflow,
                        current_rule: state.current_rule + 1,
                        expressions,
                    });
                }
            }
            Rule::NoExpr(target) => {
                match target {
                    Target::Accepted => {
                        // found a leaf node. Save the current path.
                        accepted_paths.push(state.expressions.clone());
                    }
                    Target::Rejected => {
                        // invalid path, do nothing
                    }
                    Target::Label(label) => {
                        // intermediate node, continue traversing
                        queue.push(WorkflowTraversalState {
                            current_workflow: label,
                            current_rule: 0,
                            expressions: state.expressions.clone(),
                        });
                    }
                }
            }
        }
    }

    accepted_paths.iter().map(num_solutions).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");
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
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 19114);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 330820);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 167409079868000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 123972546935551);
    }
}

use std::collections::HashMap;

use itertools::Itertools;

pub fn solve(input: &str) -> usize {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let mut workflows: HashMap<&str, Vec<Rule>> = workflows
        .lines()
        .map(|line| {
            let (label, rules) = line.split_once('{').unwrap();
            let rules = rules[..rules.len() - 1]
                .split(',')
                .map(|rule| {
                    if !rule.contains(':') {
                        Rule {
                            cond: Condition::Pass,
                            next: Next::from(rule),
                        }
                    } else {
                        let (cond, next) = rule.split_once(':').unwrap();
                        Rule {
                            cond: Condition::from(cond),
                            next: Next::from(next),
                        }
                    }
                })
                .collect();
            (label, rules)
        })
        .collect();

    while collapse_workflows(&mut workflows) > 0 {}

    parts
        .lines()
        .map(Part::from)
        .filter(|part| {
            matches!(
                qualify(part, workflows.get("in").unwrap().iter(), &workflows),
                Next::Accepted
            )
        })
        .fold(0, |acc, part| {
            acc + part.x as usize + part.m as usize + part.a as usize + part.s as usize
        })
}

fn collapse_workflows(workflows: &mut HashMap<&str, Vec<Rule>>) -> usize {
    let mut replace: HashMap<&str, Next> = HashMap::new();

    workflows.iter().for_each(|(label, rules)| {
        let unique_nexts: Vec<Next<'_>> = rules.iter().map(|rule| rule.next).unique().collect();
        if unique_nexts.len() == 1
            && (matches!(unique_nexts[0], Next::Accepted)
                || matches!(unique_nexts[0], Next::Rejected))
        {
            // Only accepts or rejects in workflow => can replace references to workflow by next
            replace.insert(label, unique_nexts[0]);
        }
    });

    workflows.iter_mut().for_each(|(_, rules)| {
        rules.iter_mut().for_each(|rule| {
            if let Next::Workflow(label) = rule.next {
                if let Some(next) = replace.get(label) {
                    rule.next = *next;
                }
            }
        })
    });

    replace.iter().for_each(|(&label, _)| {
        workflows.remove(label).unwrap();
    });

    replace.len()
}

fn qualify<'a>(
    part: &Part,
    mut workflow_iter: impl Iterator<Item = &'a Rule<'a>>,
    workflows: &'a HashMap<&str, Vec<Rule>>,
) -> Next<'a> {
    let rule = workflow_iter.next().unwrap();
    if part.check(&rule.cond) {
        match rule.next {
            Next::Workflow(label) => qualify(part, workflows.get(label).unwrap().iter(), workflows),
            _ => rule.next,
        }
    } else {
        qualify(part, workflow_iter, workflows)
    }
}

#[derive(Debug)]
struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl From<&str> for Part {
    fn from(s: &str) -> Self {
        let ratings: Vec<u16> = s[1..s.len() - 1]
            .split(',')
            .map(|rating| rating[2..].parse().unwrap())
            .collect();
        if let [x, m, a, s] = *ratings.as_slice() {
            Self { x, m, a, s }
        } else {
            unreachable!()
        }
    }
}

impl Part {
    fn rating(&self, cat: &Category) -> u16 {
        match cat {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    fn check(&self, cond: &Condition) -> bool {
        match cond {
            Condition::Gt(cat, cmp) => self.rating(cat) > *cmp,
            Condition::Lt(cat, cmp) => self.rating(cat) < *cmp,
            Condition::Pass => true,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

impl From<&str> for Category {
    fn from(s: &str) -> Self {
        match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Next<'a> {
    Workflow(&'a str),
    Accepted,
    Rejected,
}

impl<'a> From<&'a str> for Next<'a> {
    fn from(s: &'a str) -> Self {
        match s {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            _ => Self::Workflow(s),
        }
    }
}

#[derive(Debug)]
enum Condition {
    Gt(Category, u16),
    Lt(Category, u16),
    Pass,
}

impl From<&str> for Condition {
    fn from(s: &str) -> Self {
        let (cat, rating) = s.split_once(['<', '>']).unwrap();
        if s.contains("<") {
            Self::Lt(Category::from(cat), rating.parse().unwrap())
        } else if s.contains(">") {
            Self::Gt(Category::from(cat), rating.parse().unwrap())
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug)]
struct Rule<'a> {
    cond: Condition,
    next: Next<'a>,
}

impl<'a> Rule<'a> {
    fn references(&self) -> Option<Category> {
        match &self.cond {
            Condition::Gt(cat, _) => Some(*cat),
            Condition::Lt(cat, _) => Some(*cat),
            Condition::Pass => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use rstest::rstest;

    #[test]
    fn example_part1() {
        let example = "px{a<2006:qkq,m>2090:A,rfg}
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
        assert_eq!(solve(example), 19114);
    }

    //     #[test]
    //     fn example() {
    //         let example = "px{a<2006:qkq,m>2090:A,rfg}
    // pv{a>1716:R,A}
    // lnx{m>1548:A,A}
    // rfg{s<537:gd,x>2440:R,A}
    // qs{s>3448:A,lnx}
    // qkq{x<1416:A,crn}
    // crn{x>2662:A,R}
    // in{s<1351:px,qqz}
    // qqz{s>2770:qs,m<1801:hdj,R}
    // gd{a>3333:R,R}
    // hdj{m>838:A,pv}

    // {x=787,m=2655,a=1222,s=2876}
    // {x=1679,m=44,a=2067,s=496}
    // {x=2036,m=264,a=79,s=2244}
    // {x=2461,m=1339,a=466,s=291}
    // {x=2127,m=1623,a=2188,s=1013}";
    //         assert_eq!(solve(example), 167409079868000);
    //     }

    // #[rstest]
    // #[case("{x=787,m=2655,a=1222,s=2876}", 7540)]
    // #[case("{x=2036,m=264,a=79,s=2244}", 4623)]
    // #[case("{x=2127,m=1623,a=2188,s=1013}", 6951)]
    // fn test_process_line(#[case] input: &str, #[case] expected: usize) {
    //     assert_eq!(process_line(input), expected);
    // }
}

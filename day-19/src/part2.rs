use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet};

use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};

pub fn solve(input: &str) -> usize {
    let (workflows, _) = input.split_once("\n\n").unwrap();

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
    let slowlanes = calculate_slowlanes(&workflows);

    let mut transitions: HashMap<Category, HashSet<usize>> = HashMap::from([
        (Category::X, HashSet::from([1, 4001])),
        (Category::M, HashSet::from([1, 4001])),
        (Category::A, HashSet::from([1, 4001])),
        (Category::S, HashSet::from([1, 4001])),
    ]);

    workflows.iter().for_each(|(_, rules)| {
        rules.iter().for_each(|rule| match rule.cond {
            Condition::Gt(cat, val) => {
                transitions.get_mut(&cat).unwrap().insert(val as usize + 1);
            }
            Condition::Lt(cat, val) => {
                transitions.get_mut(&cat).unwrap().insert(val as usize);
            }
            Condition::Pass => (),
        })
    });

    let x_windows = transitions
        .get(&Category::X)
        .unwrap()
        .into_iter()
        .sorted()
        .tuple_windows()
        .collect::<Vec<(&usize, &usize)>>();
    let m_windows = transitions
        .get(&Category::M)
        .unwrap()
        .into_iter()
        .sorted()
        .tuple_windows()
        .collect::<Vec<(&usize, &usize)>>();
    let a_windows = transitions
        .get(&Category::A)
        .unwrap()
        .into_iter()
        .sorted()
        .tuple_windows()
        .collect::<Vec<(&usize, &usize)>>();
    let s_windows = transitions
        .get(&Category::S)
        .unwrap()
        .into_iter()
        .sorted()
        .tuple_windows()
        .collect::<Vec<(&usize, &usize)>>();

    let workflow_entry = workflows.get("in").unwrap().iter();

    let total_iterations = x_windows.len() * m_windows.len() * a_windows.len() * s_windows.len();
    let progress_bar = ProgressBar::new(total_iterations as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    iproduct!(x_windows, m_windows, a_windows, s_windows)
        .progress_with(progress_bar)
        .map(|((&x0, &x1), (&m0, &m1), (&a0, &a1), (&s0, &s1))| {
            let part = Part {
                x: x0 as u16,
                m: m0 as u16,
                a: a0 as u16,
                s: s0 as u16,
            };
            if let Next::Accepted = qualify(&part, workflow_entry.clone(), &workflows, &slowlanes) {
                (x1 - x0) * (m1 - m0) * (a1 - a0) * (s1 - s0)
            } else {
                0
            }
        })
        .sum()
}

fn calculate_slowlanes<'a>(
    workflows: &'a HashMap<&str, Vec<Rule>>,
) -> HashMap<&'a str, HashSet<&'a Category>> {
    let mut slowlanes: HashMap<&str, HashSet<&Category>> = HashMap::new();

    while workflows.len() > slowlanes.len() {
        workflows.iter().for_each(|(&label, rules)| {
            if !slowlanes.contains_key(label) {
                let references = rules
                    .iter()
                    .filter_map(|rule| rule.next.get_reference())
                    .collect::<Vec<&str>>();
                let all_references_known = references
                    .iter()
                    .all(|reference| slowlanes.contains_key(reference));

                if all_references_known {
                    let own_referenced_categories = rules
                        .iter()
                        .filter_map(|rule| rule.cond.get_referenced_category());
                    let other_referenced_categories = references
                        .iter()
                        .flat_map(|&reference| slowlanes.get(reference))
                        .flatten()
                        .map(|&reference| reference);
                    slowlanes.insert(
                        label,
                        own_referenced_categories
                            .chain(other_referenced_categories)
                            .collect(),
                    );
                }
            }
        });
    }
    slowlanes
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
    slowlanes: &'a HashMap<&str, HashSet<&Category>>,
) -> Next<'a> {
    for category in [Category::X, Category::M, Category::A, Category::S] {
        // if slowlanes.get()
    }

    let rule = workflow_iter.next().unwrap();
    if part.check(&rule.cond) {
        match rule.next {
            Next::Workflow(label) => qualify(
                part,
                workflows.get(label).unwrap().iter(),
                workflows,
                slowlanes,
            ),
            _ => rule.next,
        }
    } else {
        qualify(part, workflow_iter, workflows, slowlanes)
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

impl<'a> Next<'a> {
    fn get_reference(&self) -> Option<&str> {
        match &self {
            Next::Workflow(reference) => Some(&reference),
            Next::Accepted | Next::Rejected => None,
        }
    }
}

#[derive(Debug)]
enum Condition {
    Gt(Category, u16),
    Lt(Category, u16),
    Pass,
}

impl Condition {
    fn get_referenced_category(&self) -> Option<&Category> {
        match &self {
            Condition::Gt(cat, _) => Some(cat),
            Condition::Lt(cat, _) => Some(cat),
            Condition::Pass => None,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
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
        assert_eq!(solve(example), 167409079868000);
    }
}

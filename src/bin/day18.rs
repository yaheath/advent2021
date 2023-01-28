use std::cell::{Ref, RefCell};
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use advent_lib::read::read_input;

#[derive(Clone, Debug)]
struct Node(Rc<RefCell<Element>>);

impl Node {
    fn new(element: Element) -> Self {
        Node(
            Rc::new(RefCell::new(element))
        )
    }
    fn borrow(&self) -> Ref<Element> {
        self.0.borrow()
    }
    fn replace(&self, element: Element) -> Element {
        self.0.replace(element)
    }
    fn duplicate(&self) -> Node {
        Node(
            Rc::new(RefCell::new(self.borrow().duplicate()))
        )
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.borrow())
    }
}

#[derive(Clone, Debug)]
enum Element {
    Single(u8),
    Pair(Node, Node),
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::Single(v) => write!(f, "{v}"),
            Element::Pair(a, b) => write!(f, "[{a},{b}]"),
        }
    }
}

impl Element {
    fn add_singles(&self, other: &Element) -> Self {
        match (self, other) {
            (Element::Single(a), Element::Single(b)) => Element::Single(a + b),
            _ => panic!(),
        }
    }
    fn duplicate(&self) -> Self {
        match self {
            Element::Single(v) => Element::Single(*v),
            Element::Pair(a, b) => Element::Pair(a.duplicate(), b.duplicate()),
        }
    }
}

#[derive(Clone, Debug)]
struct SFNum (
    Node, // this is expected to be a node containing Element::Pair
);

fn parse_element(itr: &mut dyn Iterator<Item=char>) -> Option<Element> {
    let first = itr.next()?;
    match first {
        '[' => {
            let sa = parse_element(itr)?;
            if itr.next()? != ',' { return None }
            let sb = parse_element(itr)?;
            if itr.next()? != ']' { return None }
            Some(Element::Pair(Node::new(sa), Node::new(sb)))
        },
        '0'..='9' => {
            // assuming inputs are always "reduced", i.e. no values > 9
            let val = first as u8 - b'0';
            Some(Element::Single(val))
        },
        _ => None
    }
}

impl FromStr for SFNum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr = s.chars().peekable();
        if let Some(e) = parse_element(&mut itr) {
            Ok(SFNum(Node::new(e)))
        }
        else {
            Err(())
        }
    }
}

enum TResult {
    NoChange,
    Reduced,
    AddRight(u8),
    Explode,
}


fn traverse_explode(node: &Node, depth: usize, left_num: &RefCell<Option<Node>>, right_num: Option<u8>) -> TResult {
    //println!("traversing: {node} depth={depth}");
    let mut right_num = right_num;
    let mut traverse_left: Option<Node> = None;
    let mut traverse_right: Option<Node> = None;
    let mut handle_single: Option<u8> = None;
    match &*node.borrow() {
        Element::Pair(left, right) => {
            if depth >= 4 && right_num.is_none() {
                return TResult::Explode;
            } else {
                traverse_left = Some(left.clone());
                traverse_right = Some(right.clone());
            }
        },
        Element::Single(val) => {
            handle_single = Some(*val);
        },
    }
    if traverse_left.is_some() {
        let left = traverse_left.unwrap();
        let right = traverse_right.unwrap();
        match traverse_explode(&left, depth + 1, left_num, right_num) {
            TResult::Reduced => { return TResult::Reduced; },
            TResult::AddRight(val) => { right_num = Some(val) },
            TResult::NoChange => {},
            TResult::Explode => {
                let oldleft = left.replace(Element::Single(0));
                match oldleft {
                    Element::Pair(a, b) => {
                        let leftopt = left_num.borrow();
                        if let Some(leftelem) = &*leftopt {
                            let new = a.borrow().add_singles(&leftelem.borrow());
                            leftelem.replace(new);
                        }
                        if let Element::Single(rn) = *b.borrow() {
                            right_num = Some(rn);
                        }
                    },
                    other => panic!("unexpected element {other:?}"),
                }
            },
        };
        match traverse_explode(&right, depth + 1, left_num, right_num) {
            TResult::Reduced => { return TResult::Reduced; },
            TResult::AddRight(val) => { return TResult::AddRight(val); },
            TResult::NoChange => {},
            TResult::Explode => {
                let oldright = right.replace(Element::Single(0));
                match oldright {
                    Element::Pair(a, b) => {
                        let leftopt = left_num.borrow();
                        if let Some(leftelem) = &*leftopt {
                            let new = a.borrow().add_singles(&leftelem.borrow());
                            leftelem.replace(new);
                        }
                        if let Element::Single(rn) = *b.borrow() {
                            return TResult::AddRight(rn);
                        }
                    },
                    other => panic!("unexpected element {other:?}"),
                }
            },
        };
    }
    if handle_single.is_some() {
        let val = handle_single.unwrap();
        if let Some(inc) = right_num {
            node.replace(Element::Single(val + inc));
            return TResult::Reduced;
        }
        *(left_num.borrow_mut()) = Some(node.clone());
    }

    return TResult::NoChange;
}

fn traverse_split(node: &Node) -> TResult {
    let mut traverse_left: Option<Node> = None;
    let mut traverse_right: Option<Node> = None;
    let mut handle_single: Option<u8> = None;
    match &*node.borrow() {
        Element::Pair(left, right) => {
            traverse_left = Some(left.clone());
            traverse_right = Some(right.clone());
        },
        Element::Single(val) => {
            handle_single = Some(*val);
        },
    }
    if traverse_left.is_some() {
        let left = traverse_left.unwrap();
        let right = traverse_right.unwrap();
        match traverse_split(&left) {
            TResult::Reduced => { return TResult::Reduced; },
            TResult::NoChange => {},
            _ => panic!(),
        }
        match traverse_split(&right) {
            TResult::Reduced => { return TResult::Reduced; },
            TResult::NoChange => {},
            _ => panic!(),
        }
    }
    if handle_single.is_some() {
        let val = handle_single.unwrap();
        if val > 9 {
            let newelem = Element::Pair(
                Node::new(Element::Single(val / 2)),
                Node::new(Element::Single((val + 1) / 2)),
            );
            node.replace(newelem);
            return TResult::Reduced;
        };
    }
    return TResult::NoChange;
}

fn magnitude(node: &Node) -> u64 {
    match &*node.borrow() {
        Element::Single(val) => *val as u64,
        Element::Pair(a, b) => magnitude(a) * 3 + magnitude(b) * 2,
    }
}

impl SFNum {
    fn reduce(&mut self) {
        loop {
            let ln = RefCell::new(None);
            match traverse_explode(&self.0, 0, &ln, None) {
                TResult::NoChange => { },
                TResult::Reduced | TResult::AddRight(_) => {continue},
                TResult::Explode => panic!(),
            }
            match traverse_split(&self.0) {
                TResult::NoChange => { break; },
                TResult::Reduced => {},
                _ => panic!(),
            }
        }
    }
    fn magnitude(&self) -> u64 {
        magnitude(&self.0)
    }
    fn add(&self, other: &Self) -> Self {
        let mut new = SFNum(
            Node::new(
                Element::Pair(
                    self.0.duplicate(),
                    other.0.duplicate(),
                )
            )
        );
        new.reduce();
        new
    }
}

impl fmt::Display for SFNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn part1(input: &Vec<SFNum>) -> u64 {
    let mut sum = input[0].clone();
    for row in input.iter().skip(1) {
        sum = sum.add(row);
    }
    sum.magnitude()
}

fn part2(input: &Vec<SFNum>) -> u64 {
    input
        .iter()
        .tuple_combinations()
        .flat_map(|(a, b)| vec![a.add(b), b.add(a)])
        .map(|x| x.magnitude())
        .max()
        .unwrap()
}

fn main() {
    let input: Vec<SFNum> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use advent_lib::read::test_input;
    use super::*;

    #[test]
    fn day18_test() {
        let sfnum = SFNum::from_str("[[1,9],[8,5]]").unwrap();
        assert_eq!(format!("{sfnum}"), "[[1,9],[8,5]]");
        let sfnum = SFNum::from_str("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]").unwrap();
        assert_eq!(format!("{sfnum}"), "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]");

        for (from, to) in vec![
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            ("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
        ] {
            let mut sfnum = SFNum::from_str(from).unwrap();
            sfnum.reduce();
            assert_eq!(format!("{sfnum}"), to);
        }

        let mut sfnum1 = SFNum::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
        let sfnum2 = SFNum::from_str("[1,1]").unwrap();
        sfnum1 = sfnum1.add(&sfnum2);
        assert_eq!(format!("{sfnum1}"), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

        let sumtest = vec![
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ];
        let mut sfnum = SFNum::from_str(sumtest[0]).unwrap();
        for s in sumtest.iter().skip(1) {
            let n = SFNum::from_str(s).unwrap();
            sfnum = sfnum.add(&n);
            //println!("= {sfnum}");
        }
        assert_eq!(format!("{sfnum}"), "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");

        let input: Vec<SFNum> = test_input(include_str!("day18.testinput"));
        assert_eq!(part1(&input), 4140);
        assert_eq!(part2(&input), 3993);
    }
}

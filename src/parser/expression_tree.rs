use once_cell::sync::{OnceCell, Lazy};

use crate::{lexer::token::TokenKind, error::ParserError};

use super::{term::Term, binary_operator::BinaryOperator, unary_operator::{UnaryOperator, self}, util::InputUtil, Node};

#[derive(Debug, PartialEq)]
pub struct ExpressionTree {
    pub left: Option<Box<ExpressionTree>>,
    pub this: ExpressionNode,
    pub right: Option<Box<ExpressionTree>>,
}
#[derive(Debug, PartialEq)]
pub enum ExpressionNode {
    Prefix(UnaryOperator),
    Infix(BinaryOperator),
    Leaf(Term),
    Branch(Vec<TokenKind>),
}

struct OperatorInfo {
    pub associativity: Associativity,
    pub operator_type: OperatorType
}

enum OperatorType {
    Infix,
    Prefix,
}

enum Associativity {
    LeftToRight,
    RightToLeft,
}

static OPERATOR_INFO: Lazy<Vec<(Vec<TokenKind>, OperatorInfo)>> = Lazy::new(|| {
    vec![
        (
            vec![TokenKind::Plus, TokenKind::Minus], 
            OperatorInfo { associativity: Associativity::LeftToRight, operator_type: OperatorType::Infix }
        ),
        (
            vec![TokenKind::LEq], OperatorInfo { associativity: Associativity::LeftToRight, operator_type: OperatorType::Infix }),
    ]
});

pub fn create_tree(area: super::ParseInput) -> Result<ExpressionTree, ParserError> {
    for (operators, operator_info) in OPERATOR_INFO.iter() {
        if let Some(tree) = find_tree_for_operators(area, operators, operator_info){
            return Ok(tree);
        }
    }
    
    todo!();
}

fn find_area_parentheses(input: super::ParseInput) -> Result<super::ParseInput, ParserError> {
    println!("Find parentheses {input:?}");
    let mut count = 0;
    let mut cursor = 0;
    loop {
        match input.get(cursor) {
            Some(TokenKind::ParenthesesOpen) => count += 1,
            Some(TokenKind::ParenthesesClosed) => count -= 1,
            Some(_) => {}
            None => return Err(ParserError::NoTokenFound),
        }
        if count == 0 {
            println!("Find parentheses returns {:?}", &input[1..cursor]);
            return Ok(&input[1..cursor]);
        }
        if count < 0 {
            unreachable!();
        }
        cursor += 1;
    }
}

fn assign_first_left_tree(left_area: super::ParseInput) -> Option<Box<ExpressionTree>> {
    let mut left_area= left_area;

    match left_area.get(0) {
        Some(TokenKind::ParenthesesOpen) => {
            todo!()
        },
        Some(token) if UnaryOperator::parse(left_area).is_ok() => {
            let (left_tail, operator) = UnaryOperator::parse(left_area).unwrap();
            left_area = left_tail;
            if let Ok((left_tail, term)) = Term::parse(left_area) {
                if left_tail.is_empty() {
                    Some( 
                        ExpressionTree {
                            left: Some(
                                ExpressionTree {
                                    left: None,
                                    right: None, 
                                    this: ExpressionNode::Leaf(term),
                                }.into()
                            ),
                            right: None, 
                            this: ExpressionNode::Prefix(operator),
                        }.into()
                    )
                } else {
                    let left_node = ExpressionNode::Branch(Vec::from(left_area));     
                    Some( 
                        ExpressionTree {
                            left: None,
                            right: None, 
                            this: left_node,
                        }.into()
                    )
                }
            } else {
                todo!()
            }
        },
        Some(token) if Term::parse(left_area).is_ok() => {
            let (left_tail, term) = Term::parse(left_area).unwrap();
            if left_tail.is_empty() {
                Some( 
                    ExpressionTree {
                        left: None,
                        right: None, 
                        this: ExpressionNode::Leaf(term),
                    }.into()
                )
            } else {
                let left_node = ExpressionNode::Branch(Vec::from(left_area));     
                Some( 
                    ExpressionTree {
                        left: None,
                        right: None, 
                        this: left_node,
                    }.into()
                )
            }
        },
        Some(token) => todo!(),
        None => todo!()
    }

    // if let Ok((left_tail, operator)) = UnaryOperator::parse(left_area) {
    //     left_area = left_tail;
    //     if let Ok((left_tail, term)) = Term::parse(left_area) {
    //         if left_tail.is_empty() {
    //             Some( 
    //                 ExpressionTree {
    //                     left: Some(
    //                         ExpressionTree {
    //                             left: None,
    //                             right: None, 
    //                             this: ExpressionNode::Leaf(term),
    //                         }.into()
    //                     ),
    //                     right: None, 
    //                     this: ExpressionNode::Prefix(operator),
    //                 }.into()
    //             )
    //         } else {
    //             let left_node = ExpressionNode::Branch(Vec::from(left_area));     
    //             Some( 
    //                 ExpressionTree {
    //                     left: None,
    //                     right: None, 
    //                     this: left_node,
    //                 }.into()
    //             )
    //         }
    //     } else {
    //         todo!()
    //     }
    // } else if let Ok((left_tail, term)) = Term::parse(left_area) {
    //     if left_tail.is_empty() {
    //         Some( 
    //             ExpressionTree {
    //                 left: None,
    //                 right: None, 
    //                 this: ExpressionNode::Leaf(term),
    //             }.into()
    //         )
    //     } else {
    //         let left_node = ExpressionNode::Branch(Vec::from(left_area));     
    //         Some( 
    //             ExpressionTree {
    //                 left: None,
    //                 right: None, 
    //                 this: left_node,
    //             }.into()
    //         )
    //     }
    // } else if left_area.get(0) == Some(&TokenKind::ParenthesesOpen) {
    //     todo!()
    // } else {
    //     todo!()
    // }
}

fn find_tree_for_operators(area: super::ParseInput, operators: &Vec<TokenKind>, operator_info: &OperatorInfo) -> Option<ExpressionTree> {
    
    let mut area = area;
    
    let mut split = area.find_index_where(|token| {operators.contains(token)})?;
    let mut left_tree:Box<ExpressionTree> = assign_first_left_tree(&area[..split])?;
    //TODO WHAT IF PREFIX
    loop {
        let operator = match operator_info.operator_type {
            OperatorType::Infix => { 
                let (_, operator) = BinaryOperator::parse(&area[split..]).ok()?;
                // tail= new_tail;
                ExpressionNode::Infix(operator)
            }
            OperatorType::Prefix => { 
                let (_, operator) = UnaryOperator::parse(&area[split..]).ok()?;
                // tail= new_tail;
                ExpressionNode::Prefix(operator)
            }
        };
        let right_tree = 
        match area.get(split+1) {
            Some(&TokenKind::ParenthesesOpen) => {
                let sub_area = find_area_parentheses(&area[split+1..]).ok()?;
                area = &area[sub_area.len()+2+1..]; // Add parentheses and the split
                create_tree(sub_area)
            }
            Some(_) => {
                let (result, term) = Term::parse(&area[split+1..]).ok()?;
                area = result;
                Ok(ExpressionTree { 
                    left: None,
                    right: None, 
                    this: ExpressionNode::Leaf(term),
                })
            }
            None => { todo!() }
        }.ok().map(|tree| tree.into());
        left_tree = 
                ExpressionTree {
                    left: Some(left_tree),
                    right: right_tree, 
                    this: operator,
                }.into()
            ;
        if let Some(cursor) = area.find_index_where(|token| {operators.contains(token)}) {
            split = cursor;
        } else {
            return Some(*left_tree);
        }
    }
}
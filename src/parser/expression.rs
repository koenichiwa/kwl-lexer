use std::collections::VecDeque;

use crate::{lexer::token::TokenKind, error::ParserError};

use super::{term::Term, Node, ParseInput, util::InputUtil, unary_operator::UnaryOperator, binary_operator::BinaryOperator, expression_tree::{ExpressionTree, ExpressionNode, create_tree}};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Complex(ExpressionTree),
    Simple(Term),
}

impl Node for Expression {
    fn parse(input: super::ParseInput) -> super::ParseResult<Self> {      
        
        let tokens = create_tree(input)?;
        let cursor = input.find(&TokenKind::NewLine).unwrap_or_default();
        Ok((&cursor, Expression::Complex(tokens)))
    }
}

impl Expression {
    
}

// fn bla(input: super::ParseInput) -> Result<Vec<TokenKind>, ParserError> {
//     let area = find_area(input)?;
//     let mut stack = VecDeque::<_>::new();
//     let mut queue = Vec::<_>::new();

//     for token in &area[1..] {
//         match token {
//             TokenKind::Comma => {}
//             TokenKind::StringLiteral(string) => todo!(),
//             TokenKind::IntegerLiteral(int) => queue.push(TokenKind::IntegerLiteral(*int)),
//             TokenKind::FloatLiteral(float) => queue.push(TokenKind::FloatLiteral(*float)),
//             TokenKind::Identifier(identifier) => queue.push(TokenKind::Identifier(identifier.to_owned())),
//             TokenKind::Plus => { 
//                 if let Some(operator) = stack.get(0) {
//                     if *operator == TokenKind::Plus || *operator == TokenKind::Minus {
//                         queue.push(stack.pop_front().unwrap())
//                     }
//                 }
//                 stack.push_front(TokenKind::Plus);
//             }
//             TokenKind::Minus => { 
//                 if let Some(operator) = stack.get(0) {
//                     if *operator == TokenKind::Plus || *operator == TokenKind::Minus {
//                         queue.push(stack.pop_front().unwrap())
//                     }
//                 }
//                 stack.push_front(TokenKind::Minus);
//             }
//             TokenKind::FunctionIdentifier(identifier) => {
//                 stack.push_front(TokenKind::FunctionIdentifier(identifier.to_owned()));
//             },
//             TokenKind::ParenthesesOpen => {
//                 stack.push_front(TokenKind::ParenthesesOpen);
//             }
//             TokenKind::LEq => {
//                 stack.push_front(TokenKind::LEq);
//             }
//             TokenKind::ParenthesesClosed => {
//                 loop {
//                     let token = stack.pop_front().ok_or_else(|| todo!())?;
//                     if token == TokenKind::ParenthesesOpen { break; }
//                     else { queue.push(token) }
//                 }
//             }
//             token => { 
//                 println!("HERE: {area:?}");
//                 todo!()
//             }
//         }
//     }

//     while !stack.is_empty() {
//         queue.push(stack.pop_front().unwrap());
//     }

//     Ok(queue)
// }



// THIS ONE

// fn create_tree(area: super::ParseInput) -> Result<ExpressionTree, ParserError> {
//     println!("Expression: {area:?}");  
//     let mut area = area;
//     let mut left_tree = 
//     match area.get(0) {
//         Some(TokenKind::ParenthesesOpen) => {
//                 let sub_area = find_area_parentheses(area)?;
//                 area = &area[sub_area.len()+2..]; // Add parentheses
//                 create_tree(sub_area)
//             }
//         Some(_) => { 
//             let (tail, term) = Term::parse(&area)?;
//             area = tail;
//             Ok(ExpressionTree {
//                 left: None,
//                 right: None, 
//                 this: ExpressionNode::Leaf(term)
//             })
//         }
//         None => Err(ParserError::NoTokenFound)
//     }?;
//     println!("Expression 1: {area:?}");  
//     loop {
//         left_tree =
//         match area.get(0) {
//             // TokenKind::ParenthesesOpen => {
//             //     let sub_area = find_area_parentheses(area)?;
//             //     bla(sub_area)
//             // }
//             Some(TokenKind::Plus) => {
//                 let (tail, result) = get_right_tree(&area[1..], left_tree, BinaryOperator::Plus)?;
//                 area = tail;
//                 result
//             }
//             Some(TokenKind::Minus) => {
//                 let (tail, result) = get_right_tree(&area[1..], left_tree, BinaryOperator::Minus)?;
//                 area = tail;
//                 result
//             }
//             Some(TokenKind::LEq) => {
//                 let (tail, result) = get_right_tree(area, left_tree, BinaryOperator::LEq)?;
//                 area = tail;
//                 result
//             }
//             Some(token) => { 
//                 println!("Expression 2b: {area:?}");  
//                 return Err(ParserError::InvalidToken(token.to_owned())) 
//             }
//             None => { return Ok(left_tree); }
//         }
//     }
// }





// fn find_area(input: super::ParseInput) -> Result<super::ParseInput, ParserError> {
//     let cursor = input.find_index(&TokenKind::NewLine).ok_or_else(|| {
//         ParserError::NoTokenFound
//     })?;
//     Ok(&input[..cursor])
// }

// fn find_area_parentheses(input: super::ParseInput) -> Result<super::ParseInput, ParserError> {
//     let mut count = 0;
//     let mut cursor = 0;
//     loop {
//         match input.get(cursor) {
//             Some(TokenKind::ParenthesesOpen) => count += 1,
//             Some(TokenKind::ParenthesesClosed) => count -= 1,
//             Some(_) => {}
//             None => return Err(ParserError::NoTokenFound),
//         }
//         if count == 0 {
//             return Ok(&input[1..cursor]);
//         }
//         if count < 0 {
//             todo!()
//         }
//         cursor += 1;
//     }
// }

// fn get_right_tree(area: super::ParseInput, left_tree: ExpressionTree, operator: BinaryOperator) -> Result<(super::ParseInput, ExpressionTree), ParserError> {
//     let mut tail = area;
//     let right_tree =
//     if area.get(0) == Some(&TokenKind::ParenthesesOpen) {
//         let sub_area = find_area_parentheses(area)?;
//         tail = &area[sub_area.len()+2..]; // Add parentheses
//         create_tree(sub_area)
//     } else {
//         let (result, term) = Term::parse(area)?;
//         tail = result;
//         Ok(ExpressionTree { 
//             left: None,
//             right: None, 
//             this: ExpressionNode::Leaf(term),
//         })
//     }?;
//     Ok((tail, 
//         ExpressionTree {
//         left: Some(left_tree.into()),
//         right: Some(right_tree.into()), 
//         this: ExpressionNode::Infix(operator)
//     }))
// }
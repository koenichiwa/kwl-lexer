#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Unknown length
    FunctionIdentifier(String),
    Identifier(String),

    // Literals
    FloatLiteral(f64),
    IntegerLiteral(u64),
    StringLiteral(String),

    // Keywords
    DefinitionToken,
    ReturnToken,
    IfToken,
    ElseToken,

    // 2 Character Atoms
    Arrow,
    LEq,

    // 1 Character Atoms
    ParenthesesOpen,
    ParenthesesClosed,
    AccoladeOpen,
    AccoladeClosed,

    Colon,
    SemiColon,
    Comma,

    Plus,
    Minus,
    AssignmentToken,

    NewLine,
}

// TODO: try to use as returntype of lexer (FIRST SAVE THE SHIT OUT OF THE PROJECT)
struct Token<'a> {
    kind: TokenKind,
    start: &'a str,
    end: &'a str,
}

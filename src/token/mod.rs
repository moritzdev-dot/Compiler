#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    Plus,
    Minus,
    Astrik,
    Slash,

    LParent,
    RParent,
    LBrace,
    RBrace,
    LBrack,
    RBrack,
    
    Assign,

    Colon,
    Comma,
    Dot,
    Semicolon,

    Integer,
    String,
    Identifier,
    Func,
    Var,
    Const,
    Return,
    EOF,
}


#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.token_type, self.value)
        
    }
}

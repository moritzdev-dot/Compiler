use std::collections::HashMap;

use crate::token::*;

pub struct Tokenizer {
    input: String,
    cur_idx: usize,
    cur_char: char,
    next_char: char,
    keywords: HashMap<String, TokenType>
}

impl Tokenizer {
    pub fn new(s: String) -> Self {
        let mut map = HashMap::new();

        map.insert(String::from("func"), TokenType::Func);
        map.insert(String::from("return"), TokenType::Return);
        map.insert(String::from("var"), TokenType::Var);
        map.insert(String::from("const"), TokenType::Const);

        return Tokenizer {
            input: s.clone(),
            cur_idx: 0,
            cur_char: s
                .chars()
                .nth(0)
                .unwrap(),
            next_char: s
                .chars()
                .nth(1)
                .unwrap(),
            keywords: map,

        }
    }

    fn shift(&mut self) {
        if self.cur_idx + 2 >= self.input.len() {
            self.cur_char = self.next_char;
            self.next_char = '\0';
            return;
        }

        self.cur_idx += 1;
        self.cur_char = self.next_char;
        self.next_char = self
            .input
            .chars()
            .nth(self.cur_idx + 1)
            .unwrap();
    }
     
    fn is_number(c: char) -> bool {
        return '0' <= c && c <= '9'
    }
    fn is_letter(c: char) -> bool {
        return 'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z'
    }
    fn get_integer(&mut self) -> String{
        let mut s = String::new();
        while Self::is_number(self.next_char) {
            s += &String::from(self.cur_char);
            self.shift();
        }
        s += &String::from(self.cur_char);

        return s;
    }

    fn get_string(&mut self) -> String{
        let mut s = String::new();
        while self.cur_char != '"' {
            s += &String::from(self.cur_char);
            self.shift();
        }
        return s;
    }

    fn get_identifier(&mut self) -> String{
        let mut s = String::new();
        while Self::is_number(self.next_char) || 
            Self::is_letter(self.next_char) || 
            self.cur_char == '_' {
            s += &String::from(self.cur_char);
            self.shift();
        }
        s += &String::from(self.cur_char);

        return s;
    }

    pub fn next_token(&mut self) -> Token{
        let mut t = Token{
            token_type: TokenType::EOF,
            value: String::from(self.cur_char),
        };
        match self.cur_char {
            '+' => t.token_type = TokenType::Plus,
            '-' => t.token_type = TokenType::Minus,
            '*' => t.token_type = TokenType::Astrik,
            '/' => t.token_type = TokenType::Slash,
            '(' => t.token_type = TokenType::LParent,
            ')' => t.token_type = TokenType::RParent,
            '{' => t.token_type = TokenType::LBrace,
            '}' => t.token_type = TokenType::RBrace,
            '[' => t.token_type = TokenType::LBrack,
            ']' => t.token_type = TokenType::RBrack,
            '=' => t.token_type = TokenType::Assign,
            ':' => t.token_type = TokenType::Colon,
            ';' => t.token_type = TokenType::Semicolon,
            ',' => t.token_type = TokenType::Comma,
            '.' => t.token_type = TokenType::Dot,
            '\0' => t.token_type = TokenType::EOF,
            _ => {
                if Self::is_number(self.cur_char) {
                    t.value = self.get_integer();
                    t.token_type = TokenType::Integer;
                } else if self.cur_char == '"' {
                    self.shift();
                    t.value = self.get_string();
                    t.token_type = TokenType::String;
                } else {
                    t.value = self.get_identifier();
                    match self.keywords.get(&t.value) {
                        Some(ty) => {
                            t.token_type = ty.clone();
                        }
                        None => {
                            t.token_type = TokenType::Identifier;
                        }
                    }
                }
            }
        }
        self.shift();

        while self.cur_char == ' ' {
            self.shift();
        }

        return t;

    }
}


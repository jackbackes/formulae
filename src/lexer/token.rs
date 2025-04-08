use nom::*; 
use std::iter::Enumerate;
use std::fmt; 

#[derive(Debug, Clone)]
pub enum Token {
    //Literal
    Integer(i64),
    Float(f64),
    Boolean(bool), 
    Text(String), 
    // Error
    Null, 
    Div, 
    Value, 
    Ref, 
    Name, 
    Num, 
    NA, 
    GettingData, 
    // References
    MultiSheet(String), 
    Sheet(String), 
    Range(String), 
    Cell(String), 
    VRange(String), 
    HRange(String), 
    // Symbols
    Plus,
    Minus,
    Divide,
    Multiply,
    Exponent, 
    Ampersand, 
    Equal,
	Exclamation, 
    Comma,
    Period, 
    Colon,
    SemiColon,
    LAngle,
    RAngle, 
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Ident(String), 
    EOF
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for Token { }

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Integer(i) => write!(f, "{}", i), 
            Token::Float(x) => write!(f, "{}", x), 
            Token::Boolean(b) => {
                if *b {
                    write!(f, "TRUE")
                } else {
                    write!(f, "FALSE")
                }
            }, 
            Token::Text(s) => write!(f, "{}", s), 
            Token::MultiSheet(s) => write!(f, "{}", s), 
            Token::Sheet(s) => write!(f, "{}", s), 
            Token::Range(s) => write!(f, "{}", s), 
            Token::Cell(s) => write!(f, "{}", s), 
            Token::VRange(s) => write!(f, "{}", s), 
            Token::HRange(s) => write!(f, "{}", s), 
            Token::Ident(s) => write!(f, "{}", s), 
            Token::Null => write!(f, "#NULL!"), 
            Token::Div => write!(f, "#DIV/0!"), 
            Token::Value => write!(f, "#VALUE!"),
            Token::Ref => write!(f, "#REF!"), 
            Token::Name => write!(f, "#NAME!"), 
            Token::Num => write!(f, "#NUM!"), 
            Token::NA => write!(f, "#N/A!"), 
            Token::GettingData => write!(f, "#GETTING_DATA"), 
            Token::Plus => write!(f, "+"), 
            Token::Minus => write!(f, "-"), 
            Token::Divide => write!(f, "/"), 
            Token::Multiply => write!(f, "*"), 
            Token::Exponent => write!(f, "^"), 
            Token::Ampersand => write!(f, "&"), 
            Token::Equal => write!(f, "="), 
            Token::Exclamation => write!(f, "!"), 
            Token::Comma => write!(f, ","), 
            Token::Period => write!(f, "."), 
            Token::Colon => write!(f, ":"), 
            Token::SemiColon => write!(f, ";"), 
            Token::LAngle => write!(f, "<"), 
            Token::RAngle => write!(f, ">"), 
            Token::LParen => write!(f, "("), 
            Token::RParen => write!(f, ")"), 
            Token::LBrace => write!(f, "{{"), 
            Token::RBrace => write!(f, "}}"), 
            Token::LBracket => write!(f, "["), 
            Token::RBracket => write!(f, "]"), 
            Token::EOF => write!(f, ""), 
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Tokens<'a> {
    pub tok: &'a [Token], 
    pub start: usize, 
    pub end: usize, 
}

impl<'a> Tokens<'a> {
    pub fn new(vec: &'a [Token]) -> Self {
        Tokens {
            tok: vec,
            start: 0,
            end: vec.len(),
        }
    }
}

impl <'a> Input for Tokens<'a> {
    type Item = &'a Token;

    type Iter = std::slice::Iter<'a, Token>;

    type IterIndices = Enumerate<std::slice::Iter<'a, Token>>;

    fn input_len(&self) -> usize {
        self.tok.len()
    }

    fn take(&self, count: usize) -> Self {
        Tokens {
            tok: &self.tok[0..count],
            start: 0,
            end: count,
        }
    }

    fn take_from(&self, count: usize) -> Self {
        Tokens {
            tok: &self.tok[count..],
            start: 0,
            end: self.tok.len() - count,
        }
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.tok.split_at(count);
        let first = Tokens {
            tok: prefix,
            start: 0,
            end: prefix.len(),
        };
        let second = Tokens {
            tok: suffix,
            start: 0,
            end: suffix.len(),
        };
        (second, first)
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
      where
        P: Fn(Self::Item) -> bool {
        self.tok.iter().position(predicate)
    }

    fn iter_elements(&self) -> Self::Iter {
        self.tok.iter()
    }

    fn iter_indices(&self) -> Self::IterIndices {
        self.tok.iter().enumerate()
    }

    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        if self.tok.len() >= count {
            Ok(count)
        } else {
            Err(Needed::Unknown)
        }
    }
}
use crate::nodes::Node;
use crate::tokens::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    current: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut result = Self {
            tokens,
            pos: 0,
            current: None,
        };
        result.advance();
        result
    }

    fn raise_error() -> String {
        "Invalid syntax".into()
    }

    fn advance(&mut self) {
        self.current = if self.tokens.len() > self.pos {
            Some(self.tokens[self.pos])
        } else {
            None
        };
        self.pos += 1;
    }

    pub fn parse(&mut self) -> Option<Result<Box<Node>, String>> {
        if self.current.is_none() {
            return None;
        }

        let result = self.expr();

        if self.current.is_some() {
            return Some(Err(Self::raise_error()));
        }

        Some(result)
    }

    fn expr(&mut self) -> Result<Box<Node>, String> {
        let mut result = self.term()?;

        while let Some(current) = self.current {
            use Node::*;
            use Token::*;
            if current == Plus {
                self.advance();
                result = Box::new(AddNode(result, self.term()?));
            } else if current == Minus {
                self.advance();
                result = Box::new(SubtractNode(result, self.term()?));
            } else {
                break;
            }
        }

        Ok(result)
    }

    fn term(&mut self) -> Result<Box<Node>, String> {
        let mut result = self.factor()?;

        while let Some(current) = self.current {
            use Node::*;
            use Token::*;
            if current == Multiply {
                self.advance();
                result = Box::new(MultiplyNode(result, self.factor()?));
            } else if current == Divide {
                self.advance();
                result = Box::new(DivideNode(result, self.factor()?));
            } else {
                break;
            }
        }

        Ok(result)
    }

    fn factor(&mut self) -> Result<Box<Node>, String> {
        match self.current {
            Some(token) => {
                use Node::*;
                use Token::*;
                match token {
                    Lparen => {
                        self.advance();
                        let result = self.expr();

                        if let Some(current_token) = self.current {
                            if current_token != Rparen {
                                return Err(Self::raise_error());
                            }
                        } else {
                            return Err(Self::raise_error());
                        }

                        self.advance();
                        return result;
                    }
                    Number(val) => {
                        self.advance();
                        return Ok(Box::new(NumberNode(val)));
                    }
                    Plus => {
                        self.advance();
                        return Ok(Box::new(PlusNode(self.factor()?)));
                    }
                    Minus => {
                        self.advance();
                        return Ok(Box::new(MinusNode(self.factor()?)));
                    }
                    _ => return Err(Self::raise_error()),
                }
            }
            None => Err(Self::raise_error()),
        }
    }
}

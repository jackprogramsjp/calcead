use crate::tokens::Token;

pub struct Lexer {
    text: String,
    pos: usize,
    current: Option<char>,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        let mut result = Self {
            text,
            pos: 0,
            current: None,
        };
        result.advance();
        result
    }

    fn advance(&mut self) {
        self.current = self.text.chars().nth(self.pos);
        self.pos += 1;
    }

    fn gen_number(&mut self) -> Token {
        let mut decimal_point_count = 0;
        let mut number = String::from(self.current.unwrap());
        self.advance();

        while let Some(current) = self.current {
            if current == '.' || current.is_digit(10) {
                if current == '.' {
                    decimal_point_count += 1;
                    if decimal_point_count > 1 {
                        break;
                    }
                }

                number.push(current);
                self.advance();
            } else {
                break;
            }
        }

        if number.starts_with('.') {
            number = format!("0{}", number);
        }

        if number.ends_with('.') {
            number.push('0');
        }

        Token::Number(number.parse::<f32>().unwrap())
    }
}

impl Iterator for Lexer {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(' ') | Some('\n') | Some('\t') => {
                self.advance();
                self.next()
            }
            

            Some(c) => {
                if c.is_digit(10) {
                    Some(Ok(self.gen_number()))
                } else {
                    Some(Err(format!("Illegal character '{}'", c)))
                }
            },
            None => None,
        }
    }
}

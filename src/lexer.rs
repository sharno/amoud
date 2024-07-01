use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    VariableKeyword,
    IfKeyword,
    ElseKeyword,
    ThenKeyword,
    True,
    False,

    Identifier(String),
    Number(f64),
    String(String),

    Plus,
    Minus,
    Multiply,
    Divide,

    LT,
    GT,
    LTE,
    GTE,
    EQ,
    NEQ,

    LeftParen,
    RightParen,
    Equals,

    Dot,
}

struct Lexer<'a> {
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars(),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        match self.chars.next() {
            Some(ch) => match ch {
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '*' => Some(Token::Multiply),
                '/' => Some(Token::Divide),
                '<' => Some(Token::LT),
                '>' => Some(Token::LT),
                // '<=' => Some(Token::LTE),
                // '>=' => Some(Token::GTE),
                // '==' => Some(Token::EQ),
                // '!=' => Some(Token::NEQ),
                '(' => Some(Token::LeftParen),
                ')' => Some(Token::RightParen),
                '=' => Some(Token::Equals),
                '.' => Some(Token::Dot),
                '"' => Some(self.read_string()),
                '٠'..='٩' => Some(self.read_number(ch)),
                'ا'..='ي' | 'آ' | 'أ' | 'إ' => Some(self.read_identifier_or_keyword(ch)),
                _ => None, // Unrecognized character
            },
            None => None, // End of input
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.chars.clone().next() {
            if !ch.is_whitespace() {
                break;
            }
            self.chars.next();
        }
    }

    fn read_number(&mut self, first_digit: char) -> Token {
        let mut number = first_digit.to_string();
        while let Some(ch) = self.chars.clone().next() {
            if !('٠'..='٩').contains(&ch) && ch != ',' {
                break;
            }
            number.push(self.chars.next().unwrap());
        }
        Token::Number(arabic_numeral_to_float(&number))
    }

    fn read_string(&mut self) -> Token {
        let mut string = String::new();
        let mut escaped = false;

        while let Some(ch) = self.chars.next() {
            match (ch, escaped) {
                ('"', false) => break,
                ('\\', false) => escaped = true,
                (ch, true) => {
                    string.push(ch);
                    escaped = false;
                }
                (ch, false) => string.push(ch),
            }
        }
        Token::String(string)
    }

    fn read_identifier_or_keyword(&mut self, first_char: char) -> Token {
        let mut identifier = first_char.to_string();
        while let Some(ch) = self.chars.clone().next() {
            if !('ا'..='ي').contains(&ch) && !['آ', 'أ', 'إ', 'ة', 'ى'].contains(&ch) {
                break;
            }
            identifier.push(self.chars.next().unwrap());
        }

        match identifier.as_str() {
            "متغير" => Token::VariableKeyword,
            "لو" => Token::IfKeyword,
            "ف" => Token::ThenKeyword,
            "وإلا" => Token::ElseKeyword,
            "نعم" => Token::True,
            "لا" => Token::False,
            _ => Token::Identifier(identifier),
        }
    }
}

fn arabic_numeral_to_float(s: &str) -> f64 {
    s.chars().fold(0.0, |acc, c| {
        acc * 10.0
            + match c {
                '٠' => 0.0,
                '١' => 1.0,
                '٢' => 2.0,
                '٣' => 3.0,
                '٤' => 4.0,
                '٥' => 5.0,
                '٦' => 6.0,
                '٧' => 7.0,
                '٨' => 8.0,
                '٩' => 9.0,
                ',' => return acc,
                _ => acc,
            }
    })
}

pub fn run(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);

    let mut res = vec![];
    while let Some(token) = lexer.next_token() {
        res.push(token)
    }
    return res;
}

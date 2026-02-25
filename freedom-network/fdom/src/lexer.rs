use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Symbols
    At,                          // @
    OpenBrace,                   // {
    CloseBrace,                  // }
    Equals,                       // =
    Comma,                        // ,
    
    // Literals
    Identifier(String),
    String(String),
    Number(f64),
    Boolean(bool),
    
    // Keywords
    Page,
    Header,
    Section,
    Footer,
    Aside,
    Heading,
    Paragraph,
    Text,
    Link,
    Image,
    Code,
    Pre,
    List,
    Ordered,
    Table,
    Form,
    Video,
    Audio,
    
    // Special
    Eof,
}

#[derive(Error, Debug)]
pub enum LexError {
    #[error("Unexpected character: {0} at position {1}")]
    UnexpectedChar(char, usize),
    
    #[error("Unterminated string at position {0}")]
    UnterminatedString(usize),
    
    #[error("Invalid number format: {0}")]
    InvalidNumber(String),
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = if chars.is_empty() { None } else { Some(chars[0]) };
        
        Lexer {
            input: chars,
            position: 0,
            current_char,
        }
    }
    
    fn advance(&mut self) {
        self.position += 1;
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = Some(self.input[self.position]);
        }
    }
    
    fn peek(&self) -> Option<char> {
        if self.position + 1 < self.input.len() {
            Some(self.input[self.position + 1])
        } else {
            None
        }
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn skip_comment(&mut self) {
        // Line comment: //
        if self.current_char == Some('/') && self.peek() == Some('/') {
            while self.current_char.is_some() && self.current_char != Some('\n') {
                self.advance();
            }
        }
    }
    
    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        ident
    }
    
    fn read_string(&mut self, quote: char) -> Result<String, LexError> {
        let start_pos = self.position;
        self.advance(); // Skip opening quote
        
        let mut string = String::new();
        let mut is_triple = false;
        
        // Check for triple quote
        if self.current_char == Some(quote) && self.peek() == Some(quote) {
            is_triple = true;
            self.advance();
            self.advance();
        }
        
        loop {
            match self.current_char {
                None => return Err(LexError::UnterminatedString(start_pos)),
                Some(ch) => {
                    if ch == quote {
                        if is_triple {
                            if self.peek() == Some(quote) {
                                self.advance();
                                if self.peek() == Some(quote) {
                                    self.advance();
                                    self.advance();
                                    break;
                                } else {
                                    string.push(quote);
                                    string.push(quote);
                                }
                            } else {
                                string.push(quote);
                            }
                        } else {
                            self.advance();
                            break;
                        }
                    } else if ch == '\\' {
                        self.advance();
                        match self.current_char {
                            Some('n') => string.push('\n'),
                            Some('t') => string.push('\t'),
                            Some('r') => string.push('\r'),
                            Some('\\') => string.push('\\'),
                            Some('"') => string.push('"'),
                            Some('\'') => string.push('\''),
                            _ => string.push(ch),
                        }
                        self.advance();
                    } else {
                        string.push(ch);
                        self.advance();
                    }
                }
            }
        }
        
        Ok(string)
    }
    
    fn read_number(&mut self) -> Result<f64, LexError> {
        let mut num_str = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() || ch == '.' || ch == '-' && num_str.is_empty() {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        num_str.parse::<f64>()
            .map_err(|_| LexError::InvalidNumber(num_str))
    }
    
    pub fn next_token(&mut self) -> Result<Token, LexError> {
        loop {
            self.skip_whitespace();
            
            match self.current_char {
                None => return Ok(Token::Eof),
                
                Some('@') => {
                    self.advance();
                    let ident = self.read_identifier();
                    
                    let token = match ident.to_lowercase().as_str() {
                        "page" => Token::Page,
                        "header" => Token::Header,
                        "section" => Token::Section,
                        "footer" => Token::Footer,
                        "aside" => Token::Aside,
                        "heading" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => Token::Heading,
                        "paragraph" => Token::Paragraph,
                        "text" => Token::Text,
                        "link" => Token::Link,
                        "image" => Token::Image,
                        "code" => Token::Code,
                        "pre" => Token::Pre,
                        "list" => Token::List,
                        "ordered" => Token::Ordered,
                        "table" => Token::Table,
                        "form" => Token::Form,
                        "video" => Token::Video,
                        "audio" => Token::Audio,
                        _ => Token::Identifier(format!("@{}", ident)),
                    };
                    return Ok(token);
                }
                
                Some('{') => {
                    self.advance();
                    return Ok(Token::OpenBrace);
                }
                
                Some('}') => {
                    self.advance();
                    return Ok(Token::CloseBrace);
                }
                
                Some('=') => {
                    self.advance();
                    return Ok(Token::Equals);
                }
                
                Some(',') => {
                    self.advance();
                    return Ok(Token::Comma);
                }
                
                Some('"') => {
                    let string = self.read_string('"')?;
                    return Ok(Token::String(string));
                }
                
                Some('\'') => {
                    let string = self.read_string('\'')?;
                    return Ok(Token::String(string));
                }
                
                Some('/') if self.peek() == Some('/') => {
                    self.skip_comment();
                    continue;
                }
                
                Some(ch) if ch.is_ascii_digit() || (ch == '-' && self.peek().map_or(false, |c| c.is_ascii_digit())) => {
                    let num = self.read_number()?;
                    return Ok(Token::Number(num));
                }
                
                Some(ch) if ch.is_alphabetic() || ch == '_' => {
                    let ident = self.read_identifier();
                    
                    let token = match ident.to_lowercase().as_str() {
                        "true" => Token::Boolean(true),
                        "false" => Token::Boolean(false),
                        _ => Token::Identifier(ident),
                    };
                    return Ok(token);
                }
                
                Some(ch) => {
                    return Err(LexError::UnexpectedChar(ch, self.position));
                }
            }
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token()?;
            let is_eof = matches!(token, Token::Eof);
            tokens.push(token);
            
            if is_eof {
                break;
            }
        }
        
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("@page { title = \"Hello\" }");
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.iter().any(|t| matches!(t, Token::Page)));
        assert!(tokens.iter().any(|t| matches!(t, Token::String(s) if s == "Hello")));
    }
    
    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("width = 800, opacity = 0.95");
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.iter().any(|t| matches!(t, Token::Number(n) if n == &800.0)));
    }
}

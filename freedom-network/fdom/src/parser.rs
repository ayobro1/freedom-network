use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
    
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }
    
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub key: String,
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AstNode {
    Document {
        metadata: HashMap<String, Value>,
        children: Vec<AstNode>,
    },
    Element {
        tag: String,
        attributes: HashMap<String, Value>,
        children: Vec<AstNode>,
    },
    Text(String),
}

pub struct Parser {
    tokens: Vec<crate::lexer::Token>,
    position: usize,
}

use crate::lexer::Token;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }
    
    fn current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::Eof)
    }
    
    #[allow(dead_code)]
    fn peek_token(&self) -> &Token {
        self.tokens.get(self.position + 1).unwrap_or(&Token::Eof)
    }
    
    fn advance(&mut self) {
        self.position += 1;
    }
    
    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if std::mem::discriminant(self.current_token()) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, self.current_token()))
        }
    }
    
    pub fn parse(&mut self) -> Result<AstNode, String> {
        self.parse_document()
    }
    
    fn parse_document(&mut self) -> Result<AstNode, String> {
        let mut metadata = HashMap::new();
        let mut children = Vec::new();
        
        // Parse @page block if present
        if matches!(self.current_token(), Token::Page) {
            self.advance();
            self.expect(Token::OpenBrace)?;
            
            while !matches!(self.current_token(), Token::CloseBrace | Token::Eof) {
                let (key, value) = self.parse_attribute()?;
                metadata.insert(key, value);
                
                if matches!(self.current_token(), Token::Comma) {
                    self.advance();
                }
            }
            
            self.expect(Token::CloseBrace)?;
        }
        
        // Parse remaining elements
        while !matches!(self.current_token(), Token::Eof) {
            children.push(self.parse_element()?);
        }
        
        Ok(AstNode::Document { metadata, children })
    }
    
    fn parse_element(&mut self) -> Result<AstNode, String> {
        let tag = match self.current_token() {
            Token::Identifier(ref name) => name.clone(),
            Token::Header => "header".to_string(),
            Token::Section => "section".to_string(),
            Token::Footer => "footer".to_string(),
            Token::Aside => "aside".to_string(),
            Token::Paragraph => "paragraph".to_string(),
            Token::Heading => "heading".to_string(),
            Token::Text => "text".to_string(),
            Token::Link => "link".to_string(),
            Token::Image => "image".to_string(),
            Token::Code => "code".to_string(),
            Token::List => "list".to_string(),
            Token::Page => "page".to_string(),
            _ => return Err(format!("Unexpected token: {:?}", self.current_token())),
        };
        
        self.advance();
        self.expect(Token::OpenBrace)?;
        
        let mut attributes = HashMap::new();
        let mut children = Vec::new();
        
        while !matches!(self.current_token(), Token::CloseBrace | Token::Eof) {
            // Try to parse as attribute (key = value)
            if matches!(self.current_token(), Token::Identifier(_)) {
                if let Token::Identifier(ref key) = self.current_token() {
                    let key_copy = key.clone();
                    self.advance();
                    
                    if matches!(self.current_token(), Token::Equals) {
                        self.advance();
                        let value = self.parse_value()?;
                        attributes.insert(key_copy, value);
                    } else {
                        // Not an attribute, treat as element
                        self.position -= 1; // Rewind
                        children.push(self.parse_element()?);
                    }
                }
            } else {
                // Parse as child element or text
                children.push(self.parse_element()?);
            }
            
            if matches!(self.current_token(), Token::Comma) {
                self.advance();
            }
        }
        
        self.expect(Token::CloseBrace)?;
        
        Ok(AstNode::Element {
            tag,
            attributes,
            children,
        })
    }
    
    fn parse_attribute(&mut self) -> Result<(String, Value), String> {
        let key = match self.current_token() {
            Token::Identifier(ref k) => k.clone(),
            _ => return Err("Expected identifier for attribute".to_string()),
        };
        
        self.advance();
        self.expect(Token::Equals)?;
        
        let value = self.parse_value()?;
        
        Ok((key, value))
    }
    
    fn parse_value(&mut self) -> Result<Value, String> {
        match self.current_token() {
            Token::String(ref s) => {
                let value = Value::String(s.clone());
                self.advance();
                Ok(value)
            }
            Token::Number(n) => {
                let value = Value::Number(*n);
                self.advance();
                Ok(value)
            }
            Token::Boolean(b) => {
                let value = Value::Boolean(*b);
                self.advance();
                Ok(value)
            }
            _ => Err(format!("Expected value, got {:?}", self.current_token())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    
    #[test]
    fn test_parse_simple_page() {
        let input = r#"
            @page {
                title = "Test",
                author = "Anonymous"
            }
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        match ast {
            AstNode::Document { metadata, .. } => {
                assert_eq!(
                    metadata.get("title"),
                    Some(&Value::String("Test".to_string()))
                );
            }
            _ => panic!("Expected Document node"),
        }
    }
}

pub mod lexer;
pub mod parser;
pub mod renderer;

use lexer::{Lexer, LexError};
use parser::{AstNode, Parser};
use renderer::Renderer;

pub struct FdomProcessor;

impl FdomProcessor {
    /// Parse .fdom source code and convert to HTML
    pub fn process(source: &str) -> Result<String, ProcessError> {
        // Tokenize
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().map_err(ProcessError::LexError)?;
        
        // Parse
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().map_err(ProcessError::ParseError)?;
        
        // Render
        let html = Renderer::render(&ast);
        
        Ok(html)
    }
    
    /// Parse .fdom source code to AST only
    pub fn parse_ast(source: &str) -> Result<AstNode, ProcessError> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().map_err(ProcessError::LexError)?;
        
        let mut parser = Parser::new(tokens);
        parser.parse().map_err(ProcessError::ParseError)
    }
}

#[derive(Debug)]
pub enum ProcessError {
    LexError(LexError),
    ParseError(String),
}

impl std::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessError::LexError(e) => write!(f, "Lexical error: {}", e),
            ProcessError::ParseError(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl std::error::Error for ProcessError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_document() {
        let fdom = r#"
            @page {
                title = "Test",
                theme = "dark"
            }
            @header {
                @heading { "Welcome" }
            }
            @section {
                @paragraph { "This is a test." }
            }
        "#;
        
        let html = FdomProcessor::process(fdom);
        assert!(html.is_ok());
        let output = html.unwrap();
        assert!(output.contains("<title>Test</title>"));
        assert!(output.contains("Welcome"));
    }
    
    #[test]
    fn test_with_links() {
        let fdom = r#"
            @page { title = "Links" }
            @section {
                @link { href = "page.fdom", text = "Next Page" }
            }
        "#;
        
        let html = FdomProcessor::process(fdom);
        assert!(html.is_ok());
        let output = html.unwrap();
        assert!(output.contains("href=\"page.fdom\""));
    }
}

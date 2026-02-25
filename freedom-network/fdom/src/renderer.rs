use crate::parser::{AstNode, Value};
use std::collections::HashMap;

pub struct Renderer;

impl Renderer {
    pub fn render(ast: &AstNode) -> String {
        match ast {
            AstNode::Document { metadata, children } => {
                Self::render_document(metadata, children)
            }
            AstNode::Element { tag, attributes, children } => {
                Self::render_element(tag, attributes, children)
            }
            AstNode::Text(text) => text.clone(),
        }
    }
    
    fn render_document(metadata: &HashMap<String, Value>, children: &[AstNode]) -> String {
        let title = metadata
            .get("title")
            .and_then(|v| v.as_string())
            .unwrap_or("Document");
        
        let theme = metadata
            .get("theme")
            .and_then(|v| v.as_string())
            .unwrap_or("dark");
        
        let css = Self::get_theme_css(theme);
        
        let mut html = String::from("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str(&format!("<title>{}</title>\n", Self::escape_html(title)));
        html.push_str("<meta charset=\"utf-8\">\n");
        html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str("<style>\n");
        html.push_str(&css);
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");
        
        for child in children {
            html.push_str(&Self::render(child));
        }
        
        html.push_str("</body>\n</html>");
        html
    }
    
    fn render_element(tag: &str, attributes: &HashMap<String, Value>, children: &[AstNode]) -> String {
        let mut html = String::new();
        
        match tag.to_lowercase().as_str() {
            "page" => {
                // Page is handled at document level
                for child in children {
                    html.push_str(&Self::render(child));
                }
            }
            
            "header" => {
                html.push_str("<header class=\"fdom-header\">\n");
                for child in children {
                    html.push_str(&Self::render(child));
                }
                html.push_str("</header>\n");
            }
            
            "section" => {
                html.push_str("<section class=\"fdom-section\">\n");
                for child in children {
                    html.push_str(&Self::render(child));
                }
                html.push_str("</section>\n");
            }
            
            "footer" => {
                html.push_str("<footer class=\"fdom-footer\">\n");
                for child in children {
                    html.push_str(&Self::render(child));
                }
                html.push_str("</footer>\n");
            }
            
            "aside" => {
                html.push_str("<aside class=\"fdom-aside\">\n");
                for child in children {
                    html.push_str(&Self::render(child));
                }
                html.push_str("</aside>\n");
            }
            
            "heading" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                let level = if tag == "heading" { "h2" } else { tag };
                let style = Self::build_style(attributes);
                html.push_str(&format!("<{}", level));
                if !style.is_empty() {
                    html.push_str(&format!(" style=\"{}\"", style));
                }
                html.push_str(">");
                
                for child in children {
                    html.push_str(&Self::render(child));
                }
                
                html.push_str(&format!("</{}>\\n", level));
            }
            
            "paragraph" => {
                let style = Self::build_style(attributes);
                html.push_str("<p");
                if !style.is_empty() {
                    html.push_str(&format!(" style=\"{}\"", style));
                }
                html.push_str(">");
                for child in children {
                    html.push_str(&Self::render(child));
                }
                html.push_str("</p>\n");
            }
            
            "text" => {
                for child in children {
                    html.push_str(&Self::render(child));
                }
            }
            
            "strong" => {
                html.push_str("<strong>");
                for child in children {
                    html.push_str(&Self::render(child));
                }
                html.push_str("</strong>");
            }
            
            "emphasis" => {
                html.push_str("<em>");
                for child in children {
                    html.push_str(&Self::render(child));
                }
                html.push_str("</em>");
            }
            
            "link" => {
                let href = attributes
                    .get("href")
                    .and_then(|v| v.as_string())
                    .unwrap_or("#");
                
                let text = attributes
                    .get("text")
                    .and_then(|v| v.as_string())
                    .unwrap_or("Link");
                
                let title = attributes
                    .get("title")
                    .and_then(|v| v.as_string())
                    .unwrap_or("");
                
                html.push_str(&format!(
                    "<a href=\"{}\" title=\"{}\">{}</a>",
                    Self::escape_html(href),
                    Self::escape_html(title),
                    Self::escape_html(text)
                ));
            }
            
            "image" => {
                let src = attributes
                    .get("src")
                    .and_then(|v| v.as_string())
                    .unwrap_or("");
                
                let alt = attributes
                    .get("alt")
                    .and_then(|v| v.as_string())
                    .unwrap_or("");
                
                let width = attributes
                    .get("width")
                    .and_then(|v| v.as_number())
                    .map(|n| n as u32);
                
                let height = attributes
                    .get("height")
                    .and_then(|v| v.as_number())
                    .map(|n| n as u32);
                
                html.push_str(&format!(
                    "<img src=\"{}\" alt=\"{}\"",
                    Self::escape_html(src),
                    Self::escape_html(alt)
                ));
                
                if let Some(w) = width {
                    html.push_str(&format!(" width=\"{}\"", w));
                }
                if let Some(h) = height {
                    html.push_str(&format!(" height=\"{}\"", h));
                }
                
                html.push_str(" />\n");
            }
            
            "code" => {
                html.push_str("<pre><code>");
                for child in children {
                    html.push_str(&Self::escape_html(&Self::render(child)));
                }
                html.push_str("</code></pre>\n");
            }
            
            "list" => {
                html.push_str("<ul>\n");
                for child in children {
                    html.push_str("<li>");
                    html.push_str(&Self::render(child));
                    html.push_str("</li>\n");
                }
                html.push_str("</ul>\n");
            }
            
            "ordered" => {
                html.push_str("<ol>\n");
                for child in children {
                    html.push_str("<li>");
                    html.push_str(&Self::render(child));
                    html.push_str("</li>\n");
                }
                html.push_str("</ol>\n");
            }
            
            "nav" => {
                html.push_str("<nav class=\"fdom-nav\">\n");
                for child in children {
                    html.push_str(&Self::render(child));
                }
                html.push_str("</nav>\n");
            }
            
            "quote" => {
                html.push_str("<blockquote>");
                for child in children {
                    html.push_str(&Self::render(child));
                }
                html.push_str("</blockquote>\n");
            }
            
            "box" | "div" => {
                let class = attributes
                    .get("class")
                    .and_then(|v| v.as_string())
                    .unwrap_or("");
                
                let style = Self::build_style(attributes);
                html.push_str("<div");
                
                if !class.is_empty() {
                    html.push_str(&format!(" class=\"{}\"", class));
                }
                if !style.is_empty() {
                    html.push_str(&format!(" style=\"{}\"", style));
                }
                html.push_str(">\n");
                
                for child in children {
                    html.push_str(&Self::render(child));
                }
                
                html.push_str("</div>\n");
            }
            
            "table" => {
                html.push_str("<table class=\"fdom-table\">\n");
                for child in children {
                    html.push_str(&Self::render(child));
                }
                html.push_str("</table>\n");
            }
            
            "row" => {
                html.push_str("<tr>\n");
                for child in children {
                    html.push_str(&Self::render(child));
                }
                html.push_str("</tr>\n");
            }
            
            "cell" => {
                html.push_str("<td>");
                for child in children {
                    html.push_str(&Self::render(child));
                }
                html.push_str("</td>");
            }
            
            "comment" => {
                // Comments are not rendered
            }
            
            _ => {
                // Generic element
                html.push_str(&format!("<{}", tag));
                let style = Self::build_style(attributes);
                if !style.is_empty() {
                    html.push_str(&format!(" style=\"{}\"", style));
                }
                html.push_str(">\n");
                
                for child in children {
                    html.push_str(&Self::render(child));
                }
                
                html.push_str(&format!("</{}>\\n", tag));
            }
        }
        
        html
    }
    
    fn build_style(attributes: &HashMap<String, Value>) -> String {
        let mut style = String::new();
        
        if let Some(color) = attributes.get("color").and_then(|v| v.as_string()) {
            style.push_str(&format!("color: {};", color));
        }
        if let Some(bg) = attributes.get("background").and_then(|v| v.as_string()) {
            style.push_str(&format!("background: {};", bg));
        }
        if let Some(padding) = attributes.get("padding").and_then(|v| v.as_number()) {
            style.push_str(&format!("padding: {}px;", padding as u32));
        }
        if let Some(margin) = attributes.get("margin").and_then(|v| v.as_number()) {
            style.push_str(&format!("margin: {}px;", margin as u32));
        }
        if let Some(border) = attributes.get("border").and_then(|v| v.as_string()) {
            style.push_str(&format!("border: {};", border));
        }
        if let Some(align) = attributes.get("align").and_then(|v| v.as_string()) {
            style.push_str(&format!("text-align: {};", align));
        }
        if let Some(font_size) = attributes.get("font-size").and_then(|v| v.as_number()) {
            style.push_str(&format!("font-size: {}px;", font_size as u32));
        }
        if let Some(font_weight) = attributes.get("font-weight").and_then(|v| v.as_string()) {
            style.push_str(&format!("font-weight: {};", font_weight));
        }
        
        style
    }
    
    fn escape_html(text: &str) -> String {
        text.replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&#39;")
    }
    
    fn get_theme_css(theme: &str) -> String {
        match theme.to_lowercase().as_str() {
            "light" => {
                r#"
                    body { 
                        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto;
                        background: #ffffff; 
                        color: #000000;
                        margin: 0;
                        padding: 20px;
                    }
                    h1, h2, h3, h4, h5, h6 { color: #333; margin-top: 1.5em; }
                    a { color: #0066cc; text-decoration: none; }
                    a:hover { text-decoration: underline; }
                    code, pre { background: #f0f0f0; padding: 10px; border-radius: 4px; }
                    blockquote { border-left: 4px solid #ccc; padding-left: 20px; }
                "#.to_string()
            }
            "dark" => {
                r#"
                    body { 
                        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto;
                        background: #1a1a1a; 
                        color: #e0e0e0;
                        margin: 0;
                        padding: 20px;
                    }
                    h1, h2, h3, h4, h5, h6 { color: #ffffff; margin-top: 1.5em; }
                    a { color: #4a9eff; text-decoration: none; }
                    a:hover { text-decoration: underline; }
                    code, pre { background: #2a2a2a; padding: 10px; border-radius: 4px; color: #4a9eff; }
                    blockquote { border-left: 4px solid #4a9eff; padding-left: 20px; color: #b0b0b0; }
                "#.to_string()
            }
            "high-contrast" => {
                r#"
                    body { 
                        font-family: Arial, sans-serif;
                        background: #000000; 
                        color: #ffffff;
                        margin: 0;
                        padding: 20px;
                    }
                    h1, h2, h3, h4, h5, h6 { color: #ffff00; }
                    a { color: #00ff00; text-decoration: underline; }
                    strong, em { font-weight: bold; }
                "#.to_string()
            }
            _ => Self::get_theme_css("dark"),
        }
    }
}

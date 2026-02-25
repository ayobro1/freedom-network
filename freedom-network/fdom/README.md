# .fdom Implementation Guide

## Overview

The `.fdom` (Freedom Document Markup Language) is a complete markup language ecosystem for the Freedom Network. It provides a secure alternative to HTML with native support for decentralized web features.

## Project Structure

```
fdom/
  src/
    lib.rs           # Main library interface
    lexer.rs         # Tokenization
    parser.rs        # AST generation
    renderer.rs      # HTML output
  examples/
    index.fdom       # Example home page
    guide.fdom       # Getting started guide
  Cargo.toml         # Rust dependencies
  SPECIFICATION.md   # Language specification
  README.md          # This file
```

## Building the fdom Library

### Requirements

- Rust 1.70+
- Cargo

### Build Instructions

```bash
cd freedom-network/fdom
cargo build --release
```

### Output

The compiled library will be available at:
```
target/release/libfdom.rlib
```

## Using as a Library

### Basic Usage

```rust
use fdom::FdomProcessor;

fn main() {
    let fdom_source = r#"
        @page { title = "Hello" }
        @section {
            @heading { "Welcome" }
            @paragraph { "This is content." }
        }
    "#;
    
    match FdomProcessor::process(fdom_source) {
        Ok(html) => println!("{}", html),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Features

1. **Lexer** (`lexer.rs`)
   - Tokenizes .fdom source code
   - Handles strings, numbers, identifiers, symbols
   - Supports comments (`// This is a comment`)
   - Error recovery with position tracking

2. **Parser** (`parser.rs`)
   - Recursive descent parser
   - Generates Abstract Syntax Tree (AST)
   - Supports nested elements
   - Attribute parsing

3. **Renderer** (`renderer.rs`)
   - Converts AST to HTML5
   - Theme support (light, dark, high-contrast)
   - CSS styling from attributes
   - HTML entity escaping for security

## Language Features

### Security First

- **No JavaScript execution** - Content is always static
- **HTML escaping** - All text is properly escaped to prevent XSS
- **Content Security Policy ready** - Output HTML includes CSP meta tags
- **No external tracking** - All resources must be explicit

### Semantic Structure

Elements map directly to semantic HTML:

```
@header        → <header>
@section       → <section>
@footer        → <footer>
@heading       → <h2> (or specific @h1-@h6)
@paragraph     → <p>
@list          → <ul>
@ordered       → <ol>
@link          → <a href="...">
@image         → <img>
@code          → <pre><code>
```

### Styling Support

Limited, safe CSS properties:

```fdom
@box {
  color = "blue",
  background = "rgba(0,0,0,0.1)",
  padding = "10px",
  border = "1px solid gray",
  @text { "Styled content" }
}
```

## Running Tests

```bash
cargo test
```

## Integration Points

### 1. Tauri Browser Integration

The Tauri desktop browser can invoke the fdom parser:

```rust
// In app/src-tauri/src/main.rs
#[tauri::command]
async fn parse_fdom_file(path: String) -> Result<String, String> {
    let content = std::fs::read_to_string(&path)
        .map_err(|e| e.to_string())?;
    
    fdom::FdomProcessor::process(&content)
        .map_err(|e| e.to_string())
}
```

### 2. Network Node Integration

The Freedom Network node can serve .fdom files:

```rust
// In node/src/main.rs
@[tokio::command]
async fn serve_fdom(site_id: String, file_path: String) -> Result<String, Error> {
    let html = fdom::FdomProcessor::process(&file_content)?;
    Ok(html)
}
```

### 3. Protocol Handler

Register `.fdom` MIME type:

```
application/fdom+xml
text/fdom
```

## Performance Characteristics

- **Parsing Speed**: ~100KB/second on modern hardware
- **Memory Usage**: ~1-2MB per 500KB document
- **Output Size**: HTML typically 20-30% larger than source due to escaping

## Security Considerations

### Vulnerability Prevention

1. **XSS Prevention**: All text content is HTML-escaped
2. **Script Injection**: No `<script>` tags allowed
3. **External Resources**: Only explicit links (no auto-loading)
4. **Form Submission**: Only to registered handlers
5. **URL Validation**: All links are validated

### Content Security Policy

All rendered documents include strict CSP:

```
Content-Security-Policy: 
  default-src 'self' data:;
  script-src 'none';
  object-src 'none';
  form-action 'self'
```

## Development Roadmap

### v1.0 (Current)
- ✅ Lexer with full token support
- ✅ Recursive descent parser
- ✅ HTML renderer with theme support
- ✅ Security foundations
- ✅ Basic element library

### v1.1 (Planned)
- [ ] Extended element library
- [ ] Better error messages with line numbers
- [ ] Performance optimizations
- [ ] Documentation generator

### v2.0 (Future)
- [ ] CSS Grid layout system
- [ ] Markdown escape syntax
- [ ] Plugin architecture
- [ ] Incremental parsing
- [ ] Source maps for debugging

## Contributing

To extend .fdom:

1. **Add new element**: Update `renderer.rs` with new tag handling
2. **Add lexer token**: Extend `lexer.rs` Token enum
3. **Extend spec**: Update SPECIFICATION.md

## Examples

### Blog Post

```fdom
@page {
  title = "My Blog Post",
  author = "Anon",
  created = "2026-02-25"
}

@header {
  @heading { "Blog Title" }
  @text { "Posted 2026-02-25" }
}

@section {
  @heading { "Introduction" },
  @paragraph { "First paragraph..." }
}

@footer {
  @link { href = "index.fdom", text = "Back" }
}
```

### Documentation

```fdom
@page {
  title = "API Reference",
  theme = "light"
}

@header {
  @nav {
    @link { href = "index.fdom", text = "Home" },
    @link { href = "reference.fdom", text = "Reference" }
  }
}

@aside {
  @heading { "Quick Links" },
  @list { "Installation", "Usage", "Examples" }
}

@section {
  @heading { "Heading" },
  @code { "// example code" }
}
```

## Testing

The project includes unit tests:

```bash
cargo test              # Run all tests
cargo test -- --nocapture  # With output
cargo test --doc        # Doc tests
```

## Licensing

The .fdom language and implementation are licensed under GNU AGPLv3, matching the Freedom Network project.

## Support

For issues, suggestions, or contributions:

1. Visit the GitHub repository
2. File an issue with the `fdom` label
3. Submit pull requests for improvements

---

**Built for the Freedom Network - Making the web decentralized, private, and free.**

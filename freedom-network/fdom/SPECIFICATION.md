# .FDOM - Freedom Document Markup Language

The `.fdom` language is a secure, decentralized markup language designed for the Freedom Network. It combines the simplicity of HTML with built-in security, privacy, and freedom-network-native features.

## Language Version: 1.0

---

## 1. Language Overview

### 1.1 Design Principles

- **Simple** - Easy to write, read, and parse
- **Secure** - No JavaScript execution, sandboxed by default
- **Privacy-First** - Metadata minimized, no tracking
- **Decentralized** - Native support for `.freedom` domains
- **Accessible** - Semantic structure built-in
- **Lightweight** - Fast to parse and render

### 1.2 File Extension

```
.fdom - Freedom Document Markup file
```

### 1.3 MIME Type

```
application/fdom+xml
text/fdom
```

---

## 2. Document Structure

### 2.1 Basic Template

```fdom
@page {
  title = "My Site"
  author = "Anonymous"
  description = "A Freedom Network site"
}

@header {
  @heading { "Welcome to Freedom" }
}

@section {
  @paragraph { "This is secure content." }
}

@footer {
  @paragraph { "© 2026 Freedom Network" }
}
```

### 2.2 Root Elements

Every `.fdom` document begins with a `@page` declaration and contains:

- `.fdom` itself - single root element per file
- Metadata in `@page` block
- Content in sections: `@header`, `@section`, `@aside`, `@footer`

---

## 3. Elements Reference

### 3.1 Document Metadata (`@page`)

Defines document-level properties. Required in every document.

```fdom
@page {
  title = "Page Title"
  author = "Author Name"
  description = "Page description"
  theme = "dark"
  lang = "en"
}
```

**Attributes:**
- `title` (required) - Page title
- `author` - Creator name (anonymous by default)
- `description` - Brief description
- `lang` - Language code (en, fr, de, etc.)
- `theme` - "light" or "dark" (default: dark for privacy)

---

### 3.2 Layout Elements

#### `@header`
Navigation and intro content at top of page.

```fdom
@header {
  @heading { "Site Name" }
  @nav { @link { href="index.fdom", text="Home" } }
}
```

#### `@section`
Main content container. Multiple allowed per page.

```fdom
@section {
  @heading { "Chapter 1" }
  @paragraph { "Content here..." }
}
```

#### `@aside`
Sidebar or supplementary content.

```fdom
@aside {
  @heading { "Quick Links" }
  @list { "Item 1", "Item 2" }
}
```

#### `@footer`
Footer content (metadata, links, copyright).

```fdom
@footer {
  @paragraph { "© 2026 Anonymous" }
}
```

---

### 3.3 Text Content Elements

#### `@heading` (or `@h1`, `@h2`, `@h3`, `@h4`, `@h5`, `@h6`)

```fdom
@heading { "Main Title" }
@h1 { "Largest Heading" }
@h2 { "Subheading" }
@h3 { "Sub-subheading" }
```

#### `@paragraph`

```fdom
@paragraph { "A paragraph of text." }
```

#### `@text`

Inline text (no paragraph wrapper).

```fdom
@text { "Just text" }
```

#### `@code` / `@pre`

Display code blocks.

```fdom
@code {
  "function hello() {
    console.log('Hello, Freedom!');
  }"
}

@pre { "Preformatted text" }
```

#### `@quote`

Blockquote.

```fdom
@quote { "Freedom is about choice." }
```

#### `@strong` / `@emphasis`

Emphasis and strong text (no HTML execution).

```fdom
@paragraph {
  @text { "This is " }
  @strong { "bold" }
  @text { " and " }
  @emphasis { "italic" }
  @text { "." }
}
```

---

### 3.4 List Elements

#### `@list` (unordered)

```fdom
@list {
  "Item 1",
  "Item 2",
  "Item 3"
}
```

#### `@ordered` (numbered list)

```fdom
@ordered {
  "First",
  "Second",
  "Third"
}
```

#### `@dictionary` (definition list)

```fdom
@dictionary {
  term = "Freedom",
  definition = "The state of being free."
}
```

---

### 3.5 Links and Media

#### `@link`

```fdom
@link {
  href = "other-page.fdom",
  text = "Click here"
}
```

**Attributes:**
- `href` - URL or `.fdom` file path
- `text` - Display text
- `title` - Hover tooltip

Supports:
- Relative paths: `page.fdom`, `../other/page.fdom`
- Freedom domains: `example.freedom/page.fdom`
- External: `https://example.com` (handled carefully)

#### `@image`

```fdom
@image {
  src = "image.png",
  alt = "Image description",
  width = 400,
  height = 300
}
```

**Attributes:**
- `src` - Image path (relative paths or embedded base64)
- `alt` - Accessibility description (required)
- `width` - Optional width in pixels
- `height` - Optional height in pixels

#### `@video` / `@audio`

```fdom
@video {
  src = "movie.mp4",
  controls = true,
  width = 800
}

@audio {
  src = "song.mp3",
  controls = true
}
```

---

### 3.6 Form Elements (Read-Only Display)

Forms in `.fdom` are static by default—no form submission without explicit server setup.

```fdom
@form {
  target = "submit.fdom",
  method = "post",
  
  @field {
    type = "text",
    name = "username",
    placeholder = "Enter username"
  },
  
  @button { text = "Submit" }
}
```

**Note:** Form data is only sent if the document is published on a Freedom Network site with proper server handlers.

---

### 3.7 Table Elements

```fdom
@table {
  @row {
    @cell { "Header 1" },
    @cell { "Header 2" }
  },
  @row {
    @cell { "Data 1" },
    @cell { "Data 2" }
  }
}
```

---

### 3.8 Container Elements

#### `@box` / `@div`

Generic container for grouping.

```fdom
@box {
  class = "highlight",
  @paragraph { "Important content" }
}
```

#### `@card`

Pre-styled container (semantic box).

```fdom
@card {
  title = "Card Title",
  @paragraph { "Card content" }
}
```

---

## 4. Styling

### 4.1 Inline Styles

Limited, security-focused styling via attributes:

```fdom
@paragraph {
  color = "blue",
  text = "Colored text"
}

@box {
  background = "rgba(255,0,0,0.1)",
  padding = "10px",
  border = "1px solid gray",
  @text { "Styled box" }
}
```

**Supported Properties:**
- `color` - Text color (CSS color names or hex)
- `background` - Background color
- `padding` - Padding in pixels
- `margin` - Margin in pixels
- `border` - Border style
- `align` - "left", "center", "right"
- `font-size` - Size in px
- `font-weight` - "normal", "bold"

### 4.2 Themes

Use the `theme` attribute in `@page` for global styling:

```fdom
@page {
  title = "Dark Mode Site",
  theme = "dark"
}
```

**Built-in Themes:**
- `light` - Light background, dark text
- `dark` - Dark background, light text (default)
- `high-contrast` - Maximum accessibility

---

## 5. Special Elements

### 5.1 Comments

```fdom
@comment { "This is a comment and won't be rendered" }
```

### 5.2 Metadata Block

```fdom
@meta {
  keywords = "freedom, privacy, decentralized",
  robots = "index, follow",
  created = "2026-02-25",
  updated = "2026-02-25"
}
```

### 5.3 Embedded Data

```fdom
@data {
  format = "json",
  content = """
  {
    "key": "value",
    "list": [1, 2, 3]
  }
  """
}
```

---

## 6. Syntax Rules

### 6.1 Basic Syntax

```
@element { content }
@element { attr1 = "value1", attr2 = "value2", content }
@element { "Just a string" }
@element { multiple, "string", items, "here" }
```

### 6.2 Nesting

```fdom
@section {
  @heading { "Title" },
  @paragraph {
    @text { "Some " }
    @strong { "bold" }
    @text { " text" }
  }
}
```

### 6.3 Strings

- Double quotes: `"String value"`
- Single quotes: `'Alternative'`
- Multi-line: Use `""" ... """`

```fdom
@quote { """
  Multi-line
  quote here
  spanning lines
""" }
```

### 6.4 Numbers and Booleans

```fdom
@image {
  width = 800,
  height = 600,
  responsive = true,
  opacity = 0.95
}
```

---

## 7. Security Model

### 7.1 No Script Execution

`.fdom` documents **never** execute JavaScript. Content is always static and safe.

### 7.2 No External Tracking

All resources must be explicitly requested. No automatic external requests.

### 7.3 Content Security Policy (CSP)

All `.fdom` documents render with strict CSP headers:

```
Content-Security-Policy: 
  default-src 'self' data:;
  script-src 'none';
  object-src 'none';
  base-uri 'self';
  form-action 'self'
```

### 7.4 URL Validation

- Internal links: `.fdom` files, relative paths
- Freedom links: `example.freedom/path`
- External: Only HTTPS, and marked as external

---

## 8. Best Practices

### 8.1 Accessibility

Always include alt text for images:

```fdom
@image {
  src = "photo.png",
  alt = "Descriptive text for screen readers"
}
```

### 8.2 Performance

- Keep documents under 5MB
- Optimize images (use PNG/WEBP)
- Use semantic structure for faster parsing

### 8.3 Privacy

- Don't leak metadata in descriptions
- Use anonymous author names
- Avoid including tracking pixels

---

## 9. Use Cases

### 9.1 Blog Post

```fdom
@page {
  title = "My Blog Post",
  author = "Anonymous",
  description = "A blog about freedom"
}

@header {
  @heading { "Freedom Thoughts" }
  @text { "Published: 2026-02-25" }
}

@section {
  @heading { "Introduction" },
  @paragraph { "Content..." }
}

@footer {
  @link { href = "index.fdom", text = "Back to Home" }
}
```

### 9.2 Documentation Site

```fdom
@page {
  title = ".fdom Reference",
  theme = "light"
}

@header {
  @nav {
    @link { href = "index.fdom", text = "Home" },
    @link { href = "spec.fdom", text = "Spec" }
  }
}

@aside {
  @heading { "Navigation" },
  @list { "Getting Started", "Elements", "Examples" }
}

@section {
  @heading { "Guide" },
  @paragraph { "Documentation..." }
}
```

---

## 10. Example Document

```fdom
@page {
  title = "Welcome to Freedom.net",
  author = "The Freedom Network",
  description = "A decentralized, private social network",
  lang = "en",
  theme = "dark"
}

@header {
  @heading { "🌐 Freedom Network" },
  @nav {
    @link { href = "index.fdom", text = "Home" },
    @link { href = "about.fdom", text = "About" },
    @link { href = "docs.fdom", text = "Docs" }
  }
}

@section {
  @heading { "Welcome" },
  @paragraph { "Experience the internet as it should be." },
  @image {
    src = "hero.png",
    alt = "Freedom Network illustration",
    width = 600
  }
}

@section {
  @heading { "Features" },
  @list {
    "End-to-end encrypted messaging",
    "No central server tracking",
    "Peer-to-peer content distribution",
    "Privacy by default"
  }
}

@footer {
  @paragraph { "© 2026 Freedom Network" },
  @link { href = "privacy.fdom", text = "Privacy Policy" }
}
```

---

## 11. Grammar (EBNF-like)

```
Document      = Page Header? (Section | Aside)* Footer?
Page          = "@page" "{" PageMeta "}"
PageMeta      = (Attribute ",")* Attribute
Element       = "@" ElementName "{" (Attribute | Content)* "}"
Attribute     = Identifier "=" Value
Content       = Element | String | List | Identifier
String        = '"' ... '"' | "'" ... "'" | '"""' ... '"""'
List          = "{" Content ("," Content)* "}"
Identifier    = [a-zA-Z_][a-zA-Z0-9_-]*
```

---

## 12. Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-02-25 | Initial release |

---

## 13. Future Extensions (v2.0)

- Markdown escape syntax
- CSS Grid layout system
- Limited interactive elements (no execution)
- Multi-language document support
- Plugin system for domain-specific extensions

---

**Language Designed for the Freedom Network**

*Making the web decentralized, private, and free.*

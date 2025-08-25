# Emmet Parser

A Rust implementation of an Emmet parser that converts Emmet shorthand syntax to HTML.

## Features

- **Element names**: `div`, `p`, `span`, etc.
- **IDs**: `div#main` → `<div id="main"></div>`
- **Classes**: `div.container` → `<div class="container"></div>`
- **Multiple classes**: `div.container.fluid` → `<div class="container fluid"></div>`
- **Attributes**: `input[type=text][placeholder=Enter name]` → `<input type="text" placeholder="Enter name" />`
- **Text content**: `div{Hello World}` → `<div>Hello World</div>`
- **Multiplication**: `div*3` → `<div></div><div></div><div></div>`
- **Nesting**: `div>p>span` → `<div><p><span></span></p></div>`
- **Siblings**: `div+p+span` → `<div></div><p></p><span></span>`
- **Self-closing tags**: `img`, `input`, `br`, `hr`, `meta`, `link`

## Usage

### Basic Usage

```rust
use emmet_parser::emmet_to_html;

fn main() {
    let html = emmet_to_html("div#main.container>p{Hello}+span.world*2").unwrap();
    println!("{}", html);
    // Output: <div id="main" class="container"><p>Hello</p></div><span class="world"></span><span class="world"></span>
}
```

### Parsing Elements

```rust
use emmet_parser::parse_emmet;

fn main() {
    let elements = parse_emmet("div#main.container").unwrap();
    let element = &elements[0];

    println!("Tag: {}", element.tag);
    println!("ID: {:?}", element.id);
    println!("Classes: {:?}", element.classes);
}
```

## Syntax Examples

| Emmet | HTML Output |
|-------|-------------|
| `div` | `<div></div>` |
| `div#main` | `<div id="main"></div>` |
| `div.container` | `<div class="container"></div>` |
| `div#main.container` | `<div id="main" class="container"></div>` |
| `input[type=text]` | `<input type="text" />` |
| `div{Hello}` | `<div>Hello</div>` |
| `div*3` | `<div></div><div></div><div></div>` |
| `div>p>span` | `<div><p><span></span></p></div>` |
| `div+p+span` | `<div></div><p></p><span></span>` |

## API Reference

### Functions

- `parse_emmet(input: &str) -> Result<Vec<EmmetElement>, EmmetError>`
  - Parses Emmet syntax and returns a vector of elements

- `emmet_to_html(input: &str) -> Result<String, EmmetError>`
  - Converts Emmet syntax directly to HTML string

### Structs

#### `EmmetElement`

Represents a parsed HTML element with the following fields:

- `tag: String` - The HTML tag name
- `id: Option<String>` - The element ID
- `classes: Vec<String>` - List of CSS classes
- `attributes: Vec<Attribute>` - List of HTML attributes
- `text: Option<String>` - Text content
- `children: Vec<EmmetElement>` - Child elements
- `multiplier: Option<u32>` - Multiplication factor

#### `Attribute`

Represents an HTML attribute:

- `name: String` - Attribute name
- `value: Option<String>` - Attribute value (optional)

### Error Types

- `EmmetError::InvalidSyntax(String)` - Invalid syntax in the input
- `EmmetError::UnclosedBracket` - Unclosed bracket or brace
- `EmmetError::InvalidAttribute` - Invalid attribute syntax

## Running Examples

```bash
cargo run --example basic
```

## Running Tests

```bash
cargo test
```

## License

This project is licensed under the MIT License.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmmetError {
    #[error("Invalid syntax: {0}")]
    InvalidSyntax(String),
    #[error("Unclosed bracket")]
    UnclosedBracket,
    #[error("Invalid attribute syntax")]
    InvalidAttribute,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmmetElement {
    pub tag: String,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attributes: Vec<Attribute>,
    pub text: Option<String>,
    pub children: Vec<EmmetElement>,
    pub multiplier: Option<u32>,
}

impl EmmetElement {
    pub fn new(tag: &str) -> Self {
        Self {
            tag: tag.to_string(),
            id: None,
            classes: Vec::new(),
            attributes: Vec::new(),
            text: None,
            children: Vec::new(),
            multiplier: None,
        }
    }

    pub fn to_html(&self) -> String {
        let mut html = String::new();

        // Handle multiplication
        let count = self.multiplier.unwrap_or(1);
        for i in 0..count {
            html.push_str(&self.element_to_html(i));
        }

        html
    }

    fn element_to_html(&self, _index: u32) -> String {
        let mut html = String::new();

        // Opening tag
        html.push('<');
        html.push_str(&self.tag);

        // ID
        if let Some(id) = &self.id {
            html.push_str(&format!(" id=\"{}\"", id));
        }

        // Classes
        if !self.classes.is_empty() {
            let classes = self.classes.join(" ");
            html.push_str(&format!(" class=\"{}\"", classes));
        }

        // Attributes
        for attr in &self.attributes {
            if let Some(value) = &attr.value {
                html.push_str(&format!(" {}=\"{}\"", attr.name, value));
            } else {
                html.push_str(&format!(" {}", attr.name));
            }
        }

        // Self-closing tags
        let self_closing_tags = ["img", "input", "br", "hr", "meta", "link"];
        if self_closing_tags.contains(&self.tag.as_str()) {
            html.push_str(" />");
            return html;
        }

        html.push('>');

        // Text content
        if let Some(text) = &self.text {
            html.push_str(text);
        }

        // Children
        for child in &self.children {
            html.push_str(&child.to_html());
        }

        // Closing tag
        html.push_str(&format!("</{}>", self.tag));

        html
    }
}

pub struct EmmetParser {
    input: String,
    position: usize,
}

impl EmmetParser {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<EmmetElement>, EmmetError> {
        let mut elements = Vec::new();

        while self.position < self.input.len() {
            self.skip_whitespace();

            if self.peek().is_none() {
                break;
            }

            let element = self.parse_element()?;
            elements.push(element);

            // Handle siblings
            if self.peek() == Some('+') {
                self.consume_char();
                continue;
            }

            self.skip_whitespace();
        }

        Ok(elements)
    }

    fn parse_element(&mut self) -> Result<EmmetElement, EmmetError> {
        let mut element = EmmetElement::new("div");

        // Parse tag name
        if let Some(tag) = self.parse_tag_name() {
            element.tag = tag;
        }

        // Parse ID
        if self.peek() == Some('#') {
            self.consume_char();
            element.id = Some(self.parse_identifier()?);
        }

        // Parse classes
        while self.peek() == Some('.') {
            self.consume_char();
            let class = self.parse_identifier()?;
            element.classes.push(class);
        }

        // Parse attributes
        while self.peek() == Some('[') {
            let attributes = self.parse_attributes()?;
            element.attributes.extend(attributes);
        }

        // Parse text content
        if self.peek() == Some('{') {
            element.text = Some(self.parse_text_content()?);
        }

        // Parse multiplication
        if self.peek() == Some('*') {
            self.consume_char();
            element.multiplier = Some(self.parse_number()?);
        }

        // Parse children
        if self.peek() == Some('>') {
            self.consume_char();
            element.children = self.parse_children()?;
        }

        Ok(element)
    }

    fn parse_tag_name(&mut self) -> Option<String> {
        let start = self.position;

        while self.position < self.input.len() {
            let ch = self.input.chars().nth(self.position)?;
            if ch.is_alphanumeric() || ch == '-' {
                self.position += 1;
            } else {
                break;
            }
        }

        if self.position > start {
            Some(self.input[start..self.position].to_string())
        } else {
            None
        }
    }

    fn parse_identifier(&mut self) -> Result<String, EmmetError> {
        let start = self.position;

        while self.position < self.input.len() {
            let ch = self.input.chars().nth(self.position).unwrap();
            if ch.is_alphanumeric() || ch == '-' || ch == '_' {
                self.position += 1;
            } else {
                break;
            }
        }

        if self.position > start {
            Ok(self.input[start..self.position].to_string())
        } else {
            Err(EmmetError::InvalidSyntax("Expected identifier".to_string()))
        }
    }

    fn parse_attributes(&mut self) -> Result<Vec<Attribute>, EmmetError> {
        let mut attributes = Vec::new();

        if self.consume_char() != Some('[') {
            return Err(EmmetError::InvalidAttribute);
        }

        loop {
            self.skip_whitespace();

            if self.peek() == Some(']') {
                self.consume_char();
                break;
            }

            let name = self.parse_identifier()?;
            let value = if self.peek() == Some('=') {
                self.consume_char();
                Some(self.parse_attribute_value()?)
            } else {
                None
            };

            attributes.push(Attribute { name, value });

            self.skip_whitespace();

            if self.peek() == Some(',') {
                self.consume_char();
            }
        }

        Ok(attributes)
    }

    fn parse_attribute_value(&mut self) -> Result<String, EmmetError> {
        if self.peek() == Some('"') {
            self.consume_char();
            let start = self.position;

            while self.position < self.input.len() {
                if self.peek() == Some('"') {
                    self.consume_char();
                    return Ok(self.input[start..self.position - 1].to_string());
                }
                self.position += 1;
            }

            Err(EmmetError::UnclosedBracket)
        } else {
            let start = self.position;

            while self.position < self.input.len() {
                let ch = self.input.chars().nth(self.position).unwrap();
                if ch.is_alphanumeric() || ch == '-' || ch == '_' || ch == ' ' || ch == '.' {
                    self.position += 1;
                } else {
                    break;
                }
            }

            if self.position > start {
                Ok(self.input[start..self.position].to_string())
            } else {
                Err(EmmetError::InvalidAttribute)
            }
        }
    }

    fn parse_text_content(&mut self) -> Result<String, EmmetError> {
        if self.consume_char() != Some('{') {
            return Err(EmmetError::InvalidSyntax("Expected '{'".to_string()));
        }

        let start = self.position;

        while self.position < self.input.len() {
            if self.peek() == Some('}') {
                self.consume_char();
                return Ok(self.input[start..self.position - 1].to_string());
            }
            self.position += 1;
        }

        Err(EmmetError::UnclosedBracket)
    }

    fn parse_number(&mut self) -> Result<u32, EmmetError> {
        let start = self.position;

        while self.position < self.input.len() {
            let ch = self.input.chars().nth(self.position).unwrap();
            if ch.is_numeric() {
                self.position += 1;
            } else {
                break;
            }
        }

        if self.position > start {
            self.input[start..self.position]
                .parse()
                .map_err(|_| EmmetError::InvalidSyntax("Invalid number".to_string()))
        } else {
            Err(EmmetError::InvalidSyntax("Expected number".to_string()))
        }
    }

    fn parse_children(&mut self) -> Result<Vec<EmmetElement>, EmmetError> {
        let mut children = Vec::new();

        while self.position < self.input.len() {
            self.skip_whitespace();

            if self.peek().is_none() || self.peek() == Some('+') {
                break;
            }

            let child = self.parse_element()?;
            children.push(child);
        }

        Ok(children)
    }

    fn peek(&self) -> Option<char> {
        if self.position < self.input.len() {
            self.input.chars().nth(self.position)
        } else {
            None
        }
    }

    fn consume_char(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let ch = self.input.chars().nth(self.position).unwrap();
            self.position += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            let ch = self.input.chars().nth(self.position).unwrap();
            if ch.is_whitespace() {
                self.position += 1;
            } else {
                break;
            }
        }
    }
}

pub fn parse_emmet(input: &str) -> Result<Vec<EmmetElement>, EmmetError> {
    let mut parser = EmmetParser::new(input);
    parser.parse()
}

pub fn emmet_to_html(input: &str) -> Result<String, EmmetError> {
    let elements = parse_emmet(input)?;
    let html: String = elements.iter().map(|e| e.to_html()).collect();
    Ok(html)
}

#[cfg(test)]
mod tests;

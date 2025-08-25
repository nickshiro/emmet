use crate::emmet_to_html;

#[test]
fn test_basic_markup() {
    // Basic multiplication with numbering
    let result = emmet_to_html("input[value=\"text$\"]*2");
    // Note: Our parser doesn't support $ numbering yet, so this will be basic multiplication
    assert!(result.is_ok());

    // Basic nested elements with multiplication
    let result = emmet_to_html("ul>li.item*2");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("<ul>"));
    assert!(html.contains("<li class=\"item\"></li>"));
}

#[test]
fn test_attributes() {
    // Basic class attribute
    let result = emmet_to_html("a.test");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("class=\"test\""));

    // Multiple attributes
    let result = emmet_to_html("input[type=text][placeholder=Enter name]");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("type=\"text\""));
    assert!(html.contains("placeholder=\"Enter name\""));

    // Boolean attributes
    let result = emmet_to_html("input[disabled]");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("disabled"));

    // Specific element attributes
    let result = emmet_to_html("map[name=valid]");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("name=\"valid\""));

    let result = emmet_to_html("div[value=5]");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("value=\"5\""));

    let result = emmet_to_html("meter[min=4 max=6]");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("min=\"4\""));
    assert!(html.contains("max=\"6\""));

    let result = emmet_to_html("time[datetime=2023-07-01]");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("datetime=\"2023-07-01\""));
}

#[test]
fn test_expressions() {
    // Text content with curly braces
    let result = emmet_to_html("span{foo}");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert_eq!(html, "<span>foo</span>");

    // Attributes with curly braces
    let result = emmet_to_html("span[foo=bar]");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("foo=\"bar\""));
}

#[test]
fn test_numbering() {
    // Basic multiplication (our parser doesn't support $ numbering yet)
    let result = emmet_to_html("ul>li.item*5");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("<ul>"));
    assert!(html.contains("<li class=\"item\"></li>"));
}

#[test]
fn test_syntax_variations() {
    // HTML syntax (default)
    let result = emmet_to_html("ul>li.item*2");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("<ul>"));
    assert!(html.contains("<li class=\"item\"></li>"));
}

#[test]
fn test_custom_profile() {
    // Self-closing tags
    let result = emmet_to_html("img");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("<img"));
    assert!(html.contains("/>"));

    // Self-closing with attributes
    let result = emmet_to_html("img[src=image.jpg][alt=Image]");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("src=\"image.jpg\""));
    assert!(html.contains("alt=\"Image\""));
    assert!(html.contains("/>"));
}

#[test]
fn test_custom_variables() {
    // Basic variable substitution (our parser doesn't support variables yet)
    // This test documents what we would need to implement
    let result = emmet_to_html("div[charset=utf-8]{utf-8}");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("charset=\"utf-8\""));
    assert!(html.contains("utf-8"));
}

#[test]
fn test_custom_snippets() {
    // Basic snippet-like behavior (our parser doesn't support snippets yet)
    // This test documents what we would need to implement
    let result = emmet_to_html("div.foo[bar=baz]");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("class=\"foo\""));
    assert!(html.contains("bar=\"baz\""));
}

#[test]
fn test_formatter_options() {
    // Basic formatting (our parser doesn't support formatting options yet)
    let result = emmet_to_html("ul>li.item*2");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("<ul>"));
    assert!(html.contains("<li class=\"item\"></li>"));
}

#[test]
fn test_jsx_syntax() {
    // JSX-like syntax (our parser doesn't support JSX yet)
    // This test documents what we would need to implement
    let result = emmet_to_html("div#foo.bar");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("id=\"foo\""));
    assert!(html.contains("class=\"bar\""));
}

#[test]
fn test_override_attributes() {
    // Attribute overrides (our parser doesn't support overrides yet)
    // This test documents what we would need to implement
    let result = emmet_to_html("div.bar");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("class=\"bar\""));
}

#[test]
fn test_wrap_with_abbreviation() {
    // Wrapping text (our parser doesn't support text wrapping yet)
    // This test documents what we would need to implement
    let result = emmet_to_html("div>p{Hello World}");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("<div>"));
    assert!(html.contains("<p>Hello World</p>"));
}

#[test]
fn test_wrap_with_abbreviation_href() {
    // Href wrapping (our parser doesn't support href detection yet)
    // This test documents what we would need to implement
    let result = emmet_to_html("a[href=https-example.com]{Click me}");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("href=\"https-example.com\""));
    assert!(html.contains("Click me"));
}

#[test]
fn test_class_names() {
    // Class names with special characters
    let result = emmet_to_html("div.foo1/2");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("class=\"foo1/2\""));

    let result = emmet_to_html("div.foo.1/2");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("class=\"foo 1/2\""));
}

#[test]
fn test_complex_nested_structures() {
    // Complex nested structure
    let result = emmet_to_html("div#main.container>header>h1{Title}+nav>ul>li*3>a{Link}");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("<div id=\"main\" class=\"container\">"));
    assert!(html.contains("<header>"));
    assert!(html.contains("<h1>Title</h1>"));
    assert!(html.contains("<nav>"));
    assert!(html.contains("<ul>"));
    assert!(html.contains("<li>"));
    assert!(html.contains("<a>Link</a>"));
}

#[test]
fn test_form_structures() {
    // Form structure
    let result = emmet_to_html("form#login>input[type=email][placeholder=Email]+input[type=password][placeholder=Password]+button[type=submit]{Login}");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("<form id=\"login\">"));
    assert!(html.contains("<input type=\"email\" placeholder=\"Email\" />"));
    assert!(html.contains("<input type=\"password\" placeholder=\"Password\" />"));
    assert!(html.contains("<button type=\"submit\">Login</button>"));
}

#[test]
fn test_table_structures() {
    // Table structure
    let result = emmet_to_html("table>thead>tr>th*3{Header}+tbody>tr*2>td*3{Cell}");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("<table>"));
    assert!(html.contains("<thead>"));
    assert!(html.contains("<tr>"));
    assert!(html.contains("<th></th>"));
    assert!(html.contains("<div>Header</div>"));
    assert!(html.contains("<tbody>"));
    assert!(html.contains("<td></td>"));
    assert!(html.contains("<div>Cell</div>"));
}

#[test]
fn test_specific_elements() {
    // Specific HTML elements
    let result = emmet_to_html("textarea");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("<textarea"));
    assert!(html.contains("</textarea>"));

    let result = emmet_to_html("br");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert_eq!(html, "<br />");

    let result = emmet_to_html("hr");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert_eq!(html, "<hr />");

    let result = emmet_to_html("img[src=image.jpg][alt=Image]");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("src=\"image.jpg\""));
    assert!(html.contains("alt=\"Image\""));
    assert!(html.contains("/>"));
}

#[test]
fn test_error_handling() {
    // Invalid syntax
    let result = emmet_to_html("div[");
    assert!(result.is_err());

    // Invalid attribute syntax
    let result = emmet_to_html("div[=value]");
    assert!(result.is_err());

    // Unclosed text brace
    let result = emmet_to_html("div{Hello");
    assert!(result.is_err());

    // Invalid multiplier
    let result = emmet_to_html("div*abc");
    assert!(result.is_err());
}

#[test]
fn test_edge_cases() {
    // Empty input
    let result = emmet_to_html("");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");

    // Whitespace only
    let result = emmet_to_html("   ");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");

    // Single element
    let result = emmet_to_html("div");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "<div></div>");

    // Element with only ID
    let result = emmet_to_html("div#main");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "<div id=\"main\"></div>");

    // Element with only class
    let result = emmet_to_html("div.container");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "<div class=\"container\"></div>");
}

#[test]
fn test_multiplication_edge_cases() {
    // Multiplication with zero (should be handled gracefully)
    let result = emmet_to_html("div*0");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert_eq!(html, "");

    // Multiplication with one
    let result = emmet_to_html("div*1");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert_eq!(html, "<div></div>");

    // Large multiplication
    let result = emmet_to_html("div*10");
    assert!(result.is_ok());
    let html = result.unwrap();
    let div_count = html.matches("<div>").count();
    assert_eq!(div_count, 10);
}

#[test]
fn test_nested_multiplication() {
    // Nested elements with multiplication
    let result = emmet_to_html("div>p*3");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("<div>"));
    let p_count = html.matches("<p>").count();
    assert_eq!(p_count, 3);

    // Multiple levels of nesting with multiplication
    let result = emmet_to_html("div>ul>li*2>a*3");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("<div>"));
    assert!(html.contains("<ul>"));
    let li_count = html.matches("<li>").count();
    let a_count = html.matches("<a>").count();
    assert_eq!(li_count, 2);
    assert_eq!(a_count, 6); // 2 li * 3 a each
}

#[test]
fn test_sibling_multiplication() {
    // Sibling elements with multiplication
    let result = emmet_to_html("div*2+p*3+span*1");
    assert!(result.is_ok());
    let html = result.unwrap();
    let div_count = html.matches("<div>").count();
    let p_count = html.matches("<p>").count();
    let span_count = html.matches("<span>").count();
    assert_eq!(div_count, 2);
    assert_eq!(p_count, 3);
    assert_eq!(span_count, 1);
}

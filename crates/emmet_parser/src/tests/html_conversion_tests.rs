use crate::{emmet_to_html, EmmetError};

#[test]
fn test_basic_div_to_html() {
    let html = emmet_to_html("div").unwrap();
    assert_eq!(html, "<div></div>");
}

#[test]
fn test_div_with_id_to_html() {
    let html = emmet_to_html("div#main").unwrap();
    assert_eq!(html, "<div id=\"main\"></div>");
}

#[test]
fn test_div_with_class_to_html() {
    let html = emmet_to_html("div.container").unwrap();
    assert_eq!(html, "<div class=\"container\"></div>");
}

#[test]
fn test_div_with_id_and_class_to_html() {
    let html = emmet_to_html("div#main.container").unwrap();
    assert_eq!(html, "<div id=\"main\" class=\"container\"></div>");
}

#[test]
fn test_div_with_multiple_classes_to_html() {
    let html = emmet_to_html("div.container.fluid").unwrap();
    assert_eq!(html, "<div class=\"container fluid\"></div>");
}

#[test]
fn test_input_with_attributes_to_html() {
    let html = emmet_to_html("input[type=text][placeholder=Enter name]").unwrap();
    assert_eq!(html, "<input type=\"text\" placeholder=\"Enter name\" />");
}

#[test]
fn test_input_with_single_attribute_to_html() {
    let html = emmet_to_html("input[type=text]").unwrap();
    assert_eq!(html, "<input type=\"text\" />");
}

#[test]
fn test_input_with_boolean_attribute_to_html() {
    let html = emmet_to_html("input[disabled]").unwrap();
    assert_eq!(html, "<input disabled />");
}

#[test]
fn test_div_with_text_to_html() {
    let html = emmet_to_html("div{Hello World}").unwrap();
    assert_eq!(html, "<div>Hello World</div>");
}

#[test]
fn test_div_with_text_and_special_chars_to_html() {
    let html = emmet_to_html("div{Hello & World < 10}").unwrap();
    assert_eq!(html, "<div>Hello & World < 10</div>");
}

#[test]
fn test_div_with_multiplier_to_html() {
    let html = emmet_to_html("div*3").unwrap();
    assert_eq!(html, "<div></div><div></div><div></div>");
}

#[test]
fn test_div_with_id_and_multiplier_to_html() {
    let html = emmet_to_html("div#main*2").unwrap();
    assert_eq!(html, "<div id=\"main\"></div><div id=\"main\"></div>");
}

#[test]
fn test_nested_elements_to_html() {
    let html = emmet_to_html("div>p>span").unwrap();
    assert_eq!(html, "<div><p><span></span></p></div>");
}

#[test]
fn test_nested_elements_with_text_to_html() {
    let html = emmet_to_html("div>p{Hello}>span{World}").unwrap();
    assert_eq!(html, "<div><p>Hello<span>World</span></p></div>");
}

#[test]
fn test_sibling_elements_to_html() {
    let html = emmet_to_html("div+p+span").unwrap();
    assert_eq!(html, "<div></div><p></p><span></span>");
}

#[test]
fn test_sibling_elements_with_attributes_to_html() {
    let html = emmet_to_html("div#main+p.container+span").unwrap();
    assert_eq!(
        html,
        "<div id=\"main\"></div><p class=\"container\"></p><span></span>"
    );
}

#[test]
fn test_self_closing_tags_to_html() {
    let html = emmet_to_html("img[src=image.jpg][alt=Image]").unwrap();
    assert_eq!(html, "<img src=\"image.jpg\" alt=\"Image\" />");
}

#[test]
fn test_br_tag_to_html() {
    let html = emmet_to_html("br").unwrap();
    assert_eq!(html, "<br />");
}

#[test]
fn test_hr_tag_to_html() {
    let html = emmet_to_html("hr").unwrap();
    assert_eq!(html, "<hr />");
}

#[test]
fn test_complex_nested_structure_to_html() {
    let html = emmet_to_html("div#main.container>header>h1{Title}+nav>ul>li*3>a{Link}").unwrap();
    assert!(html.contains("<div id=\"main\" class=\"container\">"));
    assert!(html.contains("<header>"));
    assert!(html.contains("<h1>Title</h1>"));
    assert!(html.contains("<nav>"));
    assert!(html.contains("<ul>"));
    assert!(html.contains("<li>"));
    assert!(html.contains("<a>Link</a>"));
}

#[test]
fn test_form_structure_to_html() {
    let html = emmet_to_html("form#login>input[type=email][placeholder=Email]+input[type=password][placeholder=Password]+button[type=submit]{Login}").unwrap();
    assert!(html.contains("<form id=\"login\">"));
    assert!(html.contains("<input type=\"email\" placeholder=\"Email\" />"));
    assert!(html.contains("<input type=\"password\" placeholder=\"Password\" />"));
    assert!(html.contains("<button type=\"submit\">Login</button>"));
}

#[test]
fn test_table_structure_to_html() {
    let html = emmet_to_html("table>thead>tr>th*3{Header}+tbody>tr*2>td*3{Cell}").unwrap();
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
fn test_empty_input() {
    let result = emmet_to_html("");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_whitespace_only_input() {
    let result = emmet_to_html("   ");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_invalid_syntax() {
    let result = emmet_to_html("div[");
    assert!(result.is_err());
    match result {
        Err(EmmetError::InvalidSyntax(_)) => {}
        _ => panic!("Expected InvalidSyntax error"),
    }
}

#[test]
fn test_invalid_attribute_syntax() {
    let result = emmet_to_html("div[=value]");
    assert!(result.is_err());
}

#[test]
fn test_unclosed_text_brace() {
    let result = emmet_to_html("div{Hello");
    assert!(result.is_err());
    match result {
        Err(EmmetError::UnclosedBracket) => {}
        _ => panic!("Expected UnclosedBracket error"),
    }
}

#[test]
fn test_invalid_multiplier() {
    let result = emmet_to_html("div*abc");
    assert!(result.is_err());
}

use crate::{emmet_to_html, parse_emmet};

#[test]
fn test_basic_element() {
    let result = parse_emmet("div").unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].tag, "div");
}

#[test]
fn test_element_with_id() {
    let result = parse_emmet("div#main").unwrap();
    assert_eq!(result[0].id, Some("main".to_string()));
}

#[test]
fn test_element_with_class() {
    let result = parse_emmet("div.container").unwrap();
    assert_eq!(result[0].classes, vec!["container"]);
}

#[test]
fn test_element_with_multiple_classes() {
    let result = parse_emmet("div.container.fluid").unwrap();
    assert_eq!(result[0].classes, vec!["container", "fluid"]);
}

#[test]
fn test_element_with_id_and_class() {
    let result = parse_emmet("div#main.container").unwrap();
    assert_eq!(result[0].id, Some("main".to_string()));
    assert_eq!(result[0].classes, vec!["container"]);
}

#[test]
fn test_element_with_attributes() {
    let result = parse_emmet("input[type=text][placeholder=Enter name]").unwrap();
    assert_eq!(result[0].attributes.len(), 2);
    assert_eq!(result[0].attributes[0].name, "type");
    assert_eq!(result[0].attributes[0].value, Some("text".to_string()));
}

#[test]
fn test_element_with_text() {
    let result = parse_emmet("div{Hello World}").unwrap();
    assert_eq!(result[0].text, Some("Hello World".to_string()));
}

#[test]
fn test_element_with_multiplier() {
    let result = parse_emmet("div*3").unwrap();
    assert_eq!(result[0].multiplier, Some(3));
}

#[test]
fn test_nested_elements() {
    let result = parse_emmet("div>p>span").unwrap();
    assert_eq!(result[0].tag, "div");
    assert_eq!(result[0].children.len(), 1);
    assert_eq!(result[0].children[0].tag, "p");
    assert_eq!(result[0].children[0].children.len(), 1);
    assert_eq!(result[0].children[0].children[0].tag, "span");
}

#[test]
fn test_sibling_elements() {
    let result = parse_emmet("div+p+span").unwrap();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].tag, "div");
    assert_eq!(result[1].tag, "p");
    assert_eq!(result[2].tag, "span");
}

#[test]
fn test_complex_example() {
    let html = emmet_to_html("div#main.container>p{Hello}+span.world*2").unwrap();
    assert!(html.contains("id=\"main\""));
    assert!(html.contains("class=\"container\""));
    assert!(html.contains("Hello"));
    assert!(html.contains("class=\"world\""));
}

use emmet_parser::emmet_to_html;

fn main() {
    println!("Emmet to HTML Conversion Examples\n");
    println!("=================================\n");

    let examples = vec![
        ("Basic element", "div"),
        ("Element with ID", "div#main"),
        ("Element with class", "div.container"),
        ("Element with ID and class", "div#main.container"),
        ("Element with multiple classes", "div.container.fluid"),
        ("Input with attributes", "input[type=text][placeholder=Enter name]"),
        ("Element with text", "div{Hello World}"),
        ("Element with multiplication", "div*3"),
        ("Nested elements", "div>p>span"),
        ("Sibling elements", "div+p+span"),
        ("Self-closing tag", "img[src=image.jpg][alt=Image]"),
        ("Complex form", "form#login>input[type=email][placeholder=Email]+input[type=password][placeholder=Password]+button[type=submit]{Login}"),
        ("Table structure", "table>thead>tr>th*3{Header}+tbody>tr*2>td*3{Cell}"),
    ];

    for (description, emmet) in examples {
        println!("{}:", description);
        println!("  Emmet: {}", emmet);

        match emmet_to_html(emmet) {
            Ok(html) => {
                println!("  HTML:  {}", html);
            }
            Err(e) => {
                println!("  Error: {}", e);
            }
        }
        println!();
    }

    // Error handling example
    println!("Error Handling Examples:");
    println!("=======================\n");

    let error_examples = vec![
        ("Unclosed bracket", "div["),
        ("Invalid attribute", "div[=value]"),
        ("Unclosed text brace", "div{Hello"),
        ("Invalid multiplier", "div*abc"),
    ];

    for (description, emmet) in error_examples {
        println!("{}:", description);
        println!("  Emmet: {}", emmet);

        match emmet_to_html(emmet) {
            Ok(html) => {
                println!("  HTML:  {}", html);
            }
            Err(e) => {
                println!("  Error: {}", e);
            }
        }
        println!();
    }
}

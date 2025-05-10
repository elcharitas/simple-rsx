fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use simple_rsx::*;

    #[test]
    fn test_empty_fragment() {
        let rsx = rsx!(<></>);
        let expected = String::from("");
        let result = rsx.to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_fragment_with_text() {
        let rsx = rsx!(<>"Hello World"</>);
        let expected = String::from("Hello World");
        let result = rsx.to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_self_closing_tag() {
        let rsx = rsx!(<div class="container" id="app" />);
        let expected = String::from("<div class=\"container\" id=\"app\"></div>");
        let result = rsx.to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_elements() {
        let rsx = rsx!(
            <div class="container">
                <h1>Title</h1>
                <p>Paragraph text</p>
            </div>
        );
        let expected =
            String::from("<div class=\"container\"><h1>Title</h1><p>Paragraph text</p></div>");
        let result = rsx.to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_expression_interpolation() {
        let name = "World";
        let rsx = rsx!(<div>Hello {name}</div>);
        let expected = String::from("<div>Hello World</div>");
        let result = rsx.to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_attributes() {
        let rsx = rsx!(<input type="text" placeholder="Enter name" required="true" />);
        let expected = String::from(
            "<input type=\"text\" placeholder=\"Enter name\" required=\"true\"></input>",
        );
        let result = rsx.to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mixed_content() {
        let count = 42;
        let rsx = rsx!(
            <div class="mixed">
                <h1>Count: {count}</h1>
                <p>"Static text"</p>
                <>Fragment inside</>
            </div>
        );
        let expected = String::from(
            "<div class=\"mixed\"><h1>Count: 42</h1><p>Static text</p>Fragment inside</div>",
        );
        let result = rsx.to_string();
        assert_eq!(result, expected);
    }
}

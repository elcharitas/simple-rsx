#[allow(unused_braces)]
fn main() {
    use simple_rsx::*;
    let rsx = rsx!(<div>Hello World</div>);
    println!("{}", rsx.to_string());
}

#[cfg(test)]
#[allow(unused_braces)]
mod tests {
    #[test]
    fn test_basic_rsx() {
        use simple_rsx::*;
        let rsx = rsx!(<></>);
        assert!(rsx.to_string().is_empty())
    }

    #[test]
    fn test_text_rsx() {
        use simple_rsx::*;
        let rsx = rsx!(<>"Hello World"</>);
        assert_eq!(rsx.to_string(), "Hello World")
    }

    #[test]
    fn test_rsx_comment() {
        use simple_rsx::*;
        let rsx = rsx!(<!-- "This is a comment" -->);
        assert_eq!(rsx.to_string(), "") // TODO: emit comments in the future, <!-- This is a comment -->
    }

    #[test]
    fn test_div_rsx() {
        use simple_rsx::*;
        let rsx = rsx!(<div class="container" id="app" />);
        assert_eq!(
            rsx.to_string(),
            "<div class=\"container\" id=\"app\"></div>"
        )
    }

    #[test]
    fn test_div_children_rsx() {
        use simple_rsx::*;
        let rsx = rsx!(
            <div class="container">
                <h1>Title</h1>
                <p>Paragraph text</p>
            </div>
        );
        assert_eq!(
            rsx.to_string(),
            "<div class=\"container\"><h1>Title</h1><p>Paragraph text</p></div>"
        )
    }

    #[test]
    fn test_div_children_rsx_with_text() {
        use simple_rsx::*;
        let name = "World";
        let rsx = rsx!(<div>Hello: {name}</div>);
        assert_eq!(rsx.to_string(), "<div>Hello: World</div>")
    }

    #[test]
    fn test_div_children_rsx_with_text_and_attribute() {
        use simple_rsx::*;
        let rsx = rsx!(<input r#type="text" placeholder="Enter name" required="true" />);
        assert_eq!(
            rsx.to_string(),
            "<input type=\"text\" placeholder=\"Enter name\" required=\"true\"></input>"
        )
    }

    #[test]
    fn test_div_children_rsx_with_text_and_attribute_and_fragment() {
        use simple_rsx::*;
        let count = 42;
        let rsx = rsx!(
            <div class="mixed">
                <h1>Count: {count}</h1>
                <p>Static text</p>
                <>Fragment inside</>
            </div>
        );
        assert_eq!(
            rsx.to_string(),
            "<div class=\"mixed\"><h1>Count: 42</h1><p>Static text</p>Fragment inside</div>"
        )
    }

    #[test]
    fn test_div_children_rsx_with_if() {
        use simple_rsx::*;
        let show = true;
        let rsx = rsx!(
            <div>
                {if show {
                    rsx!(<p>Show me</p>)
                } else {
                    rsx!(<p>Hide me</p>)
                }}
            </div>
        );
        assert_eq!(rsx.to_string(), "<div><p>Show me</p></div>")
    }

    #[test]
    fn test_rsx_looping() {
        use simple_rsx::*;
        let items = vec!["Item 1", "Item 2", "Item 3"];
        let list = rsx!(
            <ul>
                {items.iter().map(|item| rsx!(<li>{item}</li>))}
            </ul>
        );
        assert_eq!(
            list.to_string(),
            "<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>"
        )
    }

    #[test]
    fn test_rsx_looping_with_index() {
        use simple_rsx::*;
        let items = vec!["Item 1", "Item 2", "Item 3"];
        let list = rsx!(
            <ul>
                {items.iter().enumerate().map(|(index, item)| rsx!(<li key={index}>{item}</li>))}
            </ul>
        );
        assert_eq!(
            list.to_string(),
            "<ul><li key=\"0\">Item 1</li><li key=\"1\">Item 2</li><li key=\"2\">Item 3</li></ul>"
        )
    }

    #[test]
    fn test_attribute_value_expression() {
        use simple_rsx::*;
        let class = "container";
        let rsx = rsx!(<div class={format!("{class}-large")} />);
        assert_eq!(rsx.to_string(), "<div class=\"container-large\"></div>")
    }

    #[test]
    fn test_attribute_value_concatenation_and_interpolation() {
        use simple_rsx::*;
        let class = "container";
        let rsx = rsx!(<div class="{class}-large" />);
        assert_eq!(rsx.to_string(), "<div class=\"container-large\"></div>")
    }

    #[test]
    fn test_component_rendering() {
        use simple_rsx::*;
        fn some_component() -> Node {
            rsx!(<div>Some component</div>)
        }
        let rsx = rsx!(
            <div>
                {some_component()}
            </div>
        );
        assert_eq!(rsx.to_string(), "<div><div>Some component</div></div>")
    }
}

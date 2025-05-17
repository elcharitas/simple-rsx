#[allow(unused_braces)]
fn main() {
    use simple_rsx::*;
    let rsx = rsx! {
        <div> Hello don&apos;t You know me? {"World"}</div>
    };
    println!("{}", rsx);
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
        let rsx = rsx!(<!-- This is a comment -->);
        assert_eq!(rsx.to_string(), "<!-- This is a comment -->")
    }

    #[test]
    fn test_div_rsx() {
        use simple_rsx::*;
        let rsx = rsx!(<div class="container" id="app" />);
        assert_eq!(
            rsx.to_string(),
            "<div id=\"app\" class=\"container\"></div>"
        )
    }

    #[test]
    fn test_div_children_rsx() {
        use simple_rsx::rsx;

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
        assert_eq!(rsx.to_string(), "<div>Hello:World</div>") // expressions don't preserve whitespace
    }

    #[test]
    fn test_div_children_rsx_with_text_and_attribute() {
        use simple_rsx::*;
        let rsx = rsx!(<input type_="text" placeholder="Enter name" required />);
        assert_eq!(
            rsx.to_string(),
            "<input type=\"text\" placeholder=\"Enter name\" required=\"true\" disabled=\"false\" readonly=\"false\"></input>"
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
                <>"Fragment inside"</>
            </div>
        );
        assert_eq!(
            rsx.to_string(),
            "<div class=\"mixed\"><h1>Count:42</h1><p>Static text</p>Fragment inside</div>"
        )
    }

    #[test]
    fn test_div_children_rsx_with_if() {
        use simple_rsx::*;
        let show = true;
        let rsx = rsx!(
            <div>
                {either!(show => <p>Show me</p>)}
            </div>
        );
        assert_eq!(rsx.to_string(), "<div><p>Show me</p></div>")
    }

    #[test]
    fn test_rsx_looping() {
        use simple_rsx::*;
        let items = ["Item 1", "Item 2", "Item 3"];
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
        let items = ["Item 1", "Item 2", "Item 3"];
        let list = rsx!(
            <ul>
                {items.iter().enumerate().map(|(index, item)| rsx!(<li key={index.to_string()}>{item}</li>))}
            </ul>
        );
        assert_eq!(
            list.to_string(),
            "<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>"
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

    #[test]
    fn test_component_rendering_with_props() {
        use simple_rsx::*;

        struct MyComponent;

        struct Props {
            message: String,
            children: Vec<Node>, // always required in components
        }

        impl Component for MyComponent {
            type Props = Props;
            fn render(props: Self::Props) -> Node {
                println!("{}", props.message);
                rsx!(<div>{props.children}</div>)
            }
        }

        let rsx = rsx!(
            <div>
                <MyComponent message="">
                    <p>Some component</p>
                </MyComponent>
            </div>
        );
        assert_eq!(
            rsx.to_string(),
            "<div><div><p>Some component</p></div></div>"
        )
    }

    #[test]
    fn test_fn_component_rendering_with_props() {
        use simple_rsx::*;

        struct Props {
            message: String,
            children: Vec<Node>, // always required in components
        }

        #[component]
        fn MyComponent(props: Props) -> Node {
            println!("{}", props.message);
            rsx!(<div>{props.children}</div>)
        }

        let rsx = rsx!(
            <div>
                <MyComponent message="Hello from mars">
                    <p>Some component</p>
                </MyComponent>
            </div>
        );
        assert_eq!(
            rsx.to_string(),
            "<div><div><p>Some component</p></div></div>"
        )
    }

    #[test]
    fn test_attribute_binding() {
        use simple_rsx::*;
        let disabled = true;
        let rsx = rsx!(<button {disabled} />); // notice how we don't need to use assignment?
        assert_eq!(rsx.to_string(), "<button disabled=\"true\"></button>")
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use simple_rsx::*;

    #[test]
    fn test_jsx() {
        let jsx = jsx!(<div class="container" id="app" />);
        let expected = String::from("<div class=\"container\" id=\"app\"></div>");
        let result = jsx.to_string();
        assert_eq!(result, expected); // Replace with your expected result
    }

    #[test]
    fn test_jsx_with_children() {
        let jsx = jsx!(<div class="container" id="app"><p>"Hello, world!"</p></div>);
        let expected =
            String::from("<div class=\"container\" id=\"app\"><p>Hello, world!</p></div>");
        let result = jsx.to_string();
        assert_eq!(result, expected); // Replace with your expected result
    }
}

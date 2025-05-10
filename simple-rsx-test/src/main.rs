fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use simple_rsx::*;

    #[test]
    fn test_rsx() {
        let rsx = rsx!(<div class="container" id="app" />);
        let expected = String::from("<div class=\"container\" id=\"app\"></div>");
        let result = rsx.to_string();
        assert_eq!(result, expected); // Replace with your expected result
    }

    #[test]
    fn test_rsx_with_children() {
        let rsx = rsx!(<div class="container" id="app"><p>"Hello, world!"</p></div>);
        let expected =
            String::from("<div class=\"container\" id=\"app\"><p>Hello, world!</p></div>");
        let result = rsx.to_string();
        assert_eq!(result, expected); // Replace with your expected result
    }
}

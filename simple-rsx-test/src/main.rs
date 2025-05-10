fn main() {
    println!("Hello, world!");
    use simple_rsx::*;

    let rsx = rsx!(<></>);
    println!("result: {}", rsx);

    let rsx = rsx!(<>"Hello World"</>);
    println!("result: {}", rsx);

    let rsx = rsx!(<div class="container" id="app" />);
    println!("result: {}", rsx);

    let rsx = rsx!(
        <div class="container">
            <h1>Title</h1>
            <p>Paragraph text</p>
        </div>
    );
    println!("result: {}", rsx);

    let name = "World";
    let rsx = rsx!(<div>Hello: {name}</div>);
    println!("result: {}", rsx);

    let rsx = rsx!(<input r#type="text" placeholder="Enter name" required="true" />);
    println!("result: {}", rsx);

    //        let count = 42;
    //        let rsx = rsx!(
    //            <div class="mixed">
    //                <h1>Count: {count}</h1>
    //                <p>"Static text"</p>
    //                <>Fragment inside</>
    //            </div>
    //        );
    //        let expected = String::from(
    //            "<div class=\"mixed\"><h1>Count: 42</h1><p>Static text</p>Fragment inside</div>",
    //        );
    //        let result = rsx.to_string();
    //        assert_eq!(result, expected);
}

#[allow(unused_braces)]
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

    let count = 42;
    let rsx = rsx!(
        <div class="mixed">
            <h1>Count: {count}</h1>
            <p>Static text</p>
            <>Fragment inside</>
        </div>
    );
    println!("result: {}", rsx);

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
    println!("result: {}", rsx);

    // Component rendering, using a function
    fn some_component() -> Node {
        rsx!(<div>Some component</div>)
    }
    let rsx = rsx!(
        <div>
            {some_component()}
        </div>
    );
    println!("result: {}", rsx);

    // List rendering
    let items = vec!["Item 1", "Item 2", "Item 3"];
    let list = rsx!(
        <ul>
            {items.iter().map(|item| rsx!(<li>{item}</li>))}
        </ul>
    );
    println!("result: {}", list);

    // List rendering with index
    let items = vec!["Item 1", "Item 2", "Item 3"];
    let list = rsx!(
        <ul>
            {items.iter().enumerate().map(|(index, item)| rsx!(<li key={index}>{item}</li>))}
        </ul>
    );
    println!("result: {}", list);

    // attribute value concatenation and interpolation
    let class = "container";
    let rsx = rsx!(<div class={format!("{class}-large")} />);
    println!("result: {}", rsx);
    // attribute value concatenation and interpolation
    let class = "container";
    let rsx = rsx!(<div class="{class}-large" />);
    println!("result: {}", rsx);

    // looping, using for loop in block
    let mut count = 0;
    let rsx = rsx!(
        <div>
            {
                let mut output = Vec::new();
                for _ in 0..5 {
                    count += 1;
                    output.push(rsx!(<p>Count: {count}</p>));
                }
                output
            }
        </div>
    );
    println!("result: {}", rsx);

    // looping, ergonomic using map
    let count = 0;
    let rsx = rsx!(
        <div>
            {(0..5).map(|_| count.into())}
        </div>
    );
    println!("result: {}", rsx);
}

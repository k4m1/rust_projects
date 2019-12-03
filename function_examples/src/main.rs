fn main() {
    println!("Hello, world!");

    example_function();
    example_with_params(69);
    // exmaple_with_multiparams(42, "howdy");
    let t = example_with_return_label();
    println!("the value of t is {}", t);
}

fn example_function() {
    println!("TFOT That function over there");
}

fn example_with_params(x: i32) {
    println!("the value of x is {}", x);
}

// fn exmaple_with_multiparams(x: i32, y: String) {
//     println!("the value of x is {} the string y is {}", x, y);
// }

fn example_with_return_label() -> i32 {
    420
}




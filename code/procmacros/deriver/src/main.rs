use deriver_macros::HelloMacro;

#[derive(HelloMacro)]
struct MyData {
    name: String,
}

fn main() {
    let person = MyData {
        name: "Alice".to_string(),
    };
    person.hello_macro();
}

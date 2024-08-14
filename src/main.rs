mod oml_value;
mod string_utils;

pub use oml_value::OmlValue;

fn main() {
    let oml_str = r#"
[hello]
value = 12
name = $"hello {123} world"
"#;
    let root = match OmlValue::from_str(oml_str) {
        Ok(root) => root,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };
    println!("Success");
    println!("hello.name = {}", root["hello"]["name"].as_str());
}

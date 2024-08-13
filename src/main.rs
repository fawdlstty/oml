mod ast;
mod string_utils;

pub use ast::OmlValue;

fn main() {
    let oml_str = r#"
[hello]
value = 12
name = $"hello {2+2}"
"#;
    let root = match OmlValue::from_str(oml_str) {
        Ok(root) => root,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };
    println!("Success: {:?}", root);
}

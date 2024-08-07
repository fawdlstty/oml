use ast::OmlValue;

pub mod ast;

fn main() {
    let oml_str = r#"
[hello]
value = 12
"#;
    match OmlValue::from_str(oml_str) {
        Ok(val) => println!("Success: {:?}", val),
        Err(err) => println!("Error: {}", err),
    }
}

mod ast;
mod eval_utils;
mod string_utils;

pub use ast::oml_expr::OmlExpr;
pub use ast::oml_value::OmlValue;

fn main() {
    let oml_str = r#"
[hello]
value = 12
name = $"hello {value + 12} world"
"#;
    let root = match OmlExpr::from_str(oml_str) {
        Ok(root) => root,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };
    let root = match root.evalute() {
        Ok(root) => root,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };
    println!("hello.name = {}", root["hello"]["name"].as_str());
}

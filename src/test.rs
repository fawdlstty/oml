use crate::OmlExpr;

#[test]
fn test1() {
    let oml_str = r#"
[hello]
value = 12
name = $"hello world {value + 12}"
"#;
    let root = match OmlExpr::from_str(oml_str) {
        Ok(root) => root,
        Err(err) => panic!("Error: {}", err),
    };
    let root = match root.evalute() {
        Ok(root) => root,
        Err(err) => panic!("Error: {}", err),
    };
    assert_eq!(root["hello"]["name"].as_str(), "hello world 24");
}

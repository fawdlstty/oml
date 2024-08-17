# oml

![version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Ffawdlstty%2Foml%2Fmain%2FCargo.toml&query=package.version&label=version)
![status](https://img.shields.io/github/actions/workflow/status/fawdlstty/oml/rust.yml)

Open Markup Language!

## Manual

Install: Run `cargo add oml` in the project directory

```rust
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
```

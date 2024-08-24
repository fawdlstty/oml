# oml

![version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Ffawdlstty%2Foml%2Fmain%2FCargo.toml&query=package.version&label=version)
![status](https://img.shields.io/github/actions/workflow/status/fawdlstty/oml/rust.yml)

English | [简体中文](README.zh_CN.md)

Open Markup Language! A dynamic configuration scripting language that can embed script code in the configuration file to achieve dynamic configuration update.

## Manual

### rust

Install: Run `cargo add oml` in the project directory

```rust
fn main() {
    let oml_str = r#"
[hello]
value = 12
name = $"hello world {value + 12}"
"#;
    let root = match OmlExpr::from_str(oml_str) {
        Ok(root) => root,
        Err(err) => panic!("Error: {}", err),
    };
    root["hello"]["value"].set_int(30);
    let root = match root.evalute() {
        Ok(root) => root,
        Err(err) => panic!("Error: {}", err),
    };
    assert_eq!(root["hello"]["name"].as_str(), "hello world 42");
}
```

### C++

```cpp
#include <iostream>
#include <string>

#include "oml/oml.hpp"
#pragma comment(lib, "ws2_32.lib")
#pragma comment(lib, "ntdll.lib")
#pragma comment(lib, "bcrypt.lib")
#pragma comment(lib, "Userenv.lib")
#pragma comment(lib, "oml.lib")

int main() {
	auto oexpr = oml::OmlExpr::from_str(R"(
[hello]
value = 12
name = $"hello world {value + 12}"
)");
	if (oexpr.index() == 1) {
		auto err = std::get<std::string>(oexpr);
		std::cout << err << std::endl;
		return 0;
	}
	auto expr = std::get<oml::OmlExpr>(oexpr);
	auto ovalue = expr.evalute();
	if (ovalue.index() == 1) {
		auto err = std::get<std::string>(ovalue);
		std::cout << err << std::endl;
		return 0;
	}
	auto value = std::get<oml::OmlValue>(ovalue);
	auto str = value["hello"]["name"].as_str();
	std::cout << str << std::endl;
	return 0;
}
```

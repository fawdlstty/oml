# oml

![version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Ffawdlstty%2Foml%2Fmain%2FCargo.toml&query=package.version&label=version)
![status](https://img.shields.io/github/actions/workflow/status/fawdlstty/oml/rust.yml)

[English](README.md) | 简体中文

Open Markup Language! 一款动态配置脚本语言，可在配置文件里嵌入脚本代码，实现动态更新配置。

## 用户手册

### rust

安装：在项目目录下运行 `cargo add oml`

```rust
fn main() {
    let oml_str = r#"
[hello]
value = 12
name = $"hello world {value + 12}"
"#;
    let mut root = OmlExpr::from_str(oml_str).unwrap();
    root["hello"]["value"].set_int(30);
    let root = root.evalute().unwrap();
    println!("{}", root["hello"]["name"].as_str()); // hello world 42
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
    expr["hello"]["value"].set_int(30);
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

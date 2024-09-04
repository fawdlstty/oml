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
    let mut eroot = OmlExpr::from_str(oml_str).unwrap();
    eroot["hello"]["value"].set_int(30);
    let root = eroot.evalute().unwrap();
    println!("{}", root["hello"]["name"].as_str()); // hello world 42
}
```

### C++

```cpp
#include <iostream>
#include <string>

#include "oml/oml.hpp"
#ifdef _MSC_VER
#pragma comment(lib, "ws2_32.lib")
#pragma comment(lib, "ntdll.lib")
#pragma comment(lib, "bcrypt.lib")
#pragma comment(lib, "Userenv.lib")
#pragma comment(lib, "oml.lib")
#endif

int main() {
	auto oeroot = oml::OmlExpr::from_str(R"(
[hello]
value = 12
name = $"hello world {value + 12}"
)");
	if (oeroot.index() == 1) {
		std::cout << std::get<std::string>(oeroot) << std::endl;
		return 0;
	}
	auto eroot = std::get<oml::OmlExpr>(oeroot);
    eroot["hello"]["value"].set_int(30);
	auto oroot = eroot.evalute();
	if (oroot.index() == 1) {
		std::cout << std::get<std::string>(oroot) << std::endl;
		return 0;
	}
	auto root = std::get<oml::OmlValue>(oroot);
	auto str = root["hello"]["name"].as_str(); // hello world 42
	std::cout << str << std::endl;
	return 0;
}
```

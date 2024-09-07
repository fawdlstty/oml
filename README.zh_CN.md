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
name = $"hello {value + 12}"
"#;
    let mut eroot = OmlExpr::from_str(oml_str).unwrap();
    eroot["hello"]["value"].set_int(30);
    let root = eroot.evalute().unwrap();
    println!("{}", root["hello"]["name"].as_str()); // hello 42
}
```

### C++

下载并编译静态库（或动态库）

```shell
git clone git@github.com:fawdlstty/oml.git
cd oml
cargo build --release --lib # debug: cargo build --lib
```

此时静态库（或动态库）位于 `target/release` 目录下。将其拷贝至C++项目，并引用

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
name = $"hello {value + 12}"
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
    std::cout << root["hello"]["name"].as_str() << std::endl; // hello 42
    return 0;
}
```

### C#

执行命令:
```sh
dotnet add package oml
```

示例:
```csharp
using System;

namespace test {
    public class Program {
        public static void Main () {
            string src = """
[hello]
value = 12
name = $"hello {value + 12}"
""";
            var oeroot = oml.OmlExpr.from_str (src);
            if (oeroot.IsOk(out oml.OmlExpr eroot)) {
                eroot ["hello"] ["value"].set_int (30);
                var oroot = eroot.evalute ();
                if (oroot.IsOk (out oml.OmlValue root)) {
                    Console.WriteLine (root ["hello"] ["name"].as_str()); // hello 42
                } else if (oroot.IsErr (out string err)) {
                    Console.WriteLine (err);
                }
            } else if (oeroot.IsErr (out string err)) {
                Console.WriteLine (err);
            }
            Console.ReadKey ();
        }
    }
}
```

### 其他功能

当满足条件时值可用：

```oml
[hello]

value = 12

@if value == 12
name = $"hello {value}"
```

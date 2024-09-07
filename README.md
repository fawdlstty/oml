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
name = $"hello {value + 12}"
"#;
    let mut eroot = OmlExpr::from_str(oml_str).unwrap();
    eroot["hello"]["value"].set_int(30);
    let root = eroot.evalute().unwrap();
    println!("{}", root["hello"]["name"].as_str()); // hello 42
}
```

### C++

Download and compile static libraries (or dynamic libraries)

```shell
git clone git@github.com:fawdlstty/oml.git
cd oml
cargo build --release --lib # debug: cargo build --lib
```

The static library (or dynamic library) is generated in the `target/release` directory. Copy it to the C++ project and reference it

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
    auto oexpr = oml::OmlExpr::from_str(R"(
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

Run command:
```sh
dotnet add package oml
```

Example:
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
            var eroot = oml.OmlExpr.from_str (src);
            eroot ["hello"] ["value"].set_int (30);
            var root = eroot.evalute ();
            Console.WriteLine (root ["hello"] ["name"].as_str()); // hello 42
            Console.ReadKey ();
        }
    }
}
```

### Other features

The value is available when the conditions are met:

```oml
[hello]

value = 12

@if value == 12
name = $"hello {value}"
```

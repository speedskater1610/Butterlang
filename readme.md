# 🧈 BUTTER language 🧈
#### Basic Unified Toolchain for Transpilation, Execution & Runtime

A tiny transpile-compiled language written in Rust, that utilizes the Area allocation and dynamic strings by default.

![MIT License](https://img.shields.io/badge/License-MIT-green.svg)
![Made in Rust](https://img.shields.io/badge/Rust-🦀-blue)
![Stage](https://img.shields.io/badge/status-alpha-red)

## Example
```
fn sayhi(name: String) => nil {
    print("Hello: ");
    print(name);
    println("Nice to see you checking out this stupid repo");
}

fn main() => nil {
    let mut name: String = "ne";
    name += "rd";
    sayhi(name);
}
```

## 🚀 Quick Start

```bash
git clone https://github.com/LiamWJH/Butterlang
cd Butterlang
cd butter
cargo run
./main
```

## ⚙️ Features
### Techincal stuff
   - Custom lexer + parser
   - AST to C transpiler
   - basefuncs C standard library
   - Arena allocation so NO FOOT GUN

### 🧠 Technical Stuff
- 🧱 **Custom Lexer + Parser**
  Completely hand-written. No dependencies.
- **AST → C Transpiler**
  Outputs clean, readable C code.
- **Built-in Basefuncs Library (C)**
  Includes strings, I/O, safety wrappers, arena allocator.
- **Arena Allocation by Default**
  Zero footguns, zero accidental frees.
- **Dynamic String System**
  Auto-growing, Rust-like `String` experience.
- **Type System**
  Includes `Int`, `Float`, `Bool`, `String`, and `nil`.
- **Transpile–Compile Flow**
  `*.butter` → AST → C → GCC → native executable.

### Actual Language Features

#### Data types
```butter
Int
Float
String
Bool
Nil
```

#### 📝 Variables
```butter
let a: Int = 10;
let mut name: String = "hello";
name += " world";
```

#### 🔢 Basic Expressions
```butter
a = a + 1;
name += "!";
```

#### 📢 Printing
```butter
print(x);
println("Hello world!");
```

#### 🧩 Functions
```butter
fn foo(x: Int) => Int {
    return x * 2;
}
```

#### 💬 Conditions
```butter
if x == 10 {
    println("ten");
} else {
    println("not ten");
}
```

#### 🔁 Loops
```butter
while x < 20 {
    x += 1;
}
```

#### 📜 Nil Type
```butter
fn log(msg: String) => nil {
    println(msg);
}
```
#### 🏗️ Structs
```
struct Person {
  name: String,
  age: Int,
  dead: bool,
}

fn main() => Nil {
  let Liam: Person = Person { name="Liam", age="67", dead=False};
  println(Liam.age);
}
```


## 🤝 Contributing
Pull requests are welcome.
Also before doing any pull requests read the contributing.md
Please open an issue before major changes.

## 📜 License
MIT

Made with 🧠 + 🦀 + ⏱️ + ☕ + 🤓 by LiamWJH.

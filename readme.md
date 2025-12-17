# 🧈 BUTTER language 🧈
#### Basic Unified Toolchain for Transpilation, Execution & Runtime

Butter is a small experimental **transpile-compiled language** written in **Rust**.  
It converts Butter source code into **C code**, which is then compiled using **GCC** to produce a native executable.

The project is currently in **alpha stage** and is actively evolving.

![MIT License](https://img.shields.io/badge/License-MIT-green.svg)
![Made in Rust](https://img.shields.io/badge/Rust-🦀-blue)
![Stage](https://img.shields.io/badge/status-alpha-red)

---

## ✨ Example

```butter
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
```

## ⚙️ Features
### 🔧 Technical features
   - Custom lexer + parser
   - AST to C transpiler
   - basefuncs C standard library
   - Arena allocation by default (designed to reduce memory footguns and accidental frees)

### 🧠 Core Design
- 🧱 **Custom Lexer + Parser**
  Completely hand-written without external dependencies.
- **AST → C Transpiler**
  Outputs clean and readable C code.
- **Built-in Basefuncs Library (C)**
  Includes strings, I/O, safety wrappers, arena allocator.
- **Arena Allocation by Default**
  Reduces common memory management mistakes.
- **Dynamic String System**
  Auto-growing strings with a Rust-like `String` experience.
- **Type System**
  Includes `Int`, `Float`, `Bool`, `String`, and `Nil`.
- **Transpile–Compile Flow**
  `*.butter` → AST → C → GCC → native executable.

### 🧭 How Butter Works (High Level)

```text
.butter source
→ Lexer (tokens)
→ Parser (AST)
→ Transpiler (C code)
→ GCC
→ Native executable
```

### 🏗️ Project Structure (High Level)

```text
butter/
 ├─ src/
 │   ├─ butter.rs        # Compiler entry point
 │   ├─ lexer/           # Tokenization logic
 │   ├─ parser/          # AST construction
 │   ├─ ast/             # AST definitions
 │   ├─ transpiler/      # AST → C conversion
 │   └─ basefuncs/       # C runtime utilities
```
- Structure may change as the project evolves.

---

### 🧬Language Features

#### Data Types
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

#### 🔢 Expressions
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
  let Liam: Person = Person { name="Liam", age=67, dead=False};
  println(Liam.age);
}
```

## 🤝 Contributing
Contributions of all kinds are welcome.  
Pull requests are welcome.  
Please read CONTRIBUTING.md before making changes and open an issue before major feature work.  



## 📜 License
MIT License

Made with 🧠 + 🦀 + ⏱️ + ☕ + 🤓 by LiamWJH.

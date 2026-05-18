# The Xyn Programming Language

Xyn is an **experimental**, *statically typed*, and *multi-backend* programming language designed with a primary focus on *high performance*, *memory safety*, *developer ergonomics*, and *portability*.

Xyn is fully open-source under the **GNU GPLv3 license** to encourage *transparent development*, *academic research*, and *collaborative software engineering*.

## Key Features

- **Feature 1**: **Safe** & **Predictable** Memory Management: **Xyn** use *ownership* and *data values* model that can give *safe memory management* with high performance enabled.

- **Feature 2**: **Strong** Static Type System: **Xyn** will have supports for **GADT** types, *flexible variants*, *templates initiation*, and *type constraints* that can be check with it's **expressive** pattern matching.

- **Feature 3**: **Modular** Compiler Pipeline: **Crafted** with *separation of concerns* in mind. Each phase of the compiler functions as an *isolated*, *testable* **API module**.

- **Feature 4**: **Towards-Zero** External Dependencies: Built completely **from scratch** without relying on **complex** *external third-party libraries*, keeping the codebase **clean** and **highly maintainable**.

## Compiler Architecture & System Design

The Xyn compiler/interpreter follows the classic multi-pass pipeline architecture, converting raw text into executable operations through highly optimized intermediate steps.

```
   (Frontend)
[ Source Code  ]
       │ 
       ▼ 
┌──────────────┐
│    Lexer     │ ──> Converts source character streams into a Stream of Tokens
└──────────────┘
       │
       ▼
┌──────────────┐
│    Parser    │ ──> Builds a structured Abstract Syntax Tree (AST)
└──────────────┘
       │
       ▼
┌──────────────┐
│ Type Checker │ ──> Evaluates static semantics and type safety rules
└──────────────┘
       │ 
       ▼ 
   (Backend)
┌──────────────┐
│      IR      │ ──> Converts Ast into an intermediate form
└──────────────┘
       │ 
       ▼
┌──────────────┐
│  Optimizer   │ ──> Performs optimizations for IRs
└──────────────┘
       │ <— Optimized Instructions stored in `.xir` (Xyn instruction) file
       │
       ▼
┌──────────────┐
│   Backends   │ ──> Native, VM, Interpreter
└──────────────┘
```

## Detailed Pipeline Phases

- **Lexer** (Lexical Analysis): **Scans raw source files** to **identify** *keywords*, *identifiers*, *literals*, and *operators*, producing a clean *token stream* while stripping whitespaces and comments.

- **Parser** (Syntactic Analysis): Employs **pratt** and **recursive-based** parsing algorithm to transform tokens into a strongly-typed **Abstract Syntax Tree** (AST) representing the program's structural grammar.

- **Type Checker** (Semantic Analysis): **Verifies** *scoping rules*, *variable declarations*, and ensures complete **type-safety** across operations.

- **Intermediate Representation** (IR): **Converts** *checked AST* into a **lower** and **more explicit** form, it makes targeting different backends more easier and scalable.

- **Optimizer**: **Optimizes IRs** into *shorter* structures and *less instructions*. It performs *constant folding*, *code elimination*, etc. 

- **Backends**: Multiple backends like **Virtual machines**, **Native**, and **Interpreter** will use the optimized IRs and run it or compiled it again into executable file.

## Syntax Specification (Language Guide)

Below is an overview of Xyn's syntax and grammar specifications.

### EBNF Grammar (Draft)
```
(* Basic grammar rules for the Xyn language *)
program       = { statement } ;
statement     = variable_decl | assignment | function_decl | expression_stmt ;
variable_decl = ("let" | "const" | "mut") , identifier , [type] , "=" , expression , ";" ;
expression    = term , { ( "+" | "-" ) , term } ;
term          = factor , { ( "*" | "/" ) , factor } ;
factor        = identifier | number | "(" , expression , ")" ;
```

## Code Examples

### 1. Variables and Constants

```
let message: str = "Hello, Xyn!";
const PI: float = 3.14159;
```

### 2. Control Flow

```
if x > 10 {
    print("Greater");
} else {
    print("Lesser or Equal");
}
```


### 3. Functions

```
fn add(a int, b: int) int {
    return a + b;
}
```


## Repository Directory Structure
> `Note`: Some folders not exist yet or still in development.
```
xyn/
├── src/                  # Main compiler source files
│   ├── lexer/            # Lexer/Scanner modules
│   ├── parser/           # AST definitions and Pratt/Recursive parser
│   ├── sema/             # Type checking and semantic analyzer
│   ├── ir/               # Intermediate representation
│   ├── optimizer/        # IRs Optimizer
│   ├── backend/          # Multiple backend targets
│   └── main.xyn          # Main entry point of the compiler CLI
├── tests/                # Automated test suite (Integration and Unit tests)
├── examples/             # Sample scripts written in Xyn (.xyn)
├── docs/                 # Detailed architectural documentation
├── LICENSE               # GNU GPLv3 terms
└── README.md             # This file
```

## Getting Started

### Prerequisites

To build Xyn from source, ensure your development machine meets the following environment requirements:

- **Language runtime**: `Rust v1.9.5`
- **Build toolchain**: `Cargo v1.9.5`

### Installation & Build

Clone this repository to your local machine:

```
git clone https://github.com/Reynn13/Xync.git
cd xyn
```
Build the project in release mode:
```
cargo build --release
```

### Command Line Interface (CLI) Usage

Run a Xyn source file:

```
./xyn your_file.xyn
```

To launch the interactive Read-Eval-Print Loop (REPL), you can do it like this:

```
./xyn repl
```

Debug/dump the internal representation phases (Very helpful for compiler development!):

```
./xyn --dump-tokens example.xyn
./xyn --dump-ast example.xyn
```


## Development Roadmap

- [x] Phase 1: Token specification, lexical scanner implementation, and CLI scaffold.

- [x] Phase 2: Core AST nodes representation, parser construction, and syntax error reporters.

- [x] Phase 3: Static type checking and scope resolution system.

- [ ] Phase 4: Basic tree-walk interpreter or compiler backend (bytecode / assembly generation).

- [ ] Phase 5: Standard library (I/O operations, string manipulations, basic file system).

- [ ] Phase 6: Optimization passes and benchmark suites.

## Contributing

We welcome contributions of all forms! Whether you are looking to fix compiler bugs, propose new syntax specifications, write tests, or improve documentation, your help is highly appreciated.

- Fork the repository.
- Create your feature branch (git checkout -b feature/amazing-feature).
- Commit your changes using clear descriptive messages.
- Push your branch (git push origin feature/amazing-feature).
- Open a Pull Request detailing your enhancements.

## License

This project is licensed under the **GNU General Public License v3** (GPLv3). This ensures that the code remains **open**, **accessible**, and **transparent** for everyone to study and modify. See the [LICENSE](./LICENSE) file for the full legal text.
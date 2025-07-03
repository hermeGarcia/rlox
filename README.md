<h1 align="center"> rlox </h1>
<h2 align="center"> A rust implementation of the lox programming language </h3>

Lox is the programming language in [crafting interpreters](https://craftinginterpreters.com), an amazing book that I can not recommend enough.

As computer scientist passionate about compilers and programming languages I like to implement new ideas that come to mind or that I find while researching. Sometimes I also like to write posts about the implementation journey and thats why this repo is public, Lox is the perfect language for me to use a playground.

Keep in mind when diving in this codebase that, while I care a lot about code quality, this is a playground for me to experiment with new ideas. Do not expect every feature to make sense or even a complete Lox implementation, sometimes I just want to implement an analysis to understand it better.

One of my goals for this project is to use as little third party dependencies as possible. So if at some point you think that I am reinventing the wheel, I most certainly am.


## Structure of the project

- `rlox_ast` contains the AST for lox, an implementation based on buffers.
- `rlox_compiler` entry point for the compiler and repl.
- `rlox_errors` defines a common way for defining errors.
- `rlox_graphviz` defines the `inspect` tool, used to translate internal representations to `dot` files that can be visualized using Graphviz.
- `rlox_interpreter` is a tree-walk interpreter of the ast.
- `rlox_parser` is the Lox parser.
- `rlox_source` utils for storing and accessing source code.
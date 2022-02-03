# This is a [pug](https://pugjs.org/) compiler written in [rust](https://www.rust-lang.org/).

## Premise

Project started with a purpose of speeding up the javascript compiler using [WebAssembly](https://webassembly.org/).

.PUG + JSON -> Compiler -> .HTML

Now it tries to create a faster precompiled variety.

Source files -> .MOPS + JSON -> Compiler -> HTML

## Project structure

pug-html-compiler - core pug -> html

pug-scripts-compiler - procedural pug scripts compiler

utils - general ascii and array pointer utilities

## Roadmap:

✅ Tag parsing

✅ Attributes

✅ UTF-8 Support

⏳ Scripting

⏳ Multithreading

⏳ .MOPS Binary file format

⏳ QOL improvements

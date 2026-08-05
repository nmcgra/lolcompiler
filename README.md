# LOLCODE-to-HTML Compiler

A small compiler written in Rust that translates a LOLCODE-inspired markup 
language into HTML. It runs through a lexer, a recursive-descent
parser, a static scope checker, and an HTML generator, then attempts to open
the result in Chrome.

## Pipeline
1. **Lexical Analysis** (`lexer.rs`) - reads the `.lol` source file and
   produces a stream of tokens. Recognized keyword otkens (case-insensitive)
   are:
   ```
   #HAI #KTHXBYE #OBTW #TLDR #MAEK #GIMMEH #OIC #MKAY
   #I #HAZ #IT #IZ #LEMME #SEE #NEWLINE
   #SOUNDZ #VIDZ #HEAD #PARAGRAF #LIST #ITEM
   ```

   Any other whitespace-delimited word is treated as plain text. An unknown 
   `#`-prefixed token is a lexical error and aborts compilation. Tokens are
   written out to a `.lex` file alongside the source.

2. **Syntax Analysis** (`parser.rs`) - a recursive-descent parser that
   validates the token stream against the language grammer (program wrapper,
   `HEAD`/`TITLE` block, paragraphs, lists with items, audio/video tags,
   variable definitions, uses, etc.). On failure it prints a syntax error and 
   the partial output files are cleaned up.

3. **Static Scope Analysis** (`main.rs` + `semantic.rs`) - walks the token
   stream a second tim eto track variable definitions (`#I HAZ <name> #IT IZ
   <value> #MKAY`) and uses (`#LEMME SEE <name> #MKAY) across nested scopes 
   (scopes are pushed/popped on `#MAEK` / `#OIC`). Using a variable before
   it's defined in any enclosing scope is a semantic error. Results are
   written to a `.sem` file.

4. **HTML Generation** (`htmlgen.rs`) - walks the tokens once more and emits
a corresponding `.html` file, then attempts to open it in Google Chrome
(macOS and Windows only).

## Language Reference

| LOLCODE                              | HTML output                          |
|---------------------------------------|---------------------------------------|
| `#HAI ... #KTHXBYE`                   | wraps the whole document              |
| `#MAEK HEAD ... #OIC`                 | document head block                  |
| `#GIMMEH TITLE ... #MKAY`             | page title (inside `HEAD`)           |
| `#MAEK PARAGRAF ... #OIC`             | `<p>...</p>`                         |
| `#MAEK LIST ... #OIC`                 | `<ul>...</ul>`                       |
| `#GIMMEH ITEM <text> #MKAY`           | `<li><text></li>`                    |
| `#GIMMEH BOLD <text>`                 | `<b><text></b>`                      |
| `#GIMMEH ITALICS <text>`              | `<i><text></i>`                      |
| `#GIMMEH NEWLINE`                     | `<br/>`                              |
| `#GIMMEH SOUNDZ <src> #MKAY`          | `<audio controls><source src=...>`   |
| `#GIMMEH VIDZ <src> #MKAY`            | `<video controls><source src=...>`   |
| `#I HAZ <name> #IT IZ <value> #MKAY`  | defines a variable (no HTML output)  |
| `#LEMME SEE <name> #MKAY`             | `<span><name></span>`                |

Plain (non-`#`) tokens outside of a recognized construct are emitted as
literal text.

## Building

Requires the Rust toolchain (`cargo`).

```bash
cargo build --release
```

## Usage
```bash
cargo run -- <path/to/source.lol>
```

The input file must exist and have a `.lol` extension. On a successful run,
three files are produced next to the source:

- `<name>.lex` - the tokenized source
- `<name>.sem` - static scope-check result
- `<name>.html` - generated HTML, which the program will also try to open
    in Google Chrome

If syntax or semantic analyssi fails, an error is printed and any
intermediate output is removed.

## Project Layout

```
main.rs     - CLI entry point: drived the compilation pipeline
lexer.rs    - tokenizer
parser.rs   - recursive-descent syntax checker
semantic.rs - scope-based variable tracking
htmlgen.rs  - token-to-HTML translation and Chrome launch
```

## Known Limitations
- Chrome auto-launch is only implemented for macOS and Windows. On linux
  the attempt via `open -a "Google Chrome" will simplye fail with a printed
  message.
- `#OIC` unconditionally closes `<p>`, `<ul>`, and `<h1>` tags regardless of 
  which block it's actually closing, so nested/mixed block types can produce
  extra closing tags in the output HTML.
- `#OBTW` / `#TLDR` (comments) are lexted as valid tokens but have no
  parser or codegen handling.
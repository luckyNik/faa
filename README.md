# faa

Little CLI for a collection of integer sets (variant 16, RD-tree). Only the command parser and the CLI are done so far; commands just echo what they parsed.

Run it with `cargo run`, test it with `cargo test`. Type `help;` inside for the commands.

```
src/
  main.rs       entry point
  cli.rs        input loop, splits on ;
  lexer.rs      text -> tokens
  parser.rs     tokens -> Command
  command.rs    the Command type
  executor.rs   runs commands (stub for now)
  error.rs      errors
  storage/      RD-tree and database, empty for now
tests/          parser and cli tests
```

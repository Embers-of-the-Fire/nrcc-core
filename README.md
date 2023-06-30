# NRCC-Core

> [Chinese 中文](./ZH-README.md)

**NRCC** is a code counting tool written in Rust language. This repository contains its core parsing library.

## How to Use

If you are a user who wants to use this tool, please go to the **NRCC** repository to get a tool that can be used as a CLI.

If you are a developer, you can use this repository to provide code counting functionality for your project.

If you want to contribute to this repository, please check [CONTRIBUTING](./ZH-CONTRIBUTING.md) for more information.

## About Counting Rules

Compared with other code counting tools (such as **Tokei**), this repository (currently only) provides code counting as blocks. Traditional counting algorithms (as optional replacements) are planned in the development.

"Code counting as blocks" means that code blocks (including comment blocks) are not counted as independent physical lines, but as "blocks". Only blank lines are plain with physical lines.

For example, the following (Rust) code:

```rust
fn main() {
    call1(); /* Inline comment! */ call2();

    /// Independent document comment
}
```

will produce the following result:

```yaml
code: 4
blank: 1
all: 5
comment:
  doc: 1
  normal: 1
  doc_quote: 0
sub_language: {}
```

It should also be noted that this parsing library ignores the last suffix line of the file, that is, it ignores the last `'\r\n'` and other line ending symbols without producing an extra blank line.

## License

[MIT](./LICENSE-MIT)

[Apache 2.0](./LICENSE-APACHE)
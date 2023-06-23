# Contributing

## Add a New Language

### Define a New Language

You can define a new language in [languages.yaml](./languages.yaml).

First, add a new key and its metadata to the file:

```yaml
MyLang:  # This is the identity of the language. Only alphanumeric characters and `_` are allowed.
    name: MyLanguage  # This its the name used to output stats, but not the ident of the language, so space is allowed here.
    alias: [mylang, MyLang]  # This is optional, and will be passed to serde's `alias` attribute.
```

Then, add the syntax definition for the language:

```yaml
MyLang:
    name: MyLanguage
    alias: [mylang, MyLang]
    syntax:  # This is the main entry point.
        line_prefix: foo  # Optional. The prefix for any line in the source file.
        block: [["{", "}"]]  # Optional. Use 2-element array. This is not used now, but is planned for future features.
        comment:  # Comment syntax, not including doc-quotes like those in Python.
            multi: [["/*", "*/"]]  # Multi-line comments.
            single: ["//", "#"]  # Single-line comments
            doc: ["///", "//!"]  # Doc comments, single-line.
            doc_multi: [["/**", "*/"]]  # Doc comments, multi-line.
        quote:
            normal: [['\"', '\"']]  # Remenber to escape those quotes, since they'll be parsed twice. For example, `"` should be `'\"'` or `"\\\""`.
            literal: [['r\"', '\"']]  # Literal string, which will ignore `\`s.
            doc: [['"""', '"""']]  # Doc string, like Python's multi-line doc comments.
        sub_language: [["<mysub>", "</mysub>", "Rust"]]  # Sub-languages, the third element is the language's identifier but not its name.
```

### Add a Test

After you've defined the language, add a test for it.

To add a test, see [test_config.yaml](./tests/test_config.yaml).

```yaml
MyLang:  # Here should be the language's identifier
    file: ./tests/source/mylang.ml  # This lib wouldn't check its file type but only parse it as the language type you've told the parser.
    name: mylang  # This is the identifier of the function.
    stats:  # This is the correct stats for the file.
        code: 1
        blank: 1
        all: 3
        comment:
            doc: 1
            normal: 1
            doc_quote: 0
        sub_language:
            Rust:  # Use the identifier of the sub-language.
                code: 0
                blank: 0
                all: 1
                comment:
                    doc: 1
                    normal: 0
                    doc_quote: 0
```

> For details about the statistics, see the lib's documentation.

Then, you should add a test file according to the path you specified.

> Please put your test files in `./tests/source/` directory.

## Contributing to the Parser

Any PR for this project is truly appreciated, but it's better to tell why you're changing the code in your PR, since any changes will cost other developers the energy to adapt again.

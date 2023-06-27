# Development Guide

> [Chinese 中文](./ZH-CONTRIBUTING.md)

## Adding a New Language

### Defining a New Language

You can define a new language in the file [languages.yaml](./languages.yaml).

First, add a new key and language metadata in the document:

```yaml
MyLang: # This is the identifier of the language. Only alphanumeric characters and underscores are allowed.
  name: MyLanguage # This is the name of the language. It is used for data output and other communication with users. It is not an identifier, so it can be any text.
  alias: [mylang, MyLang] # Optional. If not required, set as an empty array. This property will be passed to the 'alias' property of the serde library.
```

Then, add a syntax definition for your language:

```yaml
MyLang:
  name: MyLanguage
  alias: [mylang, MyLang]
  syntax: # This is the entry for syntax definitions, and all syntax definitions need to be under this key.
    line_prefix: foo # Optional. The prefix of any line in the source file (e.g., the "///" prefix used to define Rust's doc-test).
    block: [["{", "}"]] # Optional. Use a list of two-element arrays to represent code block distinctions, as in C, but excluding code blocks in Python-like languages that are defined using indentation. This definition is not used in the current version, but it is recommended to set it up for future functionality development.
    comment: # Syntax for comments, not including comment strings (such as those commonly used in Python).
      multi: [["/*", "*/"]] # The prefix and suffix of a multiline comment.
      single: ["//", "#"] # The prefix of a single-line comment.
      doc: ["///", "//!"] # The prefix of a single-line documentation comment.
      doc_multi: [["/**", "*/"]] # The prefix and suffix of a multiline documentation comment.
    quote:
      normal: [['\"', '\"']] # Note that quotes need to be escaped, as they will be processed twice. For example, double quotes ('"') should be '\"' or '\\"'.
      litral: [['r\"', '\"']] # Literal strings that ignore backslash escaping.
      doc: [['"""', '"""']] # Document strings, as in Python.
    sub_language: [["<mysub>", "</mysub>", "Rust"]] # The third value is the identifier of the language, not the name. If you want to add a language similar to Rust's doc-test, add a new language to avoid conflicts. If you need to reuse an old language, such as CSS and Javascript in HTML, simply reference it here.
```

### Adding a Test

After defining a language, add a test for it.

Add a test file and add the following fields in the [test_config.yaml](./tests/test_config.yaml) file:

```yaml
MyLang: # This should be the identifier of the language.
  file: ./tests/sorce/mylang.ml # This is the path to the language test file. The file name is not checked during testing, so you can use any identifier you like.
  name: mylang # This is the identifier of the test function. Recommended to use the naming format of Rust.
  stats: # This is the correct file statistics data.
    code: xxx
    blank: xxx
    all: xxx # Note that the suffix line should not be included.
    comment:
      doc: xxx
      normal: xxx
      doc_quote: xxx
    sub_language:
      MySub:
        code: xxx
        blank: xxx
        all: xxx
        comment:
          doc: xxx
          normal: xxx
          doc_quote: xxx
        sub_language: {}
```

> For detailed information on statistics data, refer to the project's documentation.

## Contributing to the Parser Development

Any PRs for this project will be appreciated. Please refer to the Github template for specific PR submission recommendations.

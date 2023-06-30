# 开发指南

## 添加一个新语言

### 定义一个新语言

你可以在[languages.yaml](./languages.yaml)中定义一个新语言。

首先，在文档中添加一个新的键和语言的元数据：

```yaml
MyLang: # 这是语言的标识符，只有英文字母、数字和下划线是合法表达。
  name: MyLanguage # 这是这个语言的名称，用于作为数据输出和其他与用户沟通的情况。这不是一个标识符，所以可以为任何文字。
  alias: [mylang, MyLang] # 可选，如果无需请设置为空数组。这个属性会被传递给serde库的`alias`属性。
```

然后，为你的语言添加语法定义：

```yaml
MyLang:
  name: MyLanguage
  alias: [mylang, MyLang]
  syntax: # 这是语法定义的入口，所有语法定义都需要在这个键之下。
    line_prefix: foo # 可选。源文件中任意行的前缀（例如，在定义rust的doc-test时的“///”前缀）。
    block: [["{", "}"]] # 可选。使用一个二元数组的列表。表示代码块的区分，例如C语言等，但是类似Python的用缩进表示的代码块不在考量范围。在当前版本中这个定义没有被使用，但是为了未来可能的功能开发建议设定。
    comment: # 注释语法，不包括注释字符串（例如Python中常用的那种）。
      multi: [["/*", "*/"]] # 多行注释的前缀和后缀。
      single: ["//", "#"] # 单行注释的前缀。
      doc: ["///", "//!"] # 单行文档注释的前缀。
      doc_multi: [["/**", "*/"]] # 多行文档注释的前缀和后缀。
    quote:
      normal: [['\"', '\"']] # 请注意要对引号转码，因为他们会被处理两次。例如，双引号（`'"'`）应该是`'\"'`或`"\\\""`。
      litral: [['r\"', '\"']] # 字面量字符串，会忽略反斜杠转码。
      doc: [['"""', '"""']] # 文档字符串，参考Python。
    sub_language: [["<mysub>", "</mysub>", "Rust"]] # 第三个为语言的**标识符**而不是名称。如果你希望添加一个类似Rust的doc-test的语言，请添加一个新语言以避免冲突；如果你确定需要重新使用一个旧语言，例如html中的css和javascript，直接在这里引用即可。
```

### 添加一个测试

在定义了一个语言后，请为它添加一个测试。

添加一个测试文件，并在[test_config.yaml](./tests/test_config.yaml)中，添加如下字段：

```yaml
MyLang: # 这里应该是语言的标识符
  file: ./tests/sorce/mylang.ml # 这里是语言测试文件的路径；测试时不会检查文件名，所以可以用任意你希望的标识。
  name: mylang # 这是测试函数的标识符，推荐使用rust的命名范式。
  stats: # 这是正确的文件统计数据
    code: xxx
    blank: xxx
    all: xxx # 注意不要包含了尾缀行
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

> 对于统计数据的详细信息，参考项目的文档。

### 配置一个新语言的文件类型

在[languages.yaml](./languages.yaml)中你的语言的键后，添加如下内容：

```yaml
MyLang:
  ...
  file:
    extension: # 必须包含这个键，可以值置空。表示文件扩展名检测。
      plain: ["ml"] # 可选。完全匹配。
      case_insensitive: ["mylang"] # 可选。忽略ASCII大小写的完全匹配（这里需要填写全小写文字）。
      regex: ["*.ml"] # 可选。正则字符串匹配（从头匹配）
    file_name: # 必须包含这个键，可以值置空。表示对于整个文件名的检测。
      plain: ["ml"] # 同上
      case_insensitive: ["mylang"] # 同上
      regex: ["*.ml"] # 同上
```

随后，在[test_config.yaml](./tests/test_config.yaml)中为其添加测试：

```yaml
MyLang:
  ...
  file_detect: # 使用数组形式
    - regex.ml
```

## 参与解析器开发

任何对于这个项目的 PR 都会受到感激，具体的 PR 提交建议参考 Github 的模板。

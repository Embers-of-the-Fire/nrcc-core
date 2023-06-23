LanguageSyntax {
    line_prefix: {% if line_prefix %} Some("{{line_prefix}}") {% else %} None {% endif %},
    ignore_prefix_space: {{ ignore_prefix_space }},
    block_pairs: &[
        {% for b in block -%}
        SyntaxPair {
            name: "block{{- loop.index}}",
            left: "{{b[0]}}",
            right: "{{b[1]}}",
        },
        {% endfor %}
    ],
    comment_pairs: &[
        {% for cop in comment.multi -%}
        SyntaxPair {
            name: "cop{{- loop.index}}",
            left: "{{cop[0]}}",
            right: "{{cop[1]}}",
        },
        {% endfor %}
    ],
    quote_pairs: &[
        {% for quotep in quote.normal -%}
        SyntaxPair {
            name: "quotep{{- loop.index}}",
            left: "{{quotep[0]}}",
            right: "{{quotep[1]}}",
        },
        {% endfor %}
    ],
    literal_quote_pairs: &[
        {% for litqp in quote.literal -%}
        SyntaxPair {
            name: "litqp{{- loop.index}}",
            left: "{{litqp[0]}}",
            right: "{{litqp[1]}}",
        },
        {% endfor %}
    ],
    simple_comment: &[{%- for cmt in comment.single -%}"{{cmt}}", {% endfor -%}],
    doc_comment: &[{%- for cmt in comment.doc -%}"{{cmt}}", {% endfor -%}],
    doc_quote_pairs: &[
        {% for dqp in quote.doc -%}
        SyntaxPair {
            name: "dqp{{- loop.index}}",
            left: "{{dqp[0]}}",
            right: "{{dqp[1]}}",
        },
        {% endfor %}
    ],
    sublang_pairs: &[
        {% for sbp in sub_language -%}
        (SyntaxPair {
            name: "sbp{{- loop.index}}",
            left: "{{sbp[0]}}",
            right: "{{sbp[1]}}",
        }, LanguageType::{{sbp[2]}}),
        {% endfor %}
    ],
    doc_comment_pairs: &[
        {% for dcp in comment.doc_multi -%}
        SyntaxPair {
            name: "dcp{{- loop.index}}",
            left: "{{dcp[0]}}",
            right: "{{dcp[1]}}",
        },
        {% endfor %}
    ]
}
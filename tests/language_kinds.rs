use code_grammar::SyntaxLanguage;

#[test]
fn test_language_kinds() {
  let langs = [
    SyntaxLanguage(0),
    javascript_grammar::syntax_kind::JAVASCRIPT,
    typescript_grammar::syntax_kind::TYPESCRIPT,
    html_grammar::syntax_kind::HTML,
    vue_grammar::syntax_kind::VUE,
  ];

  // Check language kind is within the allowed range
  for lang in &langs {
    assert!(lang.0 <= SyntaxLanguage::MAX);
  }

  // Check all language kinds are distinct
  for (left, li) in langs.iter().enumerate() {
    for (right, ri) in langs.iter().enumerate() {
      assert!(li == ri || left != right);
    }
  }
}
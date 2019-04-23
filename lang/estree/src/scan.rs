use web_grammars_utils::Scanner;

pub fn is_markdown_lang_char(c: char) -> bool {
    (c >= 'a' && c <= 'z')
        // not sure if any of these three are actually wanted, but being permissive for now
        || (c >= 'A' && c <= 'Z')
        || c == '_'
        || c == '-'
}

/// Extracts code blocks from a markdown file
pub fn extract_code_blocks(text: &str) -> Vec<&str> {
  let mut text = text;
  let mut results = Vec::new();
  while !text.is_empty() {
    let mut s = Scanner::new(text);
    let c = s.bump().unwrap();
    let found = scan_part(c, &mut s);
    let length: u32 = s.into_len().into();
    if found {
      results.push(&text[..length as usize]);
    }
    text = &text[length as usize..];
  }
  results
}

fn scan_part(c: char, s: &mut Scanner) -> bool {
  match c {
    '`' => {
      return scan_code_block(s);
    }
    _ => {
      s.bump_while(|c| c != '`');
      return false;
    }
  }
}

/// Assumes preceding backtick
fn scan_code_block(s: &mut Scanner) -> bool {
    if !s.at_str("``") {
      return false;
    }
    s.bump();
    s.bump();
    s.bump_while(is_markdown_lang_char);
    while let Some(c) = s.current() {
        match c {
            '`' => {
                s.bump();
                if s.at_str("``") {
                  s.bump();
                  s.bump();
                  return true;
                }
            }
            _ => {
              s.bump();
            }
        }
    }

    false // not a valid code block, missing closing "```"
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_extract_sample() {
      let example = r#"
This document specifies the extensions to the core ESTree AST types to support the ES2018 grammar.

# Statements

```js
extend interface ForOfStatement {
  await: boolean;
}
```

`for-await-of` statements, e.g., `for await (const x of xs) {`

# Expressions

```js
extend interface ObjectExpression {
    properties: [ Property | SpreadElement ];
}
```

Spread properties, e.g., `{a: 1, ...obj, b: 2}`.
"#;

      let blocks = extract_code_blocks(example);
      assert_eq!(blocks.len(), 2);
      assert_eq!(blocks[0], r#"```js
extend interface ForOfStatement {
  await: boolean;
}
```"#);
  }
}
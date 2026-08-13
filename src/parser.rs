use crate::shell::ParseResult;

// Parse the input string into command tokens
pub fn parse(line: &str) -> ParseResult {
    let chars: Vec<char> = line.chars().collect();
    let mut tokens = Vec::new();
    let mut single_quote = false;
    let mut double_quote = false;
    let mut word = String::new();
    let mut waiting = false;
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if c == '\\' && !single_quote {
            match chars.get(i + 1) {
                None => {
                    waiting = true;
                    i += 1;
                    continue;
                }
                Some('\n') => {
                    if i + 2 == chars.len() {
                        waiting = true;
                    }
                    i += 2;
                    continue;
                }
                Some(&next) => {
                    if double_quote && !['`', '"', '\\', '$'].contains(&next) {
                        word.push('\\');
                    }
                    word.push(next);
                    i += 2;
                    continue;
                }
            }
        }

        if c == '\'' && !double_quote {
            single_quote = !single_quote;
        } else if c == '"' && !single_quote {
            double_quote = !double_quote;
        } else if c.is_whitespace() && !single_quote && !double_quote {
            if !word.is_empty() {
                tokens.push(word.clone());
                word.clear();
            }
        } else if c == '#' && !single_quote && !double_quote && i > 0 && chars[i - 1].is_whitespace() {
            return ParseResult::Complete(tokens);
        } else {
            word.push(c);
        }

        i += 1;
    }

    if !word.is_empty() {
        tokens.push(word);
    }

    if single_quote || double_quote || waiting {
        return ParseResult::Uncomplete("> ".to_string());
    }

    ParseResult::Complete(tokens)
}
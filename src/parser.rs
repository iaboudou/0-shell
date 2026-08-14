use crate::shell::ParseResult;

// Parse the input string into command tokens
pub fn parse(line: &str) -> ParseResult {
    let chars: Vec<char> = line.chars().collect();
    let mut tokens = Vec::new();
    let mut single_quote = false;
    let mut double_quote = false;
    let mut word = String::new();
    let mut i = 0;

    let t = line.chars().count() - line.trim_end_matches('\\').chars().count();
    if t % 2 != 0 {
        return ParseResult::Uncomplete("> ".to_string());
    }

    while i < chars.len() {
        let c = chars[i];

        if c == '\\' && !single_quote {
            if i + 1 >= chars.len() {
                return ParseResult::Uncomplete("> ".to_string());
            }
        
            if double_quote {
                if chars[i + 1] == '$' || chars[i + 1] == '`' || chars[i + 1] == '"' || chars[i + 1] == '\\' {
                    word.push(chars[i + 1]);
                } else if chars[i + 1] == '\n' {
                } else {
                    word.push('\\');
                    word.push(chars[i + 1]);
                }
            } else {
                if chars[i + 1] == '\n' {
                } else {
                    word.push(chars[i + 1]);
                }
            }
            i += 2;
            continue;
        } else if c == '\'' && !double_quote {
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

    if single_quote || double_quote {
        return ParseResult::Uncomplete("> ".to_string());
    }

    ParseResult::Complete(tokens)
}
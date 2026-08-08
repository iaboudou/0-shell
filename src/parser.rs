use crate::shell::ParseResult;

// Parse the input string into command tokens
pub fn parse(line: &str) -> ParseResult {
    let mut tokens = Vec::new();
    let mut in_single_quot = false;
    let mut in_duble_quots = false;
    let mut temp = String::new();

    for (i, c) in line.chars().enumerate() {
        if c == '\'' && !in_duble_quots {
            in_single_quot = !in_single_quot;
        }
        else if c == '\"' && !in_single_quot {
            in_duble_quots = !in_duble_quots;
        }
        else if c.is_whitespace() && !in_single_quot && !in_duble_quots {
            if !temp.is_empty() {
                tokens.push(temp);
                temp = String::new();
            }
        }else if c == '#' && !in_single_quot && !in_duble_quots && (i> 0 && line.chars().nth(i-1).unwrap_or_default().is_whitespace() ) {
            return ParseResult::Complete(tokens);
        }
        else {
            temp.push(c);
        }
    }

    if !temp.is_empty() {
        tokens.push(temp);
    }

    if let Some(last) = tokens.last_mut() {
        if last.ends_with('\\') {
            return ParseResult::Uncomplete("> ".to_string());
        }
    }

    if in_single_quot {
        return ParseResult::Uncomplete("quote> ".to_string());
    }

    if in_duble_quots {
        return ParseResult::Uncomplete("dquote> ".to_string());
    }

    ParseResult::Complete(tokens)
}

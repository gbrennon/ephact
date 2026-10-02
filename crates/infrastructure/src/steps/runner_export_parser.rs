use std::{collections::HashMap, str::Lines};

/// Parses the key/value records written to a runner export file.
///
/// The runner file format supports both `NAME=value` records and the
/// documented `NAME<<DELIMITER` multiline form. Records without a closing
/// delimiter are ignored because they do not represent a complete export.
pub(crate) fn parse(contents: &str) -> HashMap<String, String> {
    let mut exports = HashMap::new();
    let mut lines = contents.lines();

    while let Some(line) = lines.next() {
        if let Some((name, delimiter)) = line.split_once("<<") {
            if let Some((name, value)) = parse_multiline(&mut lines, name, delimiter) {
                exports.insert(name, value);
            }
        } else if let Some((name, value)) = parse_single_line(line) {
            exports.insert(name, value);
        }
    }

    exports
}

fn parse_single_line(line: &str) -> Option<(String, String)> {
    let (name, value) = line.split_once('=')?;
    (!name.is_empty()).then(|| (name.to_owned(), value.to_owned()))
}

fn parse_multiline(lines: &mut Lines<'_>, name: &str, delimiter: &str) -> Option<(String, String)> {
    if name.is_empty() || delimiter.is_empty() {
        return None;
    }

    let mut value = Vec::new();
    for line in lines.by_ref() {
        if line == delimiter {
            return Some((name.to_owned(), value.join("\n")));
        }
        value.push(line);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn parses_single_line_records() {
        assert_eq!(
            parse("A=1\nQUERY=a=b=c\n"),
            [
                ("A".to_owned(), "1".to_owned()),
                ("QUERY".to_owned(), "a=b=c".to_owned()),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn parses_documented_multiline_records() {
        assert_eq!(
            parse("DESCRIPTION<<EOF\nfirst\nsecond\nEOF\n"),
            [("DESCRIPTION".to_owned(), "first\nsecond".to_owned())]
                .into_iter()
                .collect()
        );
    }

    #[test]
    fn ignores_unclosed_multiline_records() {
        assert!(parse("DESCRIPTION<<EOF\nfirst\n").is_empty());
    }
}

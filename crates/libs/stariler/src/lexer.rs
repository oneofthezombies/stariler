use lazy_static::lazy_static;
use tracing::instrument;

lazy_static! {
    /// Unicode `Pattern_White_Space` characters.
    /// please see https://www.unicode.org/reports/tr31/#R3a.
    static ref WHITE_SPACES: Vec<String> = vec![
        String::from('\u{000A}'), // line feed
        String::from('\u{000B}'), // vertical tabulation
        String::from('\u{000C}'), // form feed
        String::from('\u{000D}'), // carriage return
        String::from('\u{0085}'), // next line
        String::from('\u{2028}'), // line separator
        String::from('\u{2029}'), // paragraph separator
    ];

    static ref MATCHERS:Vec<Matcher> = vec![
        Matcher {
kind: TokenKind::KeywordLet,
            is_produce: true,
            is_match: |b| match b {
            [b'l', b'e', b't', ..] => Some(3),
            _ => None,
        }},
        Matcher {
            kind: TokenKind::Whitespace,
            is_produce: false,
            is_match: |b| match b {
            [b' ', ..] => Some(1),
            [b'\t', ..] => Some(1),
            [b'\n', ..] => Some(1),
            [b'\r', ..] => Some(1),
            _ => None,
        }},
    ];
}

#[derive(Debug)]
enum TokenKind {
    KeywordLet,
    Whitespace,
}

type Start = usize;
type End = usize;

/// It's a half-open interval.  
/// `start` is inclusive, `end` is exclusive.
#[derive(Debug)]
struct Span {
    start: Start,
    end: End,
}

#[derive(Debug)]
struct Matcher {
    kind: TokenKind,
    is_produce: bool,
    is_match: fn(&[u8]) -> Option<End>,
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    span: Span,
    value: Option<Vec<u8>>,
}

#[instrument]
async fn tokenize(path: std::path::PathBuf) -> crate::Result<()> {
    use tokio::io::AsyncReadExt;

    const BUFFER_SIZE: usize = 4 * 1024;
    let mut file = tokio::fs::File::open(&path).await?;
    let mut token_buffer = Vec::new();
    let mut file_buffer = [0; BUFFER_SIZE];
    let mut file_position = 0;
    loop {
        let n = file.read(&mut file_buffer[..]).await?;
        if n == 0 {
            break;
        }
        token_buffer.extend_from_slice(&file_buffer[..n]);
        loop {
            match token_buffer.as_slice() {
                [] => {
                    println!("empty");
                    break;
                }
                [b'l', b'e', b't', ..] => {
                    println!("let");
                    break;
                }
                [..] => {
                    println!("unknown");
                    break;
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn let_foo_0() {
        let token_buffer = vec![b'l', b'e', b't', b' '];
        match token_buffer.as_slice() {
            [] => {
                println!("empty");
            }
            [b'l', b'e', b't', ..] => {
                println!("let");
            }
            [..] => {
                println!("unknown");
            }
        }
    }
}

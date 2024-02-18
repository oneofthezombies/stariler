use lazy_static::lazy_static;
use tracing::instrument;

lazy_static! {
    static ref WHITESPACES: Vec<String> = get_whitespaces();
    static ref MATCHERS: Vec<Matcher> = get_matchers();
}

/// Unicode `Pattern_White_Space` characters.
/// please see <https://www.unicode.org/reports/tr31/#R3a>.
// TODO: refactor to return Vec<Vec<u8>>
fn get_whitespaces() -> Vec<String> {
    vec![
        String::from('\u{000A}'), // line feed
        String::from('\u{000B}'), // vertical tabulation
        String::from('\u{000C}'), // form feed
        String::from('\u{000D}'), // carriage return
        String::from('\u{0085}'), // next line
        String::from('\u{2028}'), // line separator
        String::from('\u{2029}'), // paragraph separator
    ]
}

fn get_matchers() -> Vec<Matcher> {
    vec![
        Matcher {
            kind: TokenKind::KeywordLet,
            is_produce: true,
            is_match: |buf| {
                // TODO: refactor to reusable function
                if buf.starts_with(b"let") {
                    Some(3)
                } else {
                    None
                }
            },
        },
        Matcher {
            kind: TokenKind::Whitespace,
            is_produce: false,
            is_match: |buf| {
                let mut i = 0;
                while i < buf.len() {
                    let mut found = false;
                    for ws in WHITESPACES.iter() {
                        if buf[i..].starts_with(ws.as_bytes()) {
                            i += ws.len();
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        break;
                    }
                }
                if i > 0 {
                    Some(i)
                } else {
                    None
                }
            },
        },
    ]
}

#[derive(Debug, Clone, Copy)]
enum TokenKind {
    KeywordLet,
    Whitespace,
}

type Position = usize;
type Length = usize;
type Start = Position;
type End = Position;

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
    pub(crate) is_produce: bool,
    pub(crate) is_match: fn(&[u8]) -> Option<Length>,
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    span: Span,
    value: Option<Vec<u8>>,
}

#[instrument]
fn do_match(file_position: Position, buffer: &[u8]) -> Option<(Length, Option<Token>)> {
    for matcher in MATCHERS.iter() {
        if let Some(end) = (matcher.is_match)(buffer) {
            let token = if matcher.is_produce {
                Some(Token {
                    kind: matcher.kind,
                    span: Span {
                        start: file_position,
                        end: file_position + end,
                    },
                    // TODO: copy the value from buffer
                    value: None,
                })
            } else {
                None
            };
            return Some((end, token));
        }
    }
    None
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
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn let_foo_0() {
        let token_buffer = String::from("let foo = 0").as_bytes();
    }
}

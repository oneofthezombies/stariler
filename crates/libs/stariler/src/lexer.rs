use tracing::instrument;

#[derive(Debug)]
enum TokenKind {
    Identifier,
    Keyword,
}

/// It's a half-open interval.  
/// `start` is inclusive, `end` is exclusive.
#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    span: Span,
}

#[instrument]
async fn tokenize(path: std::path::PathBuf) -> crate::Result<()> {
    use tokio::io::AsyncReadExt;

    const BUFFER_SIZE: usize = 4 * 1024;
    let mut file = tokio::fs::File::open(&path).await?;
    let mut untokenized = Vec::new();
    let mut buffer = [0; BUFFER_SIZE];
    loop {
        let n = file.read(&mut buffer[..]).await?;
        if n == 0 {
            break;
        }
        untokenized.extend_from_slice(&buffer[..n]);
        parse(&untokenized)?;
    }
    Ok(())
}

#[instrument]
fn parse(bytes: &[u8]) -> crate::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn single_quote_string() {
        const SOURCE: &str = r#"let s = 'hello';"#;
        parse(SOURCE.as_bytes()).unwrap();
    }
}

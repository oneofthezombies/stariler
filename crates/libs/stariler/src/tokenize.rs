use tracing::instrument;

#[instrument]
async fn tokenize(path: std::path::PathBuf) -> crate::Result<()> {
    use tokio::io::AsyncReadExt;

    const BUFFER_SIZE: usize = 4 * 1024;
    let mut file = tokio::fs::File::open(&path).await?;
    let mut untokenized = Vec::with_capacity(BUFFER_SIZE);
    let mut buffer = [0; BUFFER_SIZE];
    loop {
        let n = file.read(&mut buffer[..]).await?;
        if n == 0 {
            break;
        }
        untokenized.extend_from_slice(&buffer[..n]);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run() {
        // let path = std::path::PathBuf::from(
        //     "/Users/hunhoekim/repo/stariler/references/sample/src/index.ts",
        // );
        // let () = tokenize(path).await.unwrap();
    }
}

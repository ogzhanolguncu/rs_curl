use std::error::Error;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::cli_parser::UrlSections;

const DEFAULT_PORT: u16 = 80;

pub async fn make_call_to(req: UrlSections) -> Result<String, Box<dyn Error>> {
    let port = req.port.unwrap_or_else(|| DEFAULT_PORT);
    let mut stream = TcpStream::connect(format!("{}:{}", req.host, port)).await?;

    let request = format!(
        "GET {} HTTP/1.1\r\n\
                       Host: {} \r\n\
                       Accept: */*\r\n\
                       Connection: close\r\n\
                       \r\n",
        req.path, req.host
    );

    stream.write_all(request.as_bytes()).await?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await?;

    let response = String::from_utf8_lossy(&buffer);
    println!("Received response:\n{}", response);
    Ok(response.into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_return_success_response() {
        assert!(make_call_to(UrlSections {
            protocol: "http".to_string(),
            host: "httpbin.org".to_string(),
            port: Some(80),
            path: "/get".to_string()
        })
        .await
        .unwrap()
        .contains("http://httpbin.org/get"));
    }
}

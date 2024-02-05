use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, Value};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::cli_parser::ParsedArgs;

const DEFAULT_PORT: u16 = 80;

pub async fn make_call_to(req: ParsedArgs) -> Result<String, Box<dyn Error>> {
    let ParsedArgs {
        url_sections,
        verbose,
    } = req;
    let port = url_sections.port.unwrap_or_else(|| DEFAULT_PORT);
    let mut stream = TcpStream::connect(format!("{}:{}", url_sections.host, port)).await?;

    let request = format!(
        "GET {} HTTP/1.1\r\n\
                       Host: {} \r\n\
                       Accept: */*\r\n\
                       Connection: close\r\n\
                       \r\n",
        url_sections.path, url_sections.host
    );

    stream.write_all(request.as_bytes()).await?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await?;

    let response = String::from_utf8_lossy(&buffer);
    if verbose {
        println!("{}", response);
    } else {
        if let Some(response) = remove_headers(&response) {
            println!("{}", response);
        }
    }

    Ok(response.into_owned())
}

fn remove_headers(response: &std::borrow::Cow<'_, str>) -> Option<String> {
    if let Some(json_start) = response.find('{') {
        let json_str = &response[json_start..];

        let mut parsed: serde_json::Value =
            serde_json::from_str(json_str).expect("JSON parsing error");

        // Remove the "headers" key from the JSON data
        if let Value::Object(ref mut obj) = parsed {
            obj.remove("headers");
        }

        let pretty_json = to_string_pretty(&parsed).expect("Failed to serialize to pretty string");
        Some(pretty_json)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::cli_parser::UrlSections;

    use super::*;

    #[tokio::test]
    async fn should_return_success_response() {
        assert!(make_call_to(ParsedArgs {
            url_sections: UrlSections {
                protocol: "http".to_string(),
                host: "httpbin.org".to_string(),
                port: Some(80),
                path: "/get".to_string()
            },
            verbose: false
        })
        .await
        .unwrap()
        .contains("http://httpbin.org/get"));
    }

    #[tokio::test]
    async fn should_return_success_response_with_headers() {
        assert!(make_call_to(ParsedArgs {
            url_sections: UrlSections {
                protocol: "http".to_string(),
                host: "httpbin.org".to_string(),
                port: Some(80),
                path: "/get".to_string()
            },
            verbose: true
        })
        .await
        .unwrap()
        .contains("headers"));
    }
}

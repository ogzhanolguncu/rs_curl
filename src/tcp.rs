use std::error::Error;

use native_tls::TlsConnector as NativeTlsConnector;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_native_tls::TlsConnector;

use crate::cli_parser::ParsedArgs;

const DEFAULT_PORT: u16 = 443;

pub async fn make_call_to(req: ParsedArgs) -> Result<String, Box<dyn Error>> {
    let ParsedArgs {
        url_sections,
        verbose,
        method,
        data,
        header,
    } = req;
    let port = url_sections.port.unwrap_or_else(|| DEFAULT_PORT);

    let stream = TcpStream::connect(format!("{}:{}", url_sections.host, port)).await?;

    let connector = NativeTlsConnector::new().unwrap();
    let connector = TlsConnector::from(connector);

    let domain = url_sections.host.as_str();
    let mut stream = connector.connect(domain, stream).await?;

    let request = format!(
        "{} {} HTTP/1.1\r\n\
        Host: {} \r\n\
        {}{} \r\n\
        Accept: */*\r\n\
        Connection: close\r\n\
        \r\n\
        {}",
        method.to_string(),
        url_sections.path,
        url_sections.host,
        header.unwrap_or_default(),
        data.as_ref().map_or(String::new(), |d| format!(
            "\r\nContent-Length: {}",
            d.len()
        )),
        data.unwrap_or_default()
    );

    stream.write_all(request.as_bytes()).await?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await?;

    let mut response = String::from_utf8_lossy(&buffer).to_string();

    if verbose {
        response = add_incoming_sign(&response);
        println!("{}", response);
    } else {
        if let Some(response) = remove_headers(&response) {
            println!("{}", response);
        }
    }

    Ok(response)
}

fn add_incoming_sign(response: &str) -> String {
    let mut final_str = String::new();
    if let Some((info, json)) = split_http_response(response.to_string().as_str()) {
        for line in info.lines() {
            final_str.push_str(&format!("< {}\n", line));
        }
        final_str.push_str("<\n");
        final_str.push_str(json);
    }
    final_str
}

fn remove_headers(response: &str) -> Option<&str> {
    if let Some(json_start) = response.find('{') {
        let json_str = &response[json_start..];

        Some(json_str)
    } else {
        None
    }
}

fn split_http_response(http_response: &str) -> Option<(&str, &str)> {
    http_response.split_once("\r\n\r\n")
}

#[cfg(test)]
mod tests {
    use crate::cli_parser::{UrlSections, METHOD};

    use super::*;

    #[tokio::test]
    async fn should_return_success_response() {
        assert!(make_call_to(ParsedArgs {
            url_sections: UrlSections {
                protocol: "http".to_string(),
                host: "httpbin.org".to_string(),
                port: Some(80),
                path: "/get".to_string(),
            },
            verbose: false,
            method: METHOD::GET,
            data: None,
            header: None
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
            verbose: true,
            method: METHOD::GET,
            data: None,
            header: None
        })
        .await
        .unwrap()
        .contains("headers"));
    }

    #[tokio::test]
    async fn should_prepend_less_than_to_verbose_headers() {
        assert!(make_call_to(ParsedArgs {
            url_sections: UrlSections {
                protocol: "http".to_string(),
                host: "httpbin.org".to_string(),
                port: Some(80),
                path: "/get".to_string()
            },
            verbose: true,
            method: METHOD::GET,
            data: None,
            header: None
        })
        .await
        .unwrap()
        .contains("<"));
    }
}

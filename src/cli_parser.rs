use clap::{command, Parser};
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,
}

pub fn parser() -> Result<UrlSections, &'static str> {
    let args: Args = Args::parse();
    let parsed_url = parse_url(args.url.clone())?;

    println!("connecting to {}", args.url);
    println!(
        "Sending request {} {} {}",
        "GET", parsed_url.path, "HTTP/1.1"
    );
    println!("Host: {}", parsed_url.host);
    println!("Accept: */*",);
    println!("");

    return Ok(parsed_url);
}

#[derive(PartialEq, Debug)]
pub struct UrlSections {
    protocol: String,
    host: String,
    port: Option<u16>,
    path: String,
}

fn parse_url(url: String) -> Result<UrlSections, &'static str> {
    let pattern =
        Regex::new(r"(?P<protocol>https?)://(?P<host>[^:/]+)(?::(?P<port>\d+))?(/(?P<path>.*))?");

    match pattern {
        Ok(res) => {
            if let Some(captures) = res.captures(url.as_str()) {
                let protocol = captures.name("protocol").unwrap().as_str().to_string();
                let host = captures.name("host").unwrap().as_str().to_string();
                let path = match captures.name("path") {
                    Some(path) => path.as_str().to_string(),
                    None => "/".to_string(),
                };
                let port = captures.name("port").and_then(|x| x.as_str().parse().ok());

                Ok(UrlSections {
                    host,
                    path,
                    protocol,
                    port,
                })
            } else {
                Err("ERR - Couldn't find anything to parse")
            }
        }
        Err(_) => Err("ERR - URL is not parseable"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions_sorted::assert_eq;

    #[test]
    fn should_parse_url_with_port_and_path() {
        let url = "http://ogzhanolguncu.com:80/best-typescript-types".to_string();

        assert_eq!(
            parse_url(url).unwrap(),
            UrlSections {
                host: "ogzhanolguncu.com".to_string(),
                path: "best-typescript-types".to_string(),
                protocol: "http".to_string(),
                port: Some(80)
            }
        )
    }

    #[test]
    fn should_parse_url_without_port_and_path() {
        let url = "https://ogzhanolguncu.com".to_string();

        assert_eq!(
            parse_url(url).unwrap(),
            UrlSections {
                host: "ogzhanolguncu.com".to_string(),
                path: "/".to_string(),
                protocol: "https".to_string(),
                port: None
            }
        )
    }
}

use core::fmt;

use clap::{command, Parser, ValueEnum};
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long,value_enum, default_value_t=METHOD::GET)]
    x: METHOD,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum METHOD {
    DELETE,
    GET,
    POST,
    PUT,
}

pub struct ParsedArgs {
    pub url_sections: UrlSections,
    pub verbose: bool,
    pub method: METHOD,
}

pub fn parser() -> Result<ParsedArgs, &'static str> {
    let args: Args = Args::parse();
    let parsed_url = parse_url(args.url.clone())?;
    let is_verbose = args.verbose;
    let method = args.x;

    println!("connecting to {}", args.url);
    println!("{} {} {}", method, parsed_url.path, "HTTP/1.1");
    println!("Host: {}", parsed_url.host);
    println!("Accept: */*",);
    println!("");

    return Ok(ParsedArgs {
        url_sections: parsed_url,
        verbose: is_verbose,
        method,
    });
}

#[derive(PartialEq, Debug, Clone)]
pub struct UrlSections {
    pub protocol: String,
    pub host: String,
    pub port: Option<u16>,
    pub path: String,
}

fn parse_url(url: String) -> Result<UrlSections, &'static str> {
    let pattern =
        Regex::new(r"(?P<protocol>https?)://(?P<host>[^:/]+)(?::(?P<port>\d+))?(?P<path>/.*)?");

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
                path: "/best-typescript-types".to_string(),
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

impl fmt::Display for METHOD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

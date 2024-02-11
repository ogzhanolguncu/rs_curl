# Rust-based Curl Clone

This Curl Clone is a Rust command-line application for making HTTP requests. It combines async(tokio) rust with secure TLS connections.

## Features

- Supports HTTP and HTTPS protocols
- Allows GET, POST, PUT, DELETE methods
- Customizable request headers and body data
- Verbose mode for detailed request/response headers
- Automatic URL parsing into components
- Secure TLS communication for HTTPS requests
- Asynchronous I/O for enhanced performance

## Usage

```bash
Usage: rs_curl [OPTIONS] --url <URL>

Options:
  -u, --url <URL>
  -v, --verbose
  -h, --header <HEADER>
  -d, --data <DATA>
      --help
  -x, --x <X>            [default: get] [possible values: delete, get, post, put]
  -V, --version          Print version


 ✘ cargo run -- -u https://httpbin.org/get -v;
```

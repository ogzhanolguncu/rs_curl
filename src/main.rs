use cli_parser::parser;

pub mod cli_parser;

#[tokio::main]
async fn main() {
    let url_sections = parser();
    match url_sections {
        Ok(res) => todo!(),
        Err(_) => todo!(),
    }
}

pub mod cli_parser;
pub mod tcp;

use cli_parser::parser;
use tcp::make_call_to;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed_cli_values = parser()?;
    make_call_to(parsed_cli_values).await?;
    Ok(())
}

use components::prompt;

mod bash_parser;
mod components;
mod config;

fn main() -> std::io::Result<()> {
    prompt::prompt()?;
    Ok(())
}

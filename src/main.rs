use components::prompt;

mod bash_parser;
mod components;

fn main() -> std::io::Result<()> {
    prompt::prompt()?;
    Ok(())
}

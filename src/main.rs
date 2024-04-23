use components::prompt;

mod code;
mod components;

fn main() -> std::io::Result<()> {
    prompt::prompt()?;
    Ok(())
}

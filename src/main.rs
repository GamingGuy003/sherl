use components::prompt;

mod components;

fn main() -> std::io::Result<()> {
    loop {
        prompt::prompt()?;
    }
    Ok(())
}
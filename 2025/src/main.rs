use aoc2025::aoc;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    aoc!(2025, 11);
    Ok(())
}

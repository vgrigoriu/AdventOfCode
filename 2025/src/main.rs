use aoc::aoc;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    aoc!(2025, 11);
    Ok(())
}

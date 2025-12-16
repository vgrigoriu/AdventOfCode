pub mod dial;
pub mod solver;
pub mod tile;
pub mod utilities;
pub mod year2015;
pub mod year2016;
pub mod year2025;

#[macro_export]
macro_rules! aoc {
    ($year:literal, 1) => {
        aoc!($year, 01)
    };
    ($year:literal, 2) => {
        aoc!($year, 02)
    };
    ($year:literal, 3) => {
        aoc!($year, 03)
    };
    ($year:literal, 4) => {
        aoc!($year, 04)
    };
    ($year:literal, 5) => {
        aoc!($year, 05)
    };
    ($year:literal, 6) => {
        aoc!($year, 06)
    };
    ($year:literal, 7) => {
        aoc!($year, 07)
    };
    ($year:literal, 8) => {
        aoc!($year, 08)
    };
    ($year:literal, 9) => {
        aoc!($year, 09)
    };
    ($year:literal, $day:tt) => {
        ::paste::paste! {
            use aoc2025::[<year $year>]::[<day $day>] as solution;
            println!("{}", solution::solve1()?);
            println!("{}", solution::solve2()?);
        }
    };
}

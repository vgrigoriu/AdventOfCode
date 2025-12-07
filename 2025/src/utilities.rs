pub fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn map_grid<T, U, F>(grid: &[Vec<T>], f: F) -> Vec<Vec<U>>
where
    F: Fn(&T) -> U,
{
    grid.iter()
        .map(|row| row.iter().map(&f).collect())
        .collect()
}

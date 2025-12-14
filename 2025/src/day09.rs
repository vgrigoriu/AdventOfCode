const INPUT: &str = include_str!("../input/day09.in");

pub fn solve1() {
    let tiles: Vec<(u64, u64)> = INPUT
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(",").collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect();

    let mut max_size = 0;
    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let tile1 = tiles[i];
            let tile2 = tiles[j];
            let current_size = (tile1.0.abs_diff(tile2.0) + 1) * (tile1.1.abs_diff(tile2.1) + 1);
            if max_size < current_size {
                max_size = current_size;
            }
        }
    }

    println!("{max_size}");
}

pub fn solve2() {
    let tiles: Vec<(usize, usize)> = INPUT
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(",").collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect();

    let v_lines: Vec<_> = (0..tiles.len())
        .filter_map(|i| VLine::from_tiles(tiles[i], tiles[(i + 1) % tiles.len()]))
        .collect();
    let h_lines: Vec<_> = (0..tiles.len())
        .filter_map(|i| HLine::from_tiles(tiles[i], tiles[(i + 1) % tiles.len()]))
        .collect();

    let mut rects = vec![];
    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let tile1 = tiles[i];
            let tile2 = tiles[j];

            let rect = Rect::new(tile1, tile2);
            rects.push(rect);
        }
    }
    rects.sort_by_key(|r| -(r.size() as i64));

    let the_one = rects
        .iter()
        .filter(|rect| {
            !(v_lines.iter().any(|l| rect.intersects_v(l))
                || h_lines.iter().any(|l| rect.intersects_h(l)))
        })
        .next()
        .unwrap();

    println!("{}", the_one.size());
}

#[derive(Copy, Clone, Debug)]
struct HLine {
    y: usize,
    left: usize,
    right: usize,
}

impl HLine {
    fn from_tiles(tile1: (usize, usize), tile2: (usize, usize)) -> Option<Self> {
        if tile1.1 != tile2.1 {
            None
        } else {
            Some(HLine {
                y: tile1.1,
                left: tile1.0.min(tile2.0),
                right: tile1.0.max(tile2.0),
            })
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct VLine {
    x: usize,
    top: usize,
    bottom: usize,
}

impl VLine {
    fn from_tiles(tile1: (usize, usize), tile2: (usize, usize)) -> Option<Self> {
        if tile1.0 != tile2.0 {
            None
        } else {
            Some(VLine {
                x: tile1.0,
                top: tile1.1.min(tile2.1),
                bottom: tile1.1.max(tile2.1),
            })
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Rect {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
}

impl Rect {
    fn new(tile1: (usize, usize), tile2: (usize, usize)) -> Self {
        Rect {
            top: tile1.1.min(tile2.1),
            bottom: tile1.1.max(tile2.1),
            left: tile1.0.min(tile2.0),
            right: tile1.0.max(tile2.0),
        }
    }

    fn size(&self) -> usize {
        (self.right - self.left + 1) * (self.bottom - self.top + 1)
    }

    fn intersects_v(self, line: &VLine) -> bool {
        self.left < line.x
            && line.x < self.right
            && self.top < line.bottom
            && line.top < self.bottom
    }

    fn intersects_h(self, line: &HLine) -> bool {
        self.top < line.y
            && line.y < self.bottom
            && self.left < line.right
            && line.left < self.right
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn intersects_h(rect: &Rect, line: &HLine) -> bool {
        let mut grid = vec![vec!['.'; 10]; 10];

        for x in rect.left..rect.right {
            grid[rect.top][x] = '#';
        }
        for y in rect.top..rect.bottom {
            grid[y][rect.right] = '#';
        }
        for x in (rect.left..rect.right).rev() {
            grid[rect.bottom][x + 1] = '#';
        }
        for y in (rect.top..rect.bottom).rev() {
            grid[y + 1][rect.left] = '#';
        }

        for x in line.left..=line.right {
            grid[line.y][x] = '-';
        }

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                print!("{}", grid[y][x]);
            }
            println!();
        }
        let result = rect.intersects_h(line);
        println!("Intesects: {result}\n");

        result
    }

    fn intersects_v(rect: &Rect, line: &VLine) -> bool {
        let mut grid = vec![vec!['.'; 10]; 10];

        for x in rect.left..rect.right {
            grid[rect.top][x] = '#';
        }
        for y in rect.top..rect.bottom {
            grid[y][rect.right] = '#';
        }
        for x in (rect.left..rect.right).rev() {
            grid[rect.bottom][x + 1] = '#';
        }
        for y in (rect.top..rect.bottom).rev() {
            grid[y + 1][rect.left] = '#';
        }

        for y in line.top..=line.bottom {
            grid[y][line.x] = '|';
        }

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                print!("{}", grid[y][x]);
            }
            println!();
        }
        let result = rect.intersects_v(line);
        println!("Intesects: {result}\n");

        result
    }

    #[test]
    fn test_intersects_h() {
        let rect = Rect::new((3, 3), (6, 6));

        let line = HLine::from_tiles((0, 0), (3, 0)).unwrap();
        assert!(!intersects_h(&rect, &line));

        let line = HLine::from_tiles((0, 6), (5, 6)).unwrap();
        assert!(!intersects_h(&rect, &line));

        let line = HLine::from_tiles((0, 4), (5, 4)).unwrap();
        assert!(intersects_h(&rect, &line));

        let line = HLine::from_tiles((5, 5), (9, 5)).unwrap();
        assert!(intersects_h(&rect, &line));

        let line = HLine::from_tiles((0, 6), (9, 6)).unwrap();
        assert!(!intersects_h(&rect, &line));

        let line = HLine::from_tiles((0, 7), (9, 7)).unwrap();
        assert!(!intersects_h(&rect, &line));
    }

    #[test]
    fn test_intersects_v() {
        let rect = Rect::new((3, 3), (6, 6));

        let line = VLine::from_tiles((0, 0), (0, 7)).unwrap();
        assert!(!intersects_v(&rect, &line));

        let line = VLine::from_tiles((3, 4), (3, 5)).unwrap();
        assert!(!intersects_v(&rect, &line));

        let line = VLine::from_tiles((4, 1), (4, 3)).unwrap();
        assert!(!intersects_v(&rect, &line));

        let line = VLine::from_tiles((4, 1), (4, 4)).unwrap();
        assert!(intersects_v(&rect, &line));

        let line = VLine::from_tiles((4, 1), (4, 9)).unwrap();
        assert!(intersects_v(&rect, &line));

        let line = VLine::from_tiles((4, 4), (4, 9)).unwrap();
        assert!(intersects_v(&rect, &line));

        let line = VLine::from_tiles((4, 6), (4, 9)).unwrap();
        assert!(!intersects_v(&rect, &line));

        let line = VLine::from_tiles((7, 1), (7, 7)).unwrap();
        assert!(!intersects_v(&rect, &line));
    }
}

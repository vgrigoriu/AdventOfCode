const INPUT: &str = include_str!("../input/day09.in");

pub fn solve1() {
    let tiles: Vec<(u64, u64)> = INPUT.lines().map(|line|{
        let parts: Vec<_> = line.split(",").collect();
        (parts[0].parse().unwrap(), parts[1].parse().unwrap())
    })
    .collect();

    let mut max_size = 0;
    for i in 0..tiles.len() {
        for j in i+1..tiles.len() {
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
    let tiles: Vec<(usize, usize)> = INPUT.lines().map(|line|{
        let parts: Vec<_> = line.split(",").collect();
        (parts[0].parse().unwrap(), parts[1].parse().unwrap())
    })
    .collect();

    let v_lines: Vec<_> = (0..tiles.len()).filter_map(|i| VLine::from_tiles(tiles[i], tiles[(i+1)%tiles.len()])).collect();
    for l in &v_lines {
        println!("V: {} {}-{}", l.x, l.bottom, l.top);
    }
    let h_lines: Vec<_> = (0..tiles.len()).filter_map(|i| HLine::from_tiles(tiles[i], tiles[(i+1)%tiles.len()])).collect();
    for l in &h_lines {
        println!("H: {} {}-{}", l.y, l.left, l.right);
    }

    let mut max_size = 0;
    for i in 0..tiles.len() {
        for j in i+1..tiles.len() {
            let tile1 = tiles[i];
            let tile2 = tiles[j];

            let rect = Rect::new(tile1, tile2);
            let intersects = v_lines.iter().any(|&l| rect.intersects_v(l)) || h_lines.iter().any(|&l| rect.intersects_h(l));
            if intersects {
                println!("It intersects: {:?}", rect);
                continue;
            }

            let current_size = (tile1.0.abs_diff(tile2.0) + 1) * (tile1.1.abs_diff(tile2.1) + 1);
            if max_size < current_size {
                max_size = current_size;
            }
        }
    }

    println!("{max_size}");
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
            Some(HLine { y: tile1.1, left: tile1.0.min(tile2.0), right: tile1.0.max(tile2.0) })
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
            Some(VLine { x: tile1.0, top: tile1.1.max(tile2.1), bottom: tile1.1.min(tile2.1) })
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
    fn new(tile1: (usize, usize), tile2:(usize, usize)) -> Self {
        Rect {
            top: tile1.1.max(tile2.1),
            bottom: tile1.1.min(tile2.1),
            left: tile1.0.min(tile2.0),
            right: tile1.0.max(tile2.0),
        }
    }

    fn intersects_v(self, line: VLine) -> bool {
        let result = self.left < line.x && line.x < self.right && self.bottom < line.top && self.top < line.bottom;
        if result {
            println!("{:?} intersects {:?}", self, line);
        }
        result
    }

    fn intersects_h(self, line: HLine) -> bool {
        let result = self.bottom < line.y && line.y < self.top && self.left < line.right && line.left < self.right;
        if result {
            println!("{:?} intersects {:?}", self, line);
        }
        result
    }
}

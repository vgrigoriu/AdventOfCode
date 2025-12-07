use regex::Regex;

const INPUT: &str = include_str!("../../input/2016/day22.in");

pub fn solve1() {
    let nodes: Vec<Node> = INPUT.lines().skip(2).map(Node::from).collect();

    let mut no_viable_pairs = 0;
    for a in 0..nodes.len() {
        for b in 0..nodes.len() {
            if nodes[a].is_viable_pair(&nodes[b]) {
                no_viable_pairs += 1;
            }
        }
    }

    println!("{}", no_viable_pairs);
}

pub fn solve2() {
    let nodes: Vec<Node> = INPUT.lines().skip(2).map(Node::from).collect();
    let max_x = nodes.iter().map(|node| node.x).max().unwrap() as usize;
    let max_y = nodes.iter().map(|node| node.y).max().unwrap() as usize;

    let mut nodes_grid: Vec<_> = (0..=max_y)
        .map(|_| vec![Node::default(); max_x + 1])
        .collect();
    for &node in &nodes {
        nodes_grid[node.y][node.x] = node;
    }

    for (y, line) in nodes_grid.iter().enumerate() {
        for (x, node) in line.iter().enumerate() {
            if x == 0 && y == 0 {
                print!("(.)");
            } else if x == max_x && y == 0 {
                print!(" G ");
            } else if node.used == 0 {
                print!(" _ ");
            } else if node.used > 100 {
                print!(" # ");
            } else {
                print!(" . ");
            }
        }
        println!();
    }

    println!("{}", 33 + 5 * 35 + 1);
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Node {
    x: usize,
    y: usize,
    used: u16,
    avail: u16,
}

impl Node {
    fn from(line: &str) -> Node {
        // Filesystem              Size  Used  Avail  Use%
        // /dev/grid/node-x0-y0     86T   73T    13T   84%
        let re = Regex::new(r"/dev/grid/node-x(?<x>\d+)-y(?<y>\d+)\s+(?<size>\d+)T\s+(?<used>\d+)T\s+(?<avail>\d+)T").unwrap();

        let captures = re.captures(line).unwrap();
        Node {
            x: captures["x"].parse().unwrap(),
            y: captures["y"].parse().unwrap(),
            used: captures["used"].parse().unwrap(),
            avail: captures["avail"].parse().unwrap(),
        }
    }

    fn is_viable_pair(&self, other: &Self) -> bool {
        self.used > 0 && self != other && self.used <= other.avail
    }
}

impl Default for Node {
    fn default() -> Self {
        Node {
            x: 0,
            y: 0,
            used: 1000,
            avail: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_from() {
        assert_eq!(
            Node::from("/dev/grid/node-x0-y0     86T   73T    13T   84%"),
            Node {
                x: 0,
                y: 0,
                used: 73,
                avail: 13
            }
        );
    }
}

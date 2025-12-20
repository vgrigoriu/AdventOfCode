use color_eyre::{Result, eyre::eyre};
use std::collections::{HashMap, HashSet};

/// A directed graph parsed from adjacency list format.
///
/// Note: Does not verify acyclicity despite the name.
///
/// Input format:
/// ```text
/// node1: neighbor1 neighbor2
/// node2: neighbor3
/// ```
pub struct DAG<'a> {
    nodes: HashSet<&'a str>,
    edges: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> DAG<'a> {
    /// Parses a string into a DAG. Does not detect cycles.
    pub fn parse(s: &'a str) -> Result<Self> {
        let lines: Vec<_> = s
            .lines()
            .map(|l| l.split_once(": ").ok_or(eyre!("Line has no ': ': {l}")))
            .collect::<Result<_>>()?;

        let mut edges = HashMap::with_capacity(lines.len());
        let mut nodes = HashSet::new();
        for (node, neighbors) in lines {
            nodes.insert(node);
            let neighbors: Vec<_> = neighbors.split(' ').collect();
            for &neighbor in &neighbors {
                nodes.insert(neighbor);
            }
            edges.insert(node, neighbors);
        }

        Ok(Self { edges, nodes })
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn neighbors(&self, node: &str) -> &[&'a str] {
        self.edges.get(node).map(|n| n.as_slice()).unwrap_or(&[])
    }

    pub fn in_degrees(&self) -> HashMap<&'a str, usize> {
        let mut in_degrees: HashMap<_, _> = self.nodes.iter().map(|&n| (n, 0)).collect();
        for neighbors in self.edges.values() {
            for &neighbor in neighbors {
                *in_degrees.get_mut(neighbor).unwrap() += 1;
            }
        }
        in_degrees
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::Result;

    use crate::dag::DAG;

    #[test]
    fn parse_dags() -> Result<()> {
        let dag = DAG::parse(r"a: b")?;
        assert_eq!(dag.edges["a"], vec!["b"]);

        let dag = DAG::parse(r"a: b c")?;
        assert_eq!(dag.edges["a"], vec!["b", "c"]);

        let dag = DAG::parse(
            r"a: b c
b: c",
        )?;
        assert_eq!(dag.edges["a"], vec!["b", "c"]);
        assert_eq!(dag.edges["b"], vec!["c"]);

        Ok(())
    }

    #[test]
    fn parse_fails_on_invalid_input() {
        assert!(DAG::parse("invalid line without colon").is_err());
    }

    #[test]
    fn node_count_is_correct() -> Result<()> {
        let dag = DAG::parse(
            r"a: b
c: d",
        )?;
        assert_eq!(dag.node_count(), 4);

        let dag = DAG::parse(
            r"a: b
c: b",
        )?;
        assert_eq!(dag.node_count(), 3);

        let dag = DAG::parse(
            r"a: b
b: c",
        )?;
        assert_eq!(dag.node_count(), 3);

        let dag = DAG::parse(
            r"a: b c d
b: c
c: d e f",
        )?;
        assert_eq!(dag.node_count(), 6);
        Ok(())
    }

    #[test]
    fn test_in_degrees() -> Result<()> {
        let in_degrees = DAG::parse(
            r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out",
        )?
        .in_degrees();

        assert_eq!(in_degrees["you"], 1);
        assert_eq!(in_degrees["aaa"], 0);
        assert_eq!(in_degrees["bbb"], 1);
        assert_eq!(in_degrees["ccc"], 2);
        assert_eq!(in_degrees["ddd"], 2);
        assert_eq!(in_degrees["eee"], 2);
        assert_eq!(in_degrees["fff"], 2);
        assert_eq!(in_degrees["ggg"], 1);
        assert_eq!(in_degrees["hhh"], 1);
        assert_eq!(in_degrees["out"], 4);

        let in_degrees = DAG::parse(
            r"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out",
        )?
        .in_degrees();

        assert_eq!(in_degrees["svr"], 0);
        assert_eq!(in_degrees["aaa"], 1);
        assert_eq!(in_degrees["bbb"], 1);
        assert_eq!(in_degrees["ccc"], 2);
        assert_eq!(in_degrees["ddd"], 1);
        assert_eq!(in_degrees["eee"], 1);
        assert_eq!(in_degrees["fff"], 2);
        assert_eq!(in_degrees["ggg"], 1);
        assert_eq!(in_degrees["hhh"], 1);
        assert_eq!(in_degrees["fft"], 1);
        assert_eq!(in_degrees["tty"], 1);
        assert_eq!(in_degrees["hub"], 1);
        assert_eq!(in_degrees["dac"], 1);
        assert_eq!(in_degrees["out"], 2);

        Ok(())
    }

    #[test]
    fn neighbors_returns_correct_values() -> Result<()> {
        let dag = DAG::parse("a: b c\nb: d")?;
        assert_eq!(dag.neighbors("a"), &["b", "c"]);
        assert_eq!(dag.neighbors("b"), &["d"]);
        assert_eq!(dag.neighbors("c"), &[] as &[&str]);
        Ok(())
    }
}

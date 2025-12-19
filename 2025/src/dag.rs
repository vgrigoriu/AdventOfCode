use color_eyre::{Result, eyre::eyre};
use std::collections::{HashMap, HashSet};

pub struct DAG<'a> {
    nodes: HashSet<&'a str>,
    edges: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> DAG<'a> {
    pub fn parse(s: &'a str) -> Result<Self> {
        // e.g.:
        // aaa: you hhh
        // you: bbb ccc
        // bbb: ddd eee
        // ccc: ddd eee fff
        // ddd: ggg
        // eee: out
        // fff: out
        // ggg: out
        // hhh: ccc fff iii
        // iii: out
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

    pub fn no_of_nodes(&self) -> usize {
        // Not strictly correct, there might be nodes with no outgoing edges,
        // which won't appear as keys in the `edges` hashmap.
        self.nodes.len()
    }

    pub fn meighbors(&self, node: &str) -> &Vec<&'a str> {
        &self.edges[node]
    }

    pub fn incoming_edges(&self) -> HashMap<&'a str, usize> {
        let mut incoming_edges: HashMap<_, _> = self.nodes.iter().map(|&n| (n, 0)).collect();
        for neighbors in self.edges.values() {
            for &neighbor in neighbors {
                *incoming_edges.entry(neighbor).or_default() += 1;
            }
        }
        incoming_edges
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
    fn no_of_edges_is_correct() -> Result<()> {
        let dag = DAG::parse(
            r"a: b
c: d",
        )?;
        assert_eq!(dag.no_of_nodes(), 4);

        let dag = DAG::parse(
            r"a: b
c: b",
        )?;
        assert_eq!(dag.no_of_nodes(), 3);

        let dag = DAG::parse(
            r"a: b
b: c",
        )?;
        assert_eq!(dag.no_of_nodes(), 3);

        let dag = DAG::parse(
            r"a: b c d
b: c
c: d e f",
        )?;
        assert_eq!(dag.no_of_nodes(), 6);
        Ok(())
    }

    #[test]
    fn test_incoming_edges() -> Result<()> {
        let edges = DAG::parse(
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
        .incoming_edges();

        assert_eq!(edges["you"], 1);
        assert_eq!(edges["aaa"], 0);
        assert_eq!(edges["bbb"], 1);
        assert_eq!(edges["ccc"], 2);
        assert_eq!(edges["ddd"], 2);
        assert_eq!(edges["eee"], 2);
        assert_eq!(edges["fff"], 2);
        assert_eq!(edges["ggg"], 1);
        assert_eq!(edges["hhh"], 1);
        assert_eq!(edges["out"], 4);

        let edges = DAG::parse(
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
        .incoming_edges();

        assert_eq!(edges["svr"], 0);
        assert_eq!(edges["aaa"], 1);
        assert_eq!(edges["bbb"], 1);
        assert_eq!(edges["ccc"], 2);
        assert_eq!(edges["ddd"], 1);
        assert_eq!(edges["eee"], 1);
        assert_eq!(edges["fff"], 2);
        assert_eq!(edges["ggg"], 1);
        assert_eq!(edges["hhh"], 1);
        assert_eq!(edges["fft"], 1);
        assert_eq!(edges["tty"], 1);
        assert_eq!(edges["hub"], 1);
        assert_eq!(edges["dac"], 1);
        assert_eq!(edges["out"], 2);

        Ok(())
    }
}

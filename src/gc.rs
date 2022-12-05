use crate::Sodg;
use anyhow::{Context, Result};
use std::collections::VecDeque;

impl Sodg {
    pub fn collect(&mut self, v: u32) -> Result<()> {
        let mut queue = VecDeque::new();
        queue.push_back(v);
        while !queue.is_empty() {
            let collected_vertex_id = queue
                .pop_front()
                .context("A non-empty queue failed to yield an element, this shouldn't happen.")?;
            let collected_vertex = self
                .vertices
                .get(&collected_vertex_id)
                .context(format!("Failed to get v{collected_vertex_id}"))?
                .clone();
            if !collected_vertex.parents.is_empty() {
                continue;
            } else {
                for edge in &collected_vertex.edges {
                    queue.push_back(edge.to);
                    self.vertices
                        .get_mut(&edge.to)
                        .context(format!("Failed to get v{}", edge.to))?
                        .parents
                        .remove(&collected_vertex_id);
                }
                self.vertices.remove(&collected_vertex_id);
            }
        }
        Ok(())
    }
}

#[test]
fn does_not_collect_owned() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    g.bind(1, 2, "x")?;
    g.collect(2)?;
    assert!(g.vertices.get(&2).is_some());
    Ok(())
}

#[test]
fn collects_simple_graph() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    g.add(3)?;
    g.add(4)?;
    g.bind(1, 2, "x")?;
    g.bind(1, 3, "y")?;
    g.bind(2, 4, "z")?;
    g.collect(1)?;
    assert!(g.is_empty());
    Ok(())
}

#[test]
fn collects_complicated_graph() -> Result<()> {
    let mut g = Sodg::empty();
    for i in 1..=5 {
        g.add(i)?;
    }
    g.bind(1, 2, "x")?;
    g.bind(1, 3, "y")?;
    g.bind(2, 4, "z")?;
    g.bind(3, 5, "a")?;
    g.bind(4, 3, "b")?;
    g.collect(1)?;
    assert!(g.is_empty());
    Ok(())
}

use crate::Sodg;

#[cfg(feature = "gc")]
use anyhow::{Context, Result};

#[cfg(feature = "gc")]
use std::collections::VecDeque;

#[cfg(feature = "gc")]
use log::trace;

impl Sodg {
    /// Attempt to collect the vertex (delete it from the graph).
    ///
    /// If there are no edges leading to it, then it is deleted and
    /// all its children are collected.
    /// Otherwise, nothing happens. For example:
    ///
    /// ```
    /// use sodg::{Hex, Sodg};
    /// let mut g = Sodg::empty();
    /// g.add(1).unwrap();
    /// g.put(1, Hex::from(0)).unwrap();
    /// g.add(2).unwrap();
    /// g.put(2, Hex::from(0)).unwrap();
    /// g.bind(1, 2, "x").unwrap();
    /// g.data(2).unwrap(); // Try to collect 2
    /// assert!(g.data(2).is_ok());
    /// g.data(1).unwrap(); // Successfully collect 1
    /// assert!(g.data(1).is_err());
    /// ```
    /// # Errors
    ///
    /// If something goes wrong, an error may be returned.
    #[cfg(feature = "gc")]
    pub(crate) fn collect(&mut self, start: u32) -> Result<()> {
        let mut queue = VecDeque::new();
        queue.push_back(start);
        while !queue.is_empty() {
            let v = queue
                .pop_front()
                .context("A non-empty queue failed to yield an element, this shouldn't happen")?;
            let vtx = self
                .vertices
                .get(&v)
                .context(format!("Failed to get v{v}"))?
                .clone();
            if vtx.parents.is_empty() && vtx.taken {
                for edge in &vtx.edges {
                    queue.push_back(edge.to);
                    self.vertices
                        .get_mut(&edge.to)
                        .context(format!("Failed to get v{}", edge.to))?
                        .parents
                        .remove(&v);
                    trace!("#collect: ν{v} removed");
                }
                self.vertices.remove(&v);
            }
        }
        Ok(())
    }
}

#[test]
#[cfg(feature = "gc")]
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
#[cfg(feature = "gc")]
fn collects_simple_graph() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    g.add(3)?;
    g.add(4)?;
    g.bind(1, 2, "x")?;
    g.bind(1, 3, "y")?;
    g.bind(2, 4, "z")?;
    g.data(4)?;
    g.data(2)?;
    g.data(1)?;
    g.data(3)?;
    assert!(g.is_empty());
    Ok(())
}

#[test]
#[cfg(feature = "gc")]
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
    for i in 1..=5 {
        g.data(i)?;
    }
    assert!(g.is_empty());
    Ok(())
}

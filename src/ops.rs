// Copyright (c) 2022 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::edge::Edge;
use crate::Sodg;
use crate::Vertex;
use anyhow::{anyhow, Context, Result};
use log::trace;
use std::collections::VecDeque;
use std::str::FromStr;

impl Sodg {
    /// Add a new vertex `v1` to the Sodg:
    ///
    /// ```
    /// use sodg::Sodg;
    /// let mut sodg = Sodg::empty();
    /// sodg.add(0).unwrap();
    /// sodg.add(42).unwrap();
    /// sodg.bind(0, 42, "hello").unwrap();
    /// ```
    pub fn add(&mut self, v1: u32) -> Result<()> {
        if self.vertices.contains_key(&v1) {
            return Err(anyhow!("Vertex ν{} already exists", v1));
        }
        self.vertices.insert(v1, Vertex::empty());
        self.validate(vec![v1])?;
        trace!("#add(ν{}): new vertex added", v1);
        Ok(())
    }

    /// Makes an edge `e1` from vertex `v1` to vertex `v2` and puts `a` label on it. If the
    /// label is not equal to `"ρ"`, makes two backward edges from `v2` to `v1`
    /// and label them as `"ρ"` an `"𝜎"`.
    ///
    /// ```
    /// use sodg::Sodg;
    /// let mut sodg = Sodg::empty();
    /// sodg.add(0).unwrap();
    /// sodg.add(42).unwrap();
    /// sodg.bind(0, 42, "forward").unwrap();
    /// sodg.bind(42, 0, "backward").unwrap();
    /// ```
    pub fn bind(&mut self, v1: u32, v2: u32, a: &str) -> Result<()> {
        if v1 == v2 {
            return Err(anyhow!(
                "An edge can't depart from ν{} and arrive to itself",
                v1
            ));
        }
        if a.is_empty() {
            return Err(anyhow!(
                "Edge label can't be empty, from ν{} to ν{}",
                v1,
                v2
            ));
        }
        if !self.vertices.contains_key(&v2) {
            return Err(anyhow!("Can't arrive to ν{}, it's absent", v2));
        }
        let vtx1 = self
            .vertices
            .get_mut(&v1)
            .context(format!("Can't depart from ν{}, it's absent", v1))?;
        vtx1.edges.retain(|e| e.a != a);
        vtx1.edges.push(Edge::new(v2, a));
        self.validate(vec![v1, v2])?;
        trace!("#bind: edge added ν{}-{}->ν{}", v1, a, v2);
        Ok(())
    }

    /// Set vertex data.
    ///
    /// ```
    /// use sodg::Sodg;
    /// let mut sodg = Sodg::empty();
    /// sodg.add(42).unwrap();
    /// sodg.put(42, "hello, world!".as_bytes().to_vec()).unwrap();
    /// ```
    pub fn put(&mut self, v: u32, d: Vec<u8>) -> Result<()> {
        let vtx = self
            .vertices
            .get_mut(&v)
            .context(format!("Can't find ν{}", v))?;
        vtx.data = d.clone();
        self.validate(vec![v])?;
        trace!("#data: data of ν{} set to {}b", v, d.len());
        Ok(())
    }

    /// Read vertex data.
    ///
    /// ```
    /// use sodg::Sodg;
    /// let mut sodg = Sodg::empty();
    /// sodg.add(42).unwrap();
    /// let data : &[u8] = "hello, world!".as_bytes();
    /// sodg.put(42, data.to_vec()).unwrap();
    /// assert_eq!(data, sodg.data(42).unwrap());
    /// ```
    pub fn data(&self, v: u32) -> Result<Vec<u8>> {
        let vtx = self
            .vertices
            .get(&v)
            .context(format!("Can't find ν{}", v))?;
        Ok(vtx.data.clone())
    }

    /// Find all kids of a vertex.
    pub fn kids(&self, v: u32) -> Result<Vec<(String, u32)>> {
        let vtx = self.vertices.get(&v).context(format!("Can't find ν{v}"))?;
        Ok(vtx.edges.iter().map(|x| (x.a.clone(), x.to)).collect())
    }

    /// Find a kid of a vertex, by its edge name.
    ///
    /// ```
    /// use sodg::Sodg;
    /// let mut sodg = Sodg::empty();
    /// sodg.add(0).unwrap();
    /// sodg.add(42).unwrap();
    /// sodg.bind(0, 42, "k").unwrap();
    /// assert_eq!(42, sodg.kid(0, "k").unwrap());
    /// assert!(sodg.kid(0, "another").is_none());
    /// ```
    pub fn kid(&self, v: u32, a: &str) -> Option<u32> {
        self.vertices
            .get(&v)
            .context(format!("Can't find ν{v}"))
            .unwrap()
            .edges
            .iter()
            .find(|e| e.a == a)
            .map(|e| e.to)
    }

    /// Find a vertex in the Sodg by its locator.
    ///
    /// ```
    /// use sodg::Sodg;
    /// let mut sodg = Sodg::empty();
    /// sodg.add(0).unwrap();
    /// sodg.add(1).unwrap();
    /// sodg.bind(0, 1, "a").unwrap();
    /// sodg.add(2).unwrap();
    /// sodg.bind(1, 2, "b").unwrap();
    /// assert_eq!(2, sodg.find(0, "a.b").unwrap());
    /// ```
    pub fn find(&self, v1: u32, loc: &str) -> Result<u32> {
        let mut v = v1;
        let mut locator: VecDeque<String> = VecDeque::new();
        loc.split('.')
            .filter(|k| !k.is_empty())
            .for_each(|k| locator.push_back(k.to_string()));
        loop {
            let next = locator.pop_front();
            if next.is_none() {
                trace!("#find: end of locator, we are at ν{v}");
                break;
            }
            let k = next.unwrap().to_string();
            if k.is_empty() {
                return Err(anyhow!("System error, the locator is empty"));
            }
            if k.starts_with('ν') {
                let num: String = k.chars().skip(1).collect::<Vec<_>>().into_iter().collect();
                v = u32::from_str(num.as_str())?;
                trace!("#find: jumping directly to ν{v}");
                continue;
            }
            if let Some(to) = self.kid(v, k.as_str()) {
                trace!("#find: ν{v}.{k} -> ν{to}");
                v = to;
                continue;
            };
            let others: Vec<String> = self
                .vertices
                .get(&v)
                .unwrap()
                .edges
                .iter()
                .map(|e| e.a.clone())
                .collect();
            return Err(anyhow!(
                "Can't find .{} in ν{} among other {} attribute{}: {}",
                k,
                v,
                others.len(),
                if others.len() == 1 { "" } else { "s" },
                others.join(", ")
            ));
        }
        trace!("#find: found ν{v1} by '{loc}'");
        Ok(v)
    }

    /// Check all alerts.
    fn validate(&self, vx: Vec<u32>) -> Result<()> {
        if self.alerts_active {
            for a in self.alerts.iter() {
                let msgs = a(self, vx.clone());
                if !msgs.is_empty() {
                    return Err(anyhow!("{}", msgs.join("; ")));
                }
            }
        }
        Ok(())
    }
}

#[test]
fn adds_simple_vertex() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    assert_eq!(1, g.find(1, "")?);
    Ok(())
}

#[test]
fn binds_simple_vertices() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    let k = "hello";
    g.bind(1, 2, k)?;
    assert_eq!(2, g.find(1, k)?);
    Ok(())
}

#[test]
fn pre_defined_ids() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    let k = "a-привет";
    g.bind(1, 2, k)?;
    assert_eq!(2, g.find(1, k)?);
    Ok(())
}

#[test]
fn binds_two_names() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    g.bind(1, 2, "first")?;
    g.bind(1, 2, "second")?;
    assert_eq!(2, g.find(1, "first")?);
    Ok(())
}

#[test]
fn overwrites_edge() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    let label = "hello";
    g.bind(1, 2, label)?;
    g.add(3)?;
    g.bind(1, 3, label)?;
    assert_eq!(3, g.find(1, label)?);
    Ok(())
}

#[test]
fn binds_to_root() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "x")?;
    assert!(g.kid(0, "ρ").is_none());
    assert!(g.kid(0, "σ").is_none());
    Ok(())
}

#[test]
fn sets_simple_data() -> Result<()> {
    let mut g = Sodg::empty();
    let data = "hello".as_bytes().to_vec();
    g.add(0)?;
    g.put(0, data.clone())?;
    assert_eq!(data, g.data(0)?);
    Ok(())
}

#[test]
fn finds_root() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    assert_eq!(0, g.find(0, "")?);
    Ok(())
}

#[test]
fn finds_all_kids() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "one")?;
    g.bind(0, 1, "two")?;
    assert_eq!(2, g.kids(0)?.iter().count());
    Ok(())
}

#[test]
fn panic_on_simple_alert() -> Result<()> {
    let mut g = Sodg::empty();
    g.alert_on(|_, _| vec![format!("{}", "oops")]);
    assert!(g.add(0).is_err());
    Ok(())
}

#[test]
fn dont_panic_when_alerts_disabled() -> Result<()> {
    let mut g = Sodg::empty();
    g.alert_on(|_, _| vec!["should never happen".to_string()]);
    g.alerts_off();
    assert!(!g.add(0).is_err());
    Ok(())
}

#[test]
fn panic_on_complex_alert() -> Result<()> {
    let mut g = Sodg::empty();
    g.alert_on(|_, vx| {
        let v = 42;
        if vx.contains(&v) {
            vec![format!("Vertex no.{v} is not allowed")]
        } else {
            vec![]
        }
    });
    assert!(g.add(42).is_err());
    Ok(())
}

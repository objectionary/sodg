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

use crate::Sodg;
use crate::{DeadRelay, LambdaRelay, Relay};
use anyhow::{anyhow, Context, Result};
use log::trace;
use std::collections::VecDeque;
use std::str::FromStr;

impl Relay for DeadRelay {
    fn re(&self, v: u32, a: &str) -> Result<String> {
        Err(anyhow!("Can't find ν{v}.{a}"))
    }
}

impl DeadRelay {
    /// Make a new [`DeadRelay`], the empty one.
    pub fn new() -> Self {
        DeadRelay {}
    }
}

impl Default for DeadRelay {
    /// Make a new default [`DeadRelay`].
    #[allow(dead_code)]
    fn default() -> Self {
        Self::new()
    }
}

impl LambdaRelay {
    /// Make a new instance of [`LambdaRelay`] with the encapsulated
    /// lambda function.
    ///
    /// The function must accept three arguments:
    /// 1) the ID of the vertex where the search algorithm found a problem,
    /// 2) the name of the edge it is trying to find.
    /// The function must return a new locator,
    /// which the algorithm will use. If it is just
    /// a string, it will be treated as a name of the attribute to
    /// try instead. If it starts from `"ν"`, it is treated as an absolute
    /// locator on the entire graph.
    #[allow(dead_code)]
    pub fn new(lambda: fn(u32, &str) -> Result<String>) -> Self {
        LambdaRelay { lambda }
    }
}

impl Relay for LambdaRelay {
    fn re(&self, v: u32, a: &str) -> Result<String> {
        (self.lambda)(v, a)
    }
}

impl Sodg {
    /// Find a vertex in the Sodg by its locator using a [`Relay`]
    /// to provide alternative edge names, if the desired ones are not found.
    ///
    /// For example, here is how [`LambdaRelay`] may be used with a
    /// "relaying" function:
    ///
    /// ```
    /// use sodg::Sodg;
    /// use sodg::DeadRelay;
    /// use sodg::LambdaRelay;
    /// let mut g = Sodg::empty();
    /// g.add(0).unwrap();
    /// g.add(1).unwrap();
    /// g.bind(0, 1, "foo").unwrap();
    /// assert!(g.find(0, "bar", &DeadRelay::default()).is_err());
    /// let v = g.find(0, "bar", &LambdaRelay::new(|v, a| {
    ///   assert_eq!(a, "bar");
    ///   Ok("foo".to_string())
    /// })).unwrap();
    /// assert_eq!(1, v);
    /// ```
    ///
    /// If `v1` is absent, an `Err` will be returned.
    ///
    /// If searching algorithm fails to find the destination,
    /// an `Err` will be returned.
    pub fn find<T: Relay>(&self, v1: u32, loc: &str, relay: &T) -> Result<u32> {
        self.find_with_indent(v1, loc, relay, "")
    }

    /// Find a vertex, printing the log with an indentation prefix.
    ///
    /// This function is used only by [`Sodg::find].
    fn find_with_indent<T: Relay>(
        &self,
        v1: u32,
        loc: &str,
        relay: &T,
        indent: &str,
    ) -> Result<u32> {
        let mut v = v1;
        let mut locator: VecDeque<String> = VecDeque::new();
        loc.split('.')
            .filter(|k| !k.is_empty())
            .for_each(|k| locator.push_back(k.to_string()));
        loop {
            let next = locator.pop_front();
            if next.is_none() {
                break;
            }
            let k = next.unwrap().to_string();
            if k.starts_with('ν') {
                let num: String = k.chars().skip(1).collect::<Vec<_>>().into_iter().collect();
                v = u32::from_str(num.as_str())?;
                continue;
            }
            if let Some(to) = self.kid(v, k.as_str()) {
                v = to;
                continue;
            };
            trace!("#find(ν{v1}, {loc}): {indent}calling relay(ν{v}, {k})...");
            let redirect = relay.re(v, &k);
            let fault = if let Ok(re) = redirect {
                let mut ind = indent.to_owned();
                ind.push_str("▷ ");
                if let Ok(to) = self.find_with_indent(v, re.as_str(), relay, ind.as_str()) {
                    trace!("#find(ν{v1}, {loc}): {indent}ν{v}.{k} relayed to ν{to} (re: {re})");
                    v = to;
                    continue;
                }
                format!("re to '{re}' didn't help")
            } else {
                redirect.err().unwrap().to_string()
            };
            let others: Vec<String> = self
                .vertices
                .get(&v)
                .context(format!("Can't find ν{v}"))
                .unwrap()
                .edges
                .iter()
                .map(|e| e.a.clone())
                .collect();
            return Err(anyhow!(
                "Can't find ν{v}.{k} among [{}]: ({fault})",
                others.join(", ")
            ));
        }
        trace!("#find(ν{v1}, {loc}): {indent}found ν{v}");
        Ok(v)
    }
}

#[test]
fn finds_with_closure() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    g.add(3)?;
    g.bind(1, 2, "first")?;
    g.bind(2, 3, "something_else")?;
    assert_eq!(
        3,
        g.find(
            1,
            "first.second",
            &mut LambdaRelay::new(|v, a| {
                if v == 1 && !a.is_empty() {
                    panic!();
                }
                if v == 2 && a == "second" {
                    Ok("something_else".to_string())
                } else {
                    Ok("".to_string())
                }
            })
        )?
    );
    Ok(())
}

#[test]
fn finds_root() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    assert_eq!(0, g.find(0, "", &mut DeadRelay::default())?);
    Ok(())
}

#[test]
fn closure_return_absolute_vertex() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0).unwrap();
    g.add(1).unwrap();
    g.bind(0, 1, "foo").unwrap();
    assert!(g.find(0, "bar", &mut DeadRelay::new()).is_err());
    assert_eq!(
        1,
        g.find(
            0,
            "bar",
            &mut LambdaRelay::new(|_v, a| {
                assert_eq!(a, "bar");
                Ok("ν1".to_string())
            }),
        )?
    );
    Ok(())
}

#[cfg(test)]
struct FakeRelay {
    g: Sodg,
}

#[cfg(test)]
impl FakeRelay {
    pub fn new(g: Sodg) -> FakeRelay {
        FakeRelay { g }
    }
    pub fn find(&mut self, k: &str) -> Result<u32> {
        self.g.find(0, k, self)
    }
}

#[cfg(test)]
impl Relay for FakeRelay {
    fn re(&self, _v: u32, _a: &str) -> Result<String> {
        let cp = self as *const Self;
        let mp = cp as *mut Self;
        unsafe {
            (&mut *mp).g.add(42).unwrap();
        }
        Ok("ν42".to_string())
    }
}

#[test]
fn relay_modifies_sodg_back() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0).unwrap();
    g.add(1).unwrap();
    g.bind(0, 1, "foo").unwrap();
    let mut relay = FakeRelay::new(g);
    assert_eq!(42, relay.find("bar")?);
    Ok(())
}

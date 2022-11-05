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

use crate::Hex;
use crate::{Script, Sodg};
use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use log::trace;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

impl Script {
    /// Make a new one, parsing a string with instructions. Instructions
    /// must be separated by semicolon. There are just three of them
    /// possible: `ADD`, `BIND`, and `PUT`. The arguments must be
    /// separated by a comma. An argument may either be 1) a positive integer,
    /// 2) a variable started with `$`, 3) an attribute name, or
    /// 4) data in `XX-XX-...` hexadecimal format.
    ///
    /// ```
    /// use sodg::Script;
    /// use sodg::Sodg;
    /// let mut s = Script::from_str(
    ///   "ADD(0); ADD($ν1); BIND(0, $ν1, foo);"
    /// );
    /// let mut g = Sodg::empty();
    /// let total = s.deploy_to(&mut g).unwrap();
    /// assert_eq!(1, g.kid(0, "foo").unwrap());
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Script {
        Script {
            txt: s.to_string(),
            vars: HashMap::new(),
            root: 0,
        }
    }

    /// Make a new one, parsing a `String` with instructions.
    pub fn from_string(s: String) -> Script {
        Script::from_str(s.as_str())
    }

    /// Set a different root. By default, the root is set to zero.
    /// If the root is changed, each time zero-vertex is mentioned
    /// in instructions, if will be replaced by this number.
    ///
    /// ```
    /// use sodg::Script;
    /// use sodg::Sodg;
    /// let mut s = Script::from_str(
    ///   "ADD(1); BIND(0, 1, foo);"
    /// );
    /// s.set_root(42);
    /// let mut g = Sodg::empty();
    /// g.add(42).unwrap();
    /// s.deploy_to(&mut g).unwrap();
    /// ```
    pub fn set_root(&mut self, v: u32) {
        self.root = v;
    }

    /// Deploy the entire script to the SODG.
    pub fn deploy_to(&mut self, g: &mut Sodg) -> Result<usize> {
        let mut pos = 0;
        for cmd in self.commands().iter() {
            trace!("#deploy_to: deploying command no.{} '{}'...", pos + 1, cmd);
            self.deploy_one(cmd, g)
                .context(format!("Failure at the command no.{pos}: '{cmd}'"))?;
            pos += 1;
        }
        Ok(pos)
    }

    /// Get all commands
    fn commands(&self) -> Vec<String> {
        lazy_static! {
            static ref STRIP_COMMENTS: Regex = Regex::new("#.*\n").unwrap();
        }
        let text = self.txt.as_str();
        let clean: &str = &STRIP_COMMENTS.replace_all(text, "");
        clean
            .split(';')
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .map(|t| t.to_string())
            .collect()
    }

    /// Deploy a single command to the sodg.
    fn deploy_one(&mut self, cmd: &str, g: &mut Sodg) -> Result<()> {
        lazy_static! {
            static ref LINE: Regex = Regex::new("^([A-Z]+) *\\(([^)]*)\\)$").unwrap();
        }
        let cap = LINE.captures(cmd).context(format!("Can't parse '{cmd}'"))?;
        let args: Vec<String> = cap[2]
            .split(',')
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .map(|t| t.to_string())
            .collect();
        match &cap[1] {
            "ADD" => {
                let v = self.parse(&args[0], g)?;
                g.add(v).context(format!("Failed to ADD({})", &args[0]))
            }
            "BIND" => {
                let v1 = self.parse(&args[0], g)?;
                let v2 = self.parse(&args[1], g)?;
                let a = &args[2];
                g.bind(v1, v2, a).context(format!(
                    "Failed to BIND({}, {}, {})",
                    &args[0], &args[1], &args[2]
                ))
            }
            "PUT" => {
                let v = self.parse(&args[0], g)?;
                g.put(v, Self::parse_data(&args[1])?)
                    .context(format!("Failed to DATA({})", &args[0]))
            }
            _cmd => Err(anyhow!("Unknown command: {_cmd}")),
        }
    }

    /// Parse data
    fn parse_data(s: &str) -> Result<Hex> {
        lazy_static! {
            static ref DATA_STRIP: Regex = Regex::new("[ \t\n\r\\-]").unwrap();
            static ref DATA: Regex = Regex::new("^[0-9A-Fa-f]{2}([0-9A-Fa-f]{2})*$").unwrap();
        }
        let d: &str = &DATA_STRIP.replace_all(s, "");
        if DATA.is_match(d) {
            let bytes: Vec<u8> = (0..d.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&d[i..i + 2], 16).unwrap())
                .collect();
            Ok(Hex::from_vec(bytes))
        } else {
            Err(anyhow!("Can't parse data '{s}'"))
        }
    }

    /// Parses `$ν5` into `5`.
    fn parse(&mut self, s: &str, g: &mut Sodg) -> Result<u32> {
        let head = s.chars().next().context("Empty identifier".to_string())?;
        if head == '$' {
            let tail: String = s.chars().skip(1).collect::<Vec<_>>().into_iter().collect();
            Ok(*self.vars.entry(tail).or_insert_with(|| g.next_id()))
        } else {
            let mut v = u32::from_str(s).context(format!("Parsing of '{s}' failed"))?;
            if v == 0 {
                v = self.root;
            }
            Ok(v)
        }
    }
}

#[cfg(test)]
use std::str;

#[test]
fn simple_command() -> Result<()> {
    let mut g = Sodg::empty();
    let mut s = Script::from_str(
        "
        ADD(0);  ADD($ν1); # adding two vertices
        BIND(0, $ν1, foo  );
        PUT($ν1  , d0-bf-D1-80-d0-B8-d0-b2-d0-b5-d1-82);
        ",
    );
    let total = s.deploy_to(&mut g)?;
    assert_eq!(4, total);
    assert_eq!("привет", g.data(1)?.to_utf8()?);
    assert_eq!(1, g.kid(0, "foo").unwrap());
    Ok(())
}

#[test]
fn deploy_to_another_root() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(42)?;
    let mut s = Script::from_str(
        "
        ADD($ν1);
        BIND(0, $ν1, foo);
        ",
    );
    s.set_root(42);
    s.deploy_to(&mut g)?;
    assert_eq!(43, g.kid(42, "foo").unwrap());
    Ok(())
}

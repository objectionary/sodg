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
use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use log::trace;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

struct Script {
    txt: String,
    vars: HashMap<String, u32>,
}

impl Script {
    /// Make a new one.
    pub fn new(s: &str) -> Script {
        Script {
            txt: s.to_string(),
            vars: HashMap::new(),
        }
    }

    /// Deploy the entire script to the Sodg.
    pub fn deploy_to(&mut self, sodg: &mut Sodg) -> Result<usize> {
        let mut pos = 0;
        for cmd in self.commands().iter() {
            trace!("#deploy_to: deploying command no.{} '{}'...", pos + 1, cmd);
            self.deploy_one(cmd, sodg)
                .context(format!("Failure at the command no.{}: '{}'", pos, cmd))?;
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
    fn deploy_one(&mut self, cmd: &str, sodg: &mut Sodg) -> Result<()> {
        lazy_static! {
            static ref LINE: Regex = Regex::new("^([A-Z]+) *\\(([^)]*)\\)$").unwrap();
        }
        let cap = LINE
            .captures(cmd)
            .context(format!("Can't parse '{}'", cmd))?;
        let args: Vec<String> = cap[2]
            .split(',')
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .map(|t| t.to_string())
            .collect();
        match &cap[1] {
            "ADD" => {
                let v = self.parse(&args[0], sodg)?;
                sodg.add(v).context(format!("Failed to ADD({})", &args[0]))
            }
            "BIND" => {
                let v1 = self.parse(&args[0], sodg)?;
                let v2 = self.parse(&args[1], sodg)?;
                let a = &args[2];
                sodg.bind(v1, v2, a).context(format!(
                    "Failed to BIND({}, {}, {})",
                    &args[0], &args[1], &args[2]
                ))
            }
            "PUT" => {
                let v = self.parse(&args[0], sodg)?;
                sodg.put(v, Self::parse_data(&args[1])?)
                    .context(format!("Failed to DATA({})", &args[0]))
            }
            _cmd => Err(anyhow!("Unknown command: {}", _cmd)),
        }
    }

    /// Parse data
    fn parse_data(s: &str) -> Result<Vec<u8>> {
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
            Ok(bytes)
        } else {
            Err(anyhow!("Can't parse data '{}'", s))
        }
    }

    /// Parses `$ν5` into `5`.
    fn parse(&mut self, s: &str, sodg: &mut Sodg) -> Result<u32> {
        let head = s.chars().next().context("Empty identifier".to_string())?;
        if head == '$' {
            let tail: String = s.chars().skip(1).collect::<Vec<_>>().into_iter().collect();
            Ok(*self.vars.entry(tail).or_insert_with(|| sodg.max() + 1))
        } else {
            Ok(u32::from_str(s).context(format!("Parsing of '{}' failed", s))?)
        }
    }
}

impl Sodg {
    /// Parse string with instructions.
    pub fn from_string(txt: &str) -> Result<Sodg> {
        let mut sodg = Sodg::empty();
        let mut script = Script::new(txt);
        script.deploy_to(&mut sodg)?;
        Ok(sodg)
    }
}

#[cfg(test)]
use std::str;

#[test]
fn simple_command() -> Result<()> {
    let g = Sodg::from_string(
        "
        ADD(0);  ADD($ν1); # adding two vertices
        BIND(0, $ν1, foo  );
        PUT($ν1  , d0-bf-D1-80-d0-B8-d0-b2-d0-b5-d1-82);
        ",
    )?;
    assert_eq!("привет", str::from_utf8(g.data(1)?.as_slice())?);
    assert_eq!(1, g.kid(0, "foo").unwrap());
    Ok(())
}

// SPDX-FileCopyrightText: Copyright (c) 2022-2025 Objectionary.com
// SPDX-License-Identifier: MIT

use crate::{Hex, Script};
use crate::{Label, Sodg};
use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use log::trace;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

impl Script {
    /// Make a new one, parsing a string with instructions.
    ///
    /// Instructions
    /// must be separated by semicolon. There are just three of them
    /// possible: `ADD`, `BIND`, and `PUT`. The arguments must be
    /// separated by a comma. An argument may either be 1) a positive integer
    /// (possibly prepended by `ν`),
    /// 2) a variable started with `$`, 3) an attribute name, or
    /// 4) data in `XX-XX-...` hexadecimal format.
    ///
    /// For example:
    ///
    /// ```
    /// use std::str::FromStr;
    /// use sodg::{Label, Script};
    /// use sodg::Sodg;
    /// let mut s = Script::from_str(
    ///   "ADD(0); ADD($ν1); BIND(ν0, $ν1, foo);"
    /// );
    /// let mut g : Sodg<16> = Sodg::empty(256);
    /// let total = s.deploy_to(&mut g).unwrap();
    /// assert_eq!(1, g.kid(0, Label::from_str("foo").unwrap()).unwrap());
    /// ```
    #[allow(clippy::should_implement_trait)]
    #[must_use]
    pub fn from_str(s: &str) -> Self {
        Self {
            txt: s.to_string(),
            vars: HashMap::new(),
        }
    }

    /// Deploy the entire script to the [`Sodg`].
    ///
    /// # Errors
    ///
    /// If impossible to deploy, an error will be returned.
    pub fn deploy_to<const N: usize>(&mut self, g: &mut Sodg<N>) -> Result<usize> {
        let mut pos = 0;
        for cmd in &self.commands() {
            trace!("#deploy_to: deploying command no.{} '{}'...", pos + 1, cmd);
            self.deploy_one(cmd, g)
                .with_context(|| format!("Failure at the command no.{pos}: '{cmd}'"))?;
            pos += 1;
        }
        Ok(pos)
    }

    /// Get all commands.
    fn commands(&self) -> Vec<String> {
        lazy_static! {
            static ref STRIP_COMMENTS: Regex = Regex::new("#.*\n").unwrap();
        }
        let text = self.txt.as_str();
        let clean: &str = &STRIP_COMMENTS.replace_all(text, "");
        clean
            .split(';')
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .map(ToString::to_string)
            .collect()
    }

    /// Deploy a single command to the [`Sodg`].
    ///
    /// # Errors
    ///
    /// If impossible to deploy, an error will be returned.
    fn deploy_one<const N: usize>(&mut self, cmd: &str, g: &mut Sodg<N>) -> Result<()> {
        lazy_static! {
            static ref LINE: Regex = Regex::new("^([A-Z]+) *\\(([^)]*)\\)$").unwrap();
        }
        let cap = LINE
            .captures(cmd)
            .with_context(|| format!("Can't parse '{cmd}'"))?;
        let args: Vec<String> = cap[2]
            .split(',')
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .map(ToString::to_string)
            .collect();
        match &cap[1] {
            "ADD" => {
                let v = self.parse(args.first().with_context(|| "V is expected")?, g)?;
                g.add(v);
            }
            "BIND" => {
                let v1 = self.parse(args.first().with_context(|| "V1 is expected")?, g)?;
                let v2 = self.parse(args.get(1).with_context(|| "V2 is expected")?, g)?;
                let a =
                    Label::from_str(args.get(2).with_context(|| "Label is expected")?.as_str())?;
                g.bind(v1, v2, a);
            }
            "PUT" => {
                let v = self.parse(args.first().with_context(|| "V is expected")?, g)?;
                let d = Self::parse_data(args.get(1).with_context(|| "Data is expected")?)?;
                g.put(v, &d);
            }
            cmd => {
                return Err(anyhow!("Unknown command: {cmd}"));
            }
        }
        Ok(())
    }

    /// Parse data.
    ///
    /// # Errors
    ///
    /// If impossible to parse, an error will be returned.
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

    /// Parse `$ν5` into `5`, and `ν23` into `23`, and `42` into `42`.
    ///
    /// # Errors
    ///
    /// If impossible to parse, an error will be returned.
    fn parse<const N: usize>(&mut self, s: &str, g: &mut Sodg<N>) -> Result<usize> {
        let head = s
            .chars()
            .next()
            .with_context(|| "Empty identifier".to_string())?;
        if head == '$' || head == 'ν' {
            let tail: String = s.chars().skip(1).collect::<Vec<_>>().into_iter().collect();
            if head == '$' {
                Ok(*self.vars.entry(tail).or_insert_with(|| g.next_id()))
            } else {
                Ok(usize::from_str(tail.as_str())
                    .with_context(|| format!("Parsing of '{s}' failed"))?)
            }
        } else {
            let v = usize::from_str(s).with_context(|| format!("Parsing of '{s}' failed"))?;
            Ok(v)
        }
    }
}

#[cfg(test)]
use std::str;

#[test]
fn simple_command() {
    let mut g: Sodg<16> = Sodg::empty(256);
    let mut s = Script::from_str(
        "
        ADD(0);  ADD($ν1); # adding two vertices
        BIND(ν0, $ν1, foo  );
        PUT($ν1  , d0-bf-D1-80-d0-B8-d0-b2-d0-b5-d1-82);
        ",
    );
    let total = s.deploy_to(&mut g).unwrap();
    assert_eq!(4, total);
    assert_eq!("привет", g.data(1).unwrap().to_utf8().unwrap());
    assert_eq!(1, g.kid(0, Label::from_str("foo").unwrap()).unwrap());
}

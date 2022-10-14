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

use crate::{Alert, Sodg};

impl Sodg {
    /// Attach a new alert to this SODG.
    pub fn alert_on(&mut self, a: Alert) {
        self.alerts.push(a);
    }

    /// Disable all alerts.
    pub fn alerts_off(&mut self) {
        self.alerts_active = false;
    }

    /// Enable all alerts.
    pub fn alerts_on(&mut self) {
        self.alerts_active = true;
    }
}

#[cfg(test)]
use anyhow::Result;

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

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

use crate::sot::Sot;
use anyhow::Result;
use itertools::Itertools;
use xml_builder::{XMLBuilder, XMLElement, XMLVersion};

impl Sot {
    /// Make XML graph.
    pub fn to_xml(&self) -> Result<String> {
        let mut xml = XMLBuilder::new()
            .version(XMLVersion::XML1_1)
            .encoding("UTF-8".into())
            .build();
        let mut root = XMLElement::new("sot");
        for (v, vtx) in self.vertices.iter().sorted_by_key(|(v, _)| v.clone()) {
            let mut v_node = XMLElement::new("v");
            v_node.add_attribute("id", v.to_string().as_str());
            for e in vtx.edges.iter().sorted_by_key(|e| e.a.clone()) {
                let mut e_node = XMLElement::new("e");
                e_node.add_attribute("a", e.a.as_str());
                e_node.add_attribute("to", e.to.to_string().as_str());
                v_node.add_child(e_node).unwrap();
            }
            if !vtx.data.is_empty() {
                let mut data_node = XMLElement::new("data");
                data_node
                    .add_text(
                        vtx.data
                            .iter()
                            .map(|b| format!("{:02X}", b).to_string())
                            .collect::<Vec<String>>()
                            .join(" "),
                    )
                    .unwrap();
                v_node.add_child(data_node).unwrap();
            }
            root.add_child(v_node).unwrap();
        }
        xml.set_root_element(root);
        let mut writer: Vec<u8> = Vec::new();
        xml.generate(&mut writer).unwrap();
        Ok(std::str::from_utf8(&writer)?.to_string())
    }
}

#[cfg(test)]
use sxd_xpath::evaluate_xpath;

#[test]
fn prints_simple_graph() -> Result<()> {
    let mut sot = Sot::empty();
    sot.add(0)?;
    sot.put(0, "hello".as_bytes().to_vec())?;
    sot.add(1)?;
    sot.bind(0, 1, "foo")?;
    let xml = sot.to_xml()?;
    println!("{}", xml);
    let parser = sxd_document::parser::parse(xml.as_str())?;
    let doc = parser.as_document();
    assert_eq!(
        "foo",
        evaluate_xpath(&doc, "/sot/v[@id=0]/e[1]/@a")?.string()
    );
    assert_eq!(
        "68 65 6C 6C 6F",
        evaluate_xpath(&doc, "/sot/v[@id=0]/data")?.string()
    );
    Ok(())
}

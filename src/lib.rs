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

// @todo #1:30min For some weird reason, if I enable this check,
//  the build fails due to many warnings in the code about
//  "unused" methods. Let's find the reason and uncomment this line.
// #![deny(warnings)]

mod sot;

#[cfg(test)]
use simple_logger::SimpleLogger;

#[cfg(test)]
use log::LevelFilter;

#[cfg(test)]
#[ctor::ctor]
fn init() {
    SimpleLogger::new()
        .without_timestamps()
        .with_level(LevelFilter::Trace)
        .init()
        .unwrap();
}

// Copyright 2018, Scott J Maddox
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[allow(unused_macros)]
macro_rules! println {
    ($host:expr, $($arg:tt)*) => {{
        use arrayvec::ArrayString;
        use core::fmt::Write;
        let mut buf = ArrayString::<[_; 8192]>::new();
        write!(&mut buf, $($arg)*).unwrap();
        ($host.println)(buf.as_str());
    }}
}

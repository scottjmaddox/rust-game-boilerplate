// Copyright 2018, Scott J Maddox
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub struct HostApi {
    pub println: fn(&str),
    pub viewport_size: fn() -> (u32, u32),
    pub poll_events: fn() -> Option<Event>,
    pub push_vertex: fn(pos: [f32; 4], color: [f32; 4]),
    pub set_transform: fn(transform: [[f32; 4]; 4]),
}

#[derive(Debug, Clone)]
pub enum Event {
    ClearInputs,
    KeyPressed { key: Key },
    KeyReleased { key: Key },
}

#[derive(Debug, Clone, Copy)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
}

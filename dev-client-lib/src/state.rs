// Copyright 2018, Scott J Maddox
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use host_api::{Event, HostApi, Key};

fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

pub struct Vec2 {
    x: f32,
    y: f32,
}

/// A camera for 2D scenes (orthographic projection along z-axis).
pub struct Camera2D {
    view_center: Vec2,
    view_extents: Vec2,
}

impl Camera2D {
    pub fn new(view_center: Vec2, view_extents: Vec2) -> Self {
        assert!(view_extents.x > 0.);
        assert!(view_extents.y > 0.);
        Self {
            view_center,
            view_extents,
        }
    }

    #[inline]
    pub fn top(&self) -> f32 {
        self.view_center.y + self.view_extents.y
    }

    #[inline]
    pub fn bottom(&self) -> f32 {
        self.view_center.y - self.view_extents.y
    }

    #[inline]
    pub fn left(&self) -> f32 {
        self.view_center.x - self.view_extents.x
    }

    #[inline]
    pub fn right(&self) -> f32 {
        self.view_center.x + self.view_extents.x
    }

    pub fn transform(&self) -> [[f32; 4]; 4] {
        let l = self.left();
        let r = self.right();
        let t = self.top();
        let b = self.bottom();
        [
            [2. / (r - l), 0., 0., -(r + l) / (r - l)],
            [0., 2. / (t - b), 0., -(t + b) / (t - b)],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]
    }
}

pub struct State {
    host: &'static mut HostApi,
    camera: Camera2D,
    up_pressed: bool,
    down_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
    x: f32,
    y: f32,
}

const VELOCITY: f32 = 10.;
impl State {
    #[inline]
    pub fn new(host: &mut HostApi) -> Self {
        let host = unsafe { &mut *(host as *mut HostApi) };
        let camera = Camera2D::new(vec2(0., 0.), vec2(1., 1.));
        Self {
            host,
            camera,
            up_pressed: false,
            down_pressed: false,
            left_pressed: false,
            right_pressed: false,
            x: 0.,
            y: 0.,
        }
    }

    #[inline]
    pub fn set_host(&mut self, host: &mut HostApi) {
        self.host = unsafe { &mut *(host as *mut HostApi) };
    }

    pub fn update_and_render(&mut self) {
        let dt: f32 = 1. / 60.;
        while let Some(event) = (self.host.poll_events)() {
            self.dispatch_event(event);
        }
        if self.up_pressed {
            self.y += VELOCITY * dt;
        }
        if self.down_pressed {
            self.y -= VELOCITY * dt;
        }
        if self.left_pressed {
            self.x -= VELOCITY * dt;
        }
        if self.right_pressed {
            self.x += VELOCITY * dt;
        }
        // println!(self.host, "x = {:.2}", self.x);

        let (w, h) = (self.host.viewport_size)();
        self.camera.view_extents = vec2(15., 15. / w as f32 * h as f32);

        self.render()
    }

    fn dispatch_event(&mut self, event: Event) {
        match event {
            Event::ClearInputs => {
                self.up_pressed = false;
                self.down_pressed = false;
                self.left_pressed = false;
                self.right_pressed = false;
            }
            Event::KeyPressed { key: Key::Up } => self.up_pressed = true,
            Event::KeyReleased { key: Key::Up } => self.up_pressed = false,
            Event::KeyPressed { key: Key::Down } => self.down_pressed = true,
            Event::KeyReleased { key: Key::Down } => self.down_pressed = false,
            Event::KeyPressed { key: Key::Left } => self.left_pressed = true,
            Event::KeyReleased { key: Key::Left } => self.left_pressed = false,
            Event::KeyPressed { key: Key::Right } => self.right_pressed = true,
            Event::KeyReleased { key: Key::Right } => self.right_pressed = false,
        }
    }

    fn render(&mut self) {
        let x = self.x;
        let y = self.y;
        let player_width = 1.0;
        let player_height = 1.0;
        self.push_solid_quad(
            [
                [x, y, 0.0, 1.0],
                [x, y + player_height, 0.0, 1.0],
                [x + player_width, y + player_height, 0.0, 1.0],
                [x + player_width, y, 0.0, 1.0],
            ],
            [1., 0., 0., 1.0],
        );

        (self.host.set_transform)(self.camera.transform());
    }

    fn push_solid_tri(&mut self, vertices: [[f32; 4]; 3], color: [f32; 4]) {
        (self.host.push_vertex)(vertices[0], color);
        (self.host.push_vertex)(vertices[1], color);
        (self.host.push_vertex)(vertices[2], color);
    }

    fn push_solid_quad(&mut self, vertices: [[f32; 4]; 4], color: [f32; 4]) {
        self.push_solid_tri([vertices[0], vertices[1], vertices[2]], color);
        self.push_solid_tri([vertices[2], vertices[3], vertices[0]], color);
    }
}

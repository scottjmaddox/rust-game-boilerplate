// Copyright 2018, Scott J Maddox
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate live_reload;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate gfx;
extern crate gfx_core;
extern crate gfx_window_sdl;
extern crate sdl2;
extern crate sdl2_sys;

#[path = "../../dev-client-lib/src/host_api.rs"]
mod host_api;
mod loop_context;

use host_api::HostApi;
use loop_context::LoopContext;

use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use live_reload::{Reloadable, ShouldQuit};

use gfx::traits::FactoryExt;
use gfx::Device;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
        color: [f32; 4] = "a_Color",
    }

    constant Uniforms {
        transform: [[f32; 4];4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        uniforms: gfx::ConstantBuffer<Uniforms> = "Uniforms",
        out: gfx::BlendTarget<ColorFormat> =
            ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }
}

lazy_static! {
    static ref EVENTS: Mutex<VecDeque<host_api::Event>> = Mutex::new(VecDeque::with_capacity(1024));
    static ref VIEWPORT_WIDTH: AtomicUsize = AtomicUsize::new(0);
    static ref VIEWPORT_HEIGHT: AtomicUsize = AtomicUsize::new(0);
    static ref VERTEX: Mutex<Vec<Vertex>> = Mutex::new(Vec::with_capacity(1024));
    static ref UNIFORMS: Mutex<Uniforms> = Mutex::new(Uniforms {
        transform: [[1., 0., 0., 0.],
                    [0., 1., 0., 0.],
                    [0., 0., 1., 0.],
                    [0., 0., 0., 1.]]
    });
}
fn poll_events() -> Option<host_api::Event> {
    EVENTS.lock().unwrap().pop_front()
}
fn store_viewport_size(size: (u32, u32)) {
    VIEWPORT_WIDTH.store(size.0 as usize, Ordering::Release);
    VIEWPORT_HEIGHT.store(size.1 as usize, Ordering::Release);
}
fn viewport_size() -> (u32, u32) {
    (
        VIEWPORT_WIDTH.load(Ordering::Acquire) as u32,
        VIEWPORT_HEIGHT.load(Ordering::Acquire) as u32,
    )
}
fn push_vertex(pos: [f32; 4], color: [f32; 4]) {
    VERTEX.lock().unwrap().push(Vertex { pos, color });
}
fn set_transform(transform: [[f32; 4]; 4]) {
    UNIFORMS.lock().unwrap().transform = transform;
}

pub fn main() {
    let mut reloadable = Reloadable::<HostApi>::new(
        "target/debug/libdev_client_lib.dylib",
        HostApi {
            println,
            poll_events,
            viewport_size,
            push_vertex,
            set_transform,
        },
    ).expect("failed to load dev-client-lib");
    let mut loop_context = LoopContext::new();

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    // Request opengl core 3.2 for example:
    video
        .gl_attr()
        .set_context_profile(sdl2::video::GLProfile::Core);
    video.gl_attr().set_context_version(3, 2);
    let mut window_builder = video.window("rust-game-boilerplate", 960, 540);
    window_builder.position(0, 0);
    let (window, _glcontext, mut device, mut factory, color_view, _depth_view) =
        gfx_window_sdl::init::<ColorFormat, DepthFormat>(&video, window_builder)
            .expect("gfx_window_sdl::init failed!");

    store_viewport_size(window.drawable_size());
    let pso = factory
        .create_pipeline_simple(
            include_bytes!("../../shaders/myshader_150.glslv"),
            include_bytes!("../../shaders/myshader_150.glslf"),
            pipe::new(),
        )
        .unwrap();

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let mut uniforms_buffer = factory.create_constant_buffer(1);

    let mut quit = false;
    'main_loop: loop {
        {
            let events = &mut *EVENTS.lock().unwrap();
            for event in event_pump.poll_iter() {
                dispatch_event(events, &mut loop_context, &mut reloadable, &mut quit, event);
            }
            loop_context.begin_frame(&mut reloadable);
            if loop_context.recording() {
                for event in events.iter() {
                    loop_context.push_event(event.clone());
                }
            } else if loop_context.playing_back() {
                events.clear();
                for event in loop_context.playback_events() {
                    events.push_back(event.clone());
                }
            }
        }
        if quit {
            break 'main_loop;
        }

        if reloadable.update() == ShouldQuit::Yes {
            break 'main_loop;
        }
        reloadable
            .reload()
            .expect("failed to reload dev-client-lib");

        encoder.update_constant_buffer(&uniforms_buffer, &*UNIFORMS.lock().unwrap());
        let (vertex_buffer, slice) =
            factory.create_vertex_buffer_with_slice(&*VERTEX.lock().unwrap(), ());
        let data = pipe::Data {
            vbuf: vertex_buffer,
            uniforms: uniforms_buffer,
            out: color_view.clone(),
        };

        encoder.clear(&color_view, [0., 0., 0., 1.]);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.gl_swap_window();
        device.cleanup();

        uniforms_buffer = data.uniforms;
        VERTEX.lock().unwrap().clear();
    }
}

fn println(msg: &str) {
    println!("{}", msg);
}

use sdl2::event::Event as SdlEvent;
fn dispatch_event(
    events: &mut VecDeque<host_api::Event>, 
    loop_context: &mut LoopContext,
    reloadable: &mut Reloadable<HostApi>,
    quit: &mut bool,
    sdl_event: SdlEvent)
{
    use sdl2::keyboard::Keycode;
    use sdl2_sys::SDL_Keymod::{KMOD_LALT, KMOD_RALT};
    use host_api::Event;
    use host_api::Key;

    let kmod_alt = ::sdl2::keyboard::Mod::from_bits_truncate(KMOD_LALT as u16 | KMOD_RALT as u16);
    match sdl_event {
        SdlEvent::Quit { .. } => {
            *quit = true;
        }

        SdlEvent::KeyDown {
            keycode: Some(Keycode::L),
            repeat: false,
            keymod,
            ..
        } if keymod.intersects(kmod_alt) => {
            // Cycle looping mode between recording,
            // playback, and neither.
            if loop_context.recording() {
                println!("Begin loop playback.");
                loop_context.begin_playback(reloadable);
            } else if loop_context.playing_back() {
                println!("End loop playback.");
                loop_context.end_playback();
                // release all buttons
                events.push_back(Event::ClearInputs);
            } else {
                println!("Begin loop recording.");
                loop_context.begin_recording(reloadable);
            }
        }

        SdlEvent::KeyDown {
            keycode: Some(Keycode::Up),
            repeat: false,
            ..
        } => events.push_back(Event::KeyPressed { key: Key::Up }),
        SdlEvent::KeyUp {
            keycode: Some(Keycode::Up),
            repeat: false,
            ..
        } => events.push_back(Event::KeyReleased { key: Key::Up }),

        SdlEvent::KeyDown {
            keycode: Some(Keycode::Down),
            repeat: false,
            ..
        } => events.push_back(Event::KeyPressed { key: Key::Down }),
        SdlEvent::KeyUp {
            keycode: Some(Keycode::Down),
            repeat: false,
            ..
        } => events.push_back(Event::KeyReleased { key: Key::Down }),

        SdlEvent::KeyDown {
            keycode: Some(Keycode::Left),
            repeat: false,
            ..
        } => events.push_back(Event::KeyPressed { key: Key::Left }),
        SdlEvent::KeyUp {
            keycode: Some(Keycode::Left),
            repeat: false,
            ..
        } => events.push_back(Event::KeyReleased { key: Key::Left }),

        SdlEvent::KeyDown {
            keycode: Some(Keycode::Right),
            repeat: false,
            ..
        } => events.push_back(Event::KeyPressed { key: Key::Right }),
        SdlEvent::KeyUp {
            keycode: Some(Keycode::Right),
            repeat: false,
            ..
        } => events.push_back(Event::KeyReleased { key: Key::Right }),

        _ => {}
    }
}

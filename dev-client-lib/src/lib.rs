// Copyright 2018, Scott J Maddox
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![no_std]

extern crate arrayvec;
#[macro_use]
extern crate live_reload;

#[macro_use]
mod macros;
mod host_api;
mod state;

use state::*;
use live_reload::ShouldQuit;
use host_api::HostApi;

live_reload! {
    host: HostApi;
    state: State;
    init: init;
    reload: on_reload;
    update: update;
    unload: on_unload;
    deinit: on_deinit;
}

fn init(host: &mut HostApi, state: &mut State) {
    (host.println)("Init!");
    *state = State::new(host);
}

fn update(host: &mut HostApi, state: &mut State) -> ShouldQuit {
    state.set_host(host);
    state.update_and_render();
    ShouldQuit::No
}

fn on_reload(host: &mut HostApi, _: &mut State) {
    (host.println)("Reloading.");
}

fn on_unload(host: &mut HostApi, _: &mut State) {
    (host.println)("Unloading.");
}

fn on_deinit(host: &mut HostApi, _: &mut State) {
    (host.println)("Goodbye!");
}

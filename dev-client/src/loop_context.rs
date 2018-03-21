// Copyright 2018, Scott J Maddox
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use live_reload::{Reloadable, SaveState};
use host_api::{HostApi, Event};

/// Used for looped live editing.
/// Note: we're not handling any memory outside of the
/// `State` struct, and we're not specially handling
/// pointers, so there shouldn't be any.
/// Since the lib is no_std, there shouldn't
/// be any heap allocated memory we have to worry about,
/// but if/when we allow heap allocation, we'll
/// need to save the heap as well, or
/// looping won't save/restore that memory.
pub struct LoopContext {
    state: Option<SaveState>,
    events: Vec<Vec<Event>>,
    frame_index: usize,
    recording: bool,
    playing_back: bool,
}

impl LoopContext {
    pub fn new() -> Self {
        Self {
            state: None,
            events: Vec::new(),
            frame_index: 0,
            recording: false,
            playing_back: false,
        }
    }

    pub fn recording(&self) -> bool {
        self.recording
    }

    pub fn playing_back(&self) -> bool {
        self.playing_back
    }

    /// Call this at the beginning of each frame.
    pub fn begin_frame(&mut self, r: &mut Reloadable<HostApi>) {
        if self.recording() {
            self.begin_recording_new_frame();
        }
        if self.playing_back() {
            self.begin_playback_new_frame(r);
        }
    }

    /// Call this to begin a new recording.
    pub fn begin_recording(&mut self, r: &mut Reloadable<HostApi>) {
        self.state = Some(r.save_state());
        self.events.clear();
        self.frame_index = 0;
        self.recording = true;
        self.begin_frame(r);
    }

    fn begin_recording_new_frame(&mut self) {
        assert!(self.recording);
        self.events.push(Vec::new());
    }

    pub fn end_recording(&mut self) {
        self.recording = false;
    }

    /// Panics if begin_recording_new_frame was not called
    /// after initialization.
    pub fn push_event(&mut self, event: Event) {
        assert!(self.recording);
        self.events.last_mut().unwrap().push(event.clone());
    }

    /// Call this to start playback from the beginning.
    /// If currently recording, this ends recording first.
    /// Panics if there's no recording to play from.
    pub fn begin_playback(&mut self, r: &mut Reloadable<HostApi>) {
        if self.recording() {
            self.end_recording();
        }
        self.frame_index = 0;
        self.playing_back = true;
        r.load_state(self.state.as_ref().unwrap());
    }

    pub fn end_playback(&mut self) {
        self.playing_back = false;
    }

    /// Call this at the beginning of each frame you want to playback.
    /// This will loop when the end of the recording is reached.
    fn begin_playback_new_frame(&mut self, r: &mut Reloadable<HostApi>) {
        assert!(self.playing_back);
        self.frame_index += 1;
        if self.frame_index >= self.events.len() {
            self.begin_playback(r);
        }
    }

    pub fn playback_events(&mut self) -> ::std::slice::Iter<Event> {
        assert!(self.playing_back);
        self.events[self.frame_index].iter()
    }
}
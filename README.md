# rust-game-boilerplate

This repository contains boilerplate rust code for getting a game prototype up
and running quickly. It uses [sdl2] for window management and OpenGl context
creation, [gfx (pre-ll)] for rendering, and a slightly modified [live-reload]
for live code reloading and [looped live code editing]. No sound solutions is
currently provided, but you can add your own and pull requests are welcome!

[sdl2]: https://github.com/Rust-SDL2/rust-sdl2
[gfx-rs (pre-ll)]: https://github.com/gfx-rs/gfx/tree/pre-ll
[live-reload]: https://github.com/scottjmaddox/live-reloading-rs
[looped live code editing]: https://hero.handmade.network/episode/code/day023/

## Getting Started

To get started, clone this repository, and use cargo to run the development client, `dev-client`:

```sh
git clone https://github.com/scottjmaddox/rust-game-boilerplate
cd rust-game-boilerplate
cargo run --manifest-path=dev-client/Cargo.toml
```

Use the arrow keys on your keyboard to move the red square around. To start
recording a state/input loop, press `Alt+L`. When you are done recording, press
`Alt+L` again to end recording and start playback. To try-out looped live code
editing, open `dev-client-lib/src/state.rs`, change the `VELOCITY` constant to
`20.` (from `10.`), and in a separate terminal, run `cargo build` to rebuild the
`dev-client-lib`. You should see the player start moving twice as fast. When
you're done making your edits, press `Alt+L` once more to end the playback loop.

## Limitations / Tips

In order for state/input loops to properly save and restore, the `dev-client-lib` must be `no_std`, meaning no dynamic allocation, and the `State` struct cannot contain references or pointers to other members (or sub-members) of the `State` struct. Neither of these limitations are particularly problematic, though, if you follow these tips:

- instead of using `Box`, add (unboxed) members to the `State` struct
- instead of using `Vec`, add sufficiently large arrays or `ArrayVec`s (from the `arrayvec` crate) to the `State` struct
- instead of using `String`, use stack-allocated (i.e. local variable) `ArrayString`s (from the `arrayvec` crate)
- instead of using `String`, add sufficiently large `ArrayString`s to the `State` struct
- instead of storing references or pointers in the `State` struct, store array indices/offsets

## License

The source code in this repository is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this repository by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

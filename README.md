# ovr_overlay
An alternative to the [`openvr`](https://github.com/rust-openvr/rust-openvr)
crate that uses CXX instead of bindgen, and focuses on providing support for
overlays.

## Goals
New applications should probably seek to use `OpenXR` instead of `OpenVR`. However,
there are still some things that you can't do in `OpenXR` that you *can* do in `OpenVR`,
namely, creating overlays.

This binding allows you to create an OpenVR overlay with rust, bue doesnt necessarily
provide bindings to the rest of the OpenVR API. If you find missing functionality
(and you *will* find missing functionality), submit a PR!

## Why not build bindings on the C API?
The C API is straight up broken in the official repository. The latest release
doesn't even compile, and hasn't for [over a year](https://github.com/ValveSoftware/openvr/issues/1642).
Whats more, the C bindings don't have the same safety features as the C++ API,
eschewing the use of `const`, etc. Furthermore, the C API is autogenerated from
the C++ one, and isn't documented like the C++ one is.

## Why not use the existing `openvr` crate?
That crate hasn't been updated in over 2 years. And I don't blame them, time is better
spent getting first class support for OpenXR in Rust, as OpenVR is on its way out.

Unfortunately, that crate is missing all of the overlay functionality, and since I'm
really just focused on overlays right now, I need an alternative.

## License
All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](docs/LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](docs/LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are very good reasons to include both.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

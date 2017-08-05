# Raytron

[![license](https://img.shields.io/badge/license-MIT_or_Apache_2.0-blue.svg)](#license)
[![](https://tokei.rs/b1/github/a1xt/raytron)](#)
[![Build Status](https://travis-ci.org/a1xt/raytron.svg?branch=master)](https://travis-ci.org/a1xt/raytron)

Yet another path tracer.

## Building:
```
git clone https://github.com/a1xt/raytron.git
cd raytron
git submodule init
git submodule update
cargo run --example envmap --release
```

## Implemented:
- Primitives: sphere, triangle
- Direct lighting
- Kd-tree accelerator
- Materials: Lambertian, Phong, Cook-Torrance
- Textures

## Gallery:
WIP


## License
Raytron is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

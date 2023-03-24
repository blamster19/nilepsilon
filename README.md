# nilepsilon

_nilepsilon_ is an attempt at writing physically-based spectral path tracer entirely in Rust programming language. The project is also a language learning experience and a graphics programming toy.

The renderer is currently in an unfinished state and might or might not be developed further in the future. Proceed with caution.

## Features

Currently implemented features include:
* std output of `*.ppm` image format
* parallel rendering via `rayon`
* spectral hero wavelength sampling
* camera with focal length and sensor dimensions
* diffuse BSDF (Oren-Nayar)
* glossy BSDF (GGX)
* blackbody radiation
* refraction [*borked*]
* fresnel dielectric [*borked*]

## Running

_nilepsilon_ is written as a library, and therefore lacks main body. To test the engine, run example tests and direct their output to file: 
```
$ cargo test [test name] --release -- --nocapture >> filename.ppm
```
After waiting for render to finish, delete unnecessary std lines that got caught up in the output file.

![Sample](./sampleimage.jpg "Sample image output of `renderer_cornell_2` test")

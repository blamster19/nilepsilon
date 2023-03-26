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

### Things worth mentioning

* currently the engine represents material colors as reflectance polynomials over the visible spectrum
* the only available primitives are planes, spheres and triangles
* only mesh lights and background lights are available
* the only working shaders are Oren-Nayar diffuse and GGX glossy
* _nilepsilon_ is a summer project and that's why it's not active for the rest of the year

### References and inspiration

[My first path tracer](https://alexanderameye.github.io/notes/path-tracer/)

[Ray Tracing in One Weekend](https://raytracing.github.io/)

[Physically Based Rendering, 3rd ed.](http://www.pbr-book.org/)

[Scratchapixel](https://www.scratchapixel.com/)

[Tzu-Mao Li's course notes](https://cseweb.ucsd.edu/~tzli/cse272/wi2023/)

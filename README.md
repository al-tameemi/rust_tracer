
* [Requirements](#requirements)
* [Install](#install)
* [Latest Render](#latest-render)
* [Current Speed](#current-speed)
* [Notes](#notes)
    * [How and Where Multithreading is used](#how-and-where-multithreading-is-used)
    * [Multithreading Optimization](#multithreading-optimization)

# Rust Tracer
A CPU ray tracer written in rust.

## Requirements
* The [Rust toolchain](https://www.rust-lang.org/tools/install).

## Install
1. ```git clone git@github.com:al-tameemi/rust_tracer.git```
2. ```cd rust_tracer```
3. ```cargo run``` for debug build, ```cargo run --release``` for release build.

## Latest Render
<p align="center">
    <img src="https://github.com/al-tameemi/rust_tracer/blob/main/image.png?raw=true" alt="The latest render according to the current specs."/>
</p>

## Current Speed

| CPU                                                          | Single Threaded | Multi Threaded |
| ------------------------------------------------------------ |:---------------:|:--------------:|
| Ryzen 7 5800x 8-Core, 16-Threads @ 3.8GHz Base, 4.7GHz Boost | 9.5S            | 0.8S           |
| Apple M1 8-Core, 8-Threads @ 3.2GHz                          | 6S              | 1.4S           |

## Notes
### How and Where Multithreading is Used
Multithreading is being used during the iteration over the pixels in the screen. A new thread is requested from a thread pool for each pixel using [Rayon](https://github.com/rayon-rs/rayon)'s ```into_par_iter```.
### Multithreading Optimization
* Convert the pixel sample calculation for Anti-Aliasing into a multithreaded workload (possible?).
* Iterate over the world using threads (not worth it unless the number of objects is large enough, needs testing).
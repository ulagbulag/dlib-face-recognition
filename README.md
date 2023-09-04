# dlib-face-recognition

[![Current Crates.io Version](https://img.shields.io/crates/v/dlib-face-recognition.svg)](https://crates.io/crates/dlib-face-recognition)

Inspired by [a similar python library](https://github.com/ageitgey/face_recognition), 
`dlib-face-recognition` is a Rust library that binds to certain specific features of the [dlib C++ library](https://github.com/davisking/dlib).

This repository will dedicate itself to improve the library's content.

These include:

* An FHOG-based face detector.
* A CNN-based face detector (slower, but more powerful).
* A face landmark predictor for identifying specific landmarks (eyes, nose, etc) from face rectangles.
* A face encoding neural network for generating 128 dimensional face encodings that can be compared via their euclidean distances.

## Original Working

The original work is [here (unmaintaned; since Aug 2021)](https://github.com/expenses/face_recognition).

## Building

### Supported Platforms

* Linux { aarch64, x86_64 }
    - Ubuntu 20.04
* MacOS { aarch64, x86_64 }
    - Apple Silicon (`Apple M1`)
* Windows { x86_64 }
    - Windows 10

For better maintenance, please let us know whether the other platforms support it.
Besides, you may claim us whether the specific platform should support it through `Issues` .

### Dependencies

* cmake
* Blas
* Openblas (optional; overrides blas for better performance when enabling `openblas` feature )
* dlib (optional; can be skipped by enabling `build-native` feature)
* lapack

For Windows, [ `vcpkg` ](https://vcpkg.io/en/getting-started.html) may help building both `Blas` and `lapack` .
For other platforms such as Linux, package managers should support installing them.

### Building Native library

`dlib-face-recognition` requires dlib to be installed. You can either provide a existing system-wide installation, or build it with this library.

* To build it in compile-time:
  - ```sh
    cargo build --features build-native
    ```
* To use a system-wide dependency:
  - ```sh
    cargo build
    ```

The C++ library `dlib` will be installed via `dlib-face-recognition-sys` when the `build-native` feature flag is enabled.

For the build, this library uses `cmake` , so please make sure to have [ `cmake` ](https://cmake.org/install/) .

The `build-native` flag is **disabled by default**, offering increased build times.

### Building Rust package

`dlib-face-recognition` includes a `embed-all` feature flag that can be used with `cargo build --features embed-all` .

`embed-all` will enable the `Default::default` implementations the matching structs. These will search for the /files folder, and if a file doesn't exist it will be downloaded at runtime.

* CNN Face Detector: http://dlib.net/files/shape_predictor_68_face_landmarks.dat.bz2  
* Landmark Predictor: http://dlib.net/files/mmod_human_face_detector.dat.bz2
* Face Recognition Net: http://dlib.net/files/dlib_face_recognition_resnet_model_v1.dat.bz2

It is recommended to acquire the files before compile/runtime and set them in a protected location.
The `embed-all` flag is disabled by default, offering increased build times.

## Testing

There is one included test to recognize, and draw a face's points:

* `cargo run --features embed-all --example draw` -> To run the example.

There is two files to benchmark the code, and test some functions:

* `cargo test --features embed-all --test benchmarks` -> To run the benchmarks.
* `cargo test --features embed-all --test utilities_tests` -> To run the utilities tester.

## Examples

For more information on examples: https://github.com/ulagbulag/dlib-face-recognition/tree/master/examples/README.md

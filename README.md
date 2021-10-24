# dlib-face-recognition

Inspired by [a similar python library](https://github.com/ageitgey/face_recognition),
and the original work [here](https://github.com/podo-os/dlib-face-recognition).

`dlib-face-recognition` is a Rust library that binds to certain specific features of the [dlib C++ library](https://github.com/davisking/dlib).

This repository will dedicate itself to improve the library's content.

These include:

- An FHOG-based face detector.
- A CNN-based face detector (slower, but more powerful).
- A face landmark predictor for identifying specific landmarks (eyes, nose, etc) from face rectangles.
- A face encoding neural network for generating 128 dimensional face encodings that can be compared via their euclidean distances.


## Building

`dlib-face-recognition` requires dlib to be installed.

on (at least) OSX, I _believe_ lapack and openblas also need to be installed.

`cargo build` -> To build

## Using

To use this with any of the examples (or other codes), you will need the following files:

- CNN Face Detector: http://dlib.net/files/shape_predictor_68_face_landmarks.dat.bz2
- Landmark Predictor: http://dlib.net/files/mmod_human_face_detector.dat.bz2
- Face Recognition Net: http://dlib.net/files/dlib_face_recognition_resnet_model_v1.dat.bz2

It is highly recommended storing them in a folder called "files" within the project workspace, if you intend on using the dafult file loaders.
Default file loaders will seek files on files/file.
If desired, it's also possible to load files with absolute paths using the "new" loader method.

## Tests

There is one included test to recognize, and draw a face's points:

`cargo run --example draw` -> To run the example.

There is two files to benchmark the code, and test some functions:

`cargo test --test benchmarks` -> To run the benchmarks.
`cargo test --test utilities_tests` -> To run the utilities tester.


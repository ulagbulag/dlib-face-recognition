# Face Recognition

Inspired by [a similar python library](https://github.com/ageitgey/face_recognition), face_recognition is a Rust library that binds to certain specific features of the [dlib C++ library](https://github.com/davisking/dlib).

These include:

- An FHOG-based face detector.
- A CNN-based face detector (slower, but more powerful).
- A face landmark predictor for identifying specific landmarks (eyes, nose, etc) from face rectangles.
- A face encoding neural network for generating 128 dimensional face encodings that can be compared via their euclidean distances.

## Building

face_recognition requires dlib to be installed.

on (atleast) OSX, I _believe_ lapack and openblas also need to be installed.

face_recognition includes a `download-models` feature flag that can be used with `cargo build --features download-models`.

This will automatically download the face predictor, cnn face detector and face encoding neural network models (the fhog face detector is included in dlib and does not need to be downloaded). Alternatively, these models can be downloaded manually:

- CNN Face Detector: http://dlib.net/files/shape_predictor_68_face_landmarks.dat.bz2  
- Landmark Predictor: http://dlib.net/files/mmod_human_face_detector.dat.bz2
- Face Recognition Net: http://dlib.net/files/dlib_face_recognition_resnet_model_v1.dat.bz2

if this feature flag is enabled, the matching structs will have `Default::default` implementations provided that allows you to load them without having to worry about file locations.
# Examples

This directory contains projects showcasing the project features.

Example on how to run a example:

```sh
cd examples/draw
cargo run
```

## Applications

  * **[`draw`]**

    Simple script that receives a image, finds all existing faces and draw a square with their locations + facial landmarks.

  * **[`compare_faces`]**

    Simple script that compares two faces in terms of euclidean distance. This can be used to determine if any 2 faces are similar enough. Useful in scenarios where it may be needed to identify a unknown face, or similar cases.

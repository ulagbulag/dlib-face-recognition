use image::*;
use dlib_face_recognition::*;
use clap::Parser;

mod args;

use args::Args;

fn draw_rectangle(image: &mut RgbImage, rect: &Rectangle, colour: Rgb<u8>) {
    for x in rect.left..rect.right {
        image.put_pixel(x as u32, rect.top as u32, colour);
        image.put_pixel(x as u32, rect.bottom as u32, colour);
    }

    for y in rect.top..rect.bottom {
        image.put_pixel(rect.left as u32, y as u32, colour);
        image.put_pixel(rect.right as u32, y as u32, colour);
    }
}

fn draw_point(image: &mut RgbImage, point: &Point, colour: Rgb<u8>) {
    image.put_pixel(point.x() as u32, point.y() as u32, colour);
    image.put_pixel(point.x() as u32 + 1, point.y() as u32, colour);
    image.put_pixel(point.x() as u32 + 1, point.y() as u32 + 1, colour);
    image.put_pixel(point.x() as u32, point.y() as u32 + 1, colour);
}

fn tick<R>(name: &str, f: impl Fn() -> R) -> R {
    let now = std::time::Instant::now();
    let result = f();
    println!("[{}] elapsed time: {}ms", name, now.elapsed().as_millis());
    result
}

fn main() {
    let args = Args::parse();

    let mut image = image::open(args.input_image).unwrap().to_rgb8();
    let matrix = ImageMatrix::from_image(&image);

    let detector = FaceDetector::default();

    let Ok(cnn_detector) = FaceDetectorCnn::default() else {
        panic!("Unable to load cnn face detector!");
    };

    let Ok(landmarks) = LandmarkPredictor::default() else {
        panic!("Unable to load landmark predictor!");
    };

    let red = Rgb([255, 0, 0]);
    let green = Rgb([0, 255, 0]);

    let face_locations = tick("FaceDetector", || detector.face_locations(&matrix));

    for r in face_locations.iter() {
        draw_rectangle(&mut image, &r, red);

        let landmarks = landmarks.face_landmarks(&matrix, &r);

        for point in landmarks.iter() {
            draw_point(&mut image, &point, red);
        }
    }

    let face_locations = tick("FaceDetectorCnn", || cnn_detector.face_locations(&matrix));

    for r in face_locations.iter() {
        draw_rectangle(&mut image, &r, green);

        let landmarks = tick("LandmarkPredictor", || {
            landmarks.face_landmarks(&matrix, &r)
        });

        for point in landmarks.iter() {
            draw_point(&mut image, &point, green);
        }
    }

    if let Err(e) = image.save(&args.output_image) {
        println!("Error saving the image: {e}");
    } else {
        println!("Output image saved to {}", &args.output_image);
    }
}
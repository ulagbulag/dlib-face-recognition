use image::*;

use dlib_face_recognition::*;

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

fn main() {
    let input_image = "assets/obama_1.jpg";
    let output_image = "outputs/obama_1.jpg";

    let mut loaded_image = image::open(input_image).unwrap().to_rgb8();
    let image_matrix = ImageMatrix::from_image(&loaded_image);

    let hog_detector = FaceDetector::new();
    let cnn_detector = FaceDetectorCnn::default().unwrap();
    let landmark_predictor = LandmarkPredictor::default().unwrap();

    let color_red = Rgb([255, 0, 0]);
    let color_green = Rgb([0, 255, 0]);

    let start_time = std::time::Instant::now();
    let face_locations = hog_detector.face_locations(&image_matrix);
    let elapsed_time = start_time.elapsed().as_millis();

    println!(
        "[HoG Face Detector] elapsed time: {time}ms",
        time = elapsed_time
    );

    for face in face_locations.iter() {
        draw_rectangle(&mut loaded_image, face, color_red);

        let landmarks = landmark_predictor.face_landmarks(&image_matrix, face);

        for point in landmarks.iter() {
            draw_point(&mut loaded_image, point, color_red);
        }
    }

    let start_time = std::time::Instant::now();
    let face_locations = cnn_detector.face_locations(&image_matrix);
    let elapsed_time = start_time.elapsed().as_millis();

    println!(
        "[Cnn Face Detector] elapsed time: {time}ms",
        time = elapsed_time
    );

    for face in face_locations.iter() {
        draw_rectangle(&mut loaded_image, face, color_green);

        let start_time = std::time::Instant::now();
        let landmarks = landmark_predictor.face_landmarks(&image_matrix, face);
        let elapsed_time = start_time.elapsed().as_millis();

        println!(
            "[Landmark Predictor] elapsed time: {time}ms",
            time = elapsed_time
        );

        for point in landmarks.iter() {
            draw_point(&mut loaded_image, point, color_green);
        }
    }

    loaded_image.save(&output_image).unwrap();
}

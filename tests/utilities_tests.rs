use dlib_face_recognition::FaceEncoding;

#[test]
fn encoding_transformation() {
    let original_array: Vec<f64> = vec![2.5; 128];

    assert_eq!(true, FaceEncoding::from_vec(&original_array).is_ok());
}

#[test]
fn invalid_array_size() {
    let original_array: Vec<f64> = vec![2.5; 135];

    assert_eq!(true, FaceEncoding::from_vec(&original_array).is_err());
}

use opencv::{
    core::*,
	highgui,
    imgcodecs::{imread, IMREAD_GRAYSCALE},
    imgproc::{find_contours, draw_contours, threshold, RETR_EXTERNAL, THRESH_BINARY_INV, CHAIN_APPROX_NONE},
};

fn main() {
    let window_name = "shape detector";
    let orig = imread("a_02_shape_detector/someshapes.jpg", IMREAD_GRAYSCALE).unwrap();

    let mut img = Mat::default();
    threshold(&orig, &mut img, 50.0, 170.0, THRESH_BINARY_INV).unwrap();
    highgui::imshow(window_name, &mut img).unwrap();
    highgui::wait_key(1000).unwrap();
    let mut contours = Vector::<Mat>::new();
    find_contours(&img, &mut contours, RETR_EXTERNAL, CHAIN_APPROX_NONE, Point_::default()).unwrap();
    highgui::named_window(window_name, 0).unwrap();
    for i in 0..contours.len() {
        draw_contours( &mut img, &contours, i.try_into().unwrap(), VecN([255.0, 255.0, 0.0, 133.0]), 10, 0, &no_array(), 1, Point_::default()).unwrap();
    }
    highgui::imshow(window_name, &mut img).unwrap();
    highgui::wait_key(0).unwrap();
}
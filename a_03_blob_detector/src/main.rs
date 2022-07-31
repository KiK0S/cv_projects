use opencv::{
    core::*,
	highgui,
    features2d::{SimpleBlobDetector, SimpleBlobDetector_Params, Feature2DTrait}, 
    imgcodecs::{imread, IMREAD_GRAYSCALE},
    imgproc::{circle, cvt_color, COLOR_GRAY2BGR},
};

fn find_blobs(params: &SimpleBlobDetector_Params, orig: &Mat) -> Vector<KeyPoint> {
    let mut contours = Vector::<KeyPoint>::new();
    let mut detector = SimpleBlobDetector::create(*params).unwrap(); 
    detector.detect(orig, &mut contours, &no_array()).unwrap();
    contours
}

fn main() {
    let window_name = "blob detector";
    let orig = imread("a_03_blob_detector/blobs.jpg", IMREAD_GRAYSCALE).unwrap();
    let mut res = Mat::default();
    cvt_color(&orig, &mut res, COLOR_GRAY2BGR, 0).unwrap();
    let mut circle_params = SimpleBlobDetector_Params::default().unwrap();
    circle_params.min_circularity = 0.9;
    circle_params.filter_by_circularity = true;
    let mut ellipse_params = SimpleBlobDetector_Params::default().unwrap();
    ellipse_params.max_inertia_ratio = 0.4;
    ellipse_params.filter_by_inertia = true;
    let circles = find_blobs(&circle_params, &orig);
    let ellipses = find_blobs(&ellipse_params, &orig);
    highgui::named_window(window_name, 0).unwrap();
    for i in 0..circles.len() {
        circle(&mut res, circles.get(i).unwrap().pt.to().unwrap(), 1, VecN::<f64, 4>::new(0.0, 255.0, 0.0, 255.0), 2, 0, 0).unwrap();
    }
    for i in 0..ellipses.len() {
        circle(&mut res, ellipses.get(i).unwrap().pt.to().unwrap(), 1, VecN::<f64, 4>::new(255.0, 0.0, 255.0, 255.0), 2, 0, 0).unwrap();
    }
    highgui::imshow(window_name, &mut res).unwrap();
    highgui::wait_key(0).unwrap();
}
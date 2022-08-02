use opencv::{
    core::*,
	highgui,
    imgcodecs::{imread, IMREAD_UNCHANGED},
    imgproc::{match_template, rectangle},
};

fn main() {
    let window_name = "find waldo";
    let template = imread("a_04_find_waldo_template_search/waldo.jpg", IMREAD_UNCHANGED).unwrap();
    let mut img = imread("a_04_find_waldo_template_search/WaldoBeach.jpg", IMREAD_UNCHANGED).unwrap();
    let mut res = Mat::default();
    match_template(&img, &template, &mut res, 0, &no_array()).unwrap();
    let mut max_loc = Point2i::new(0, 0);
    min_max_loc(&res, None, None, Some(&mut max_loc), None, &no_array()).unwrap();
    println!("{:?}", max_loc);
    rectangle(&mut img, Rect2i::new(max_loc.x, max_loc.y, template.rows(), template.cols()), VecN::<f64, 4>::new(0.0, 0.0, 0.0, 255.0), 4, 0, 0).unwrap(); 
    highgui::named_window(window_name, 0).unwrap();
    highgui::imshow(window_name, &mut img).unwrap();
    highgui::wait_key(0).unwrap();
}
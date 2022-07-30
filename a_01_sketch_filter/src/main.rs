use opencv::{
    core::*,
	highgui,
    videoio::{prelude::VideoCaptureTrait, VideoCapture, CAP_ANY},
    imgproc::{blur, canny, cvt_color, threshold, COLOR_BGR2GRAY, THRESH_BINARY_INV},
};

fn main() {
    let window_name = "sketcher";
    let mut cap = VideoCapture::new(0, CAP_ANY).unwrap();

	highgui::named_window(window_name, 0).unwrap();
    loop {
		let mut frame = Mat::default();
        cap.read(&mut frame).unwrap();
        let mut blurred = Mat::default();
        blur(&frame, &mut blurred, Size{height:5, width: 5}, Point_ { x: -1, y: -1 }, BORDER_DEFAULT).unwrap();
        let mut grayscale = Mat::default();
        cvt_color(&frame, &mut grayscale, COLOR_BGR2GRAY, 0).unwrap();
        let mut edges = Mat::default();
        canny(&grayscale, &mut edges, 10.0, 70.0, 3, false).unwrap();
        let mut binarized = Mat::default();
        threshold(&edges, &mut binarized, 70.0, 255.0, THRESH_BINARY_INV).unwrap();
        
        highgui::imshow(window_name, &mut binarized).unwrap();
        highgui::wait_key(20).unwrap();
        let key = highgui::wait_key(10).unwrap();
		if key > 0 && key != 255 {
			break;
		}
    }
}
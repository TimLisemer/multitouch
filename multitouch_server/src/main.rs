extern crate opencv;

// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tuio_rs::{Server};

use opencv::{
    core::{self, Size},
    highgui, imgcodecs,
    imgproc::{self, contour_area},
    prelude::VideoCaptureTrait,
    videoio::{self, VideoCaptureTraitConst},
};

#[derive(Debug)]
struct IdToCursor {
    id: u32,
    cursor: i32,
}

#[derive(Debug, Clone)]
struct Finger {
    id: Option<u32>,
    history: Vec<(i32, i32)>,
    normalised_history: Vec<(f32, f32)>,
    active: u32, // 0 newest, 9 oldest
}

impl Finger {
    fn new(id: Option<u32>, history: Vec<(i32, i32)>, normalised_history: Vec<(f32, f32)>, active: u32) -> Self {
        Self {
            id,
            history,
            normalised_history,
            active,
        }
    }
}

// Detects fingers in an image for multitoch applications
fn main() {
    let project_root = project_root::get_project_root().expect("Failed to get project root");

    let image1_path = project_root.join("src").join("1.jpg");
    let video_path = project_root.join("src").join("mt_camera_raw.AVI");

    let background =
        imgcodecs::imread(image1_path.to_str().expect("Invalid image1 path"), 1).unwrap();

    let mut server = tuio();
    let ids_to_cursors: &mut Vec<IdToCursor> = &mut Vec::new();

    video(
        video_path.to_str().expect("Invalid video path"),
        &background,
        ids_to_cursors,
        &mut server,
    );
}

fn tuio () -> Server{
    let server_name = "server_name";
    let mut server = Server::new("server_name").unwrap();
    server.set_source_name(server_name);

    println!("[SERVER] Starting TUIO server: {}", server_name);

    return server;
}

fn tuio_new_finger (finger: Finger, server: &mut Server, ids_to_cursors: &mut Vec<IdToCursor>) {
    let coordinates = finger.normalised_history.last().unwrap();

    let cursor: i32 = server.create_cursor(coordinates.0, coordinates.1);

    let id_to_cursor = IdToCursor { id: finger.id.unwrap(), cursor };
    ids_to_cursors.push(id_to_cursor);
}

fn tuio_update_finger (finger: Finger, server: &mut Server, ids_to_cursors: &mut Vec<IdToCursor>) {
    let coordinates = finger.normalised_history.last().unwrap();

    let cursor: i32 = ids_to_cursors.iter().find(|id_to_cursor| id_to_cursor.id == finger.id.unwrap()).unwrap().cursor;

    server.update_cursor(cursor, coordinates.0, coordinates.1);
}

fn tuio_remove_finger (finger_id: u32, server: &mut Server, ids_to_cursors: &mut Vec<IdToCursor>) {
    if ids_to_cursors.len() == 0 {
        return;
    }

    let cursor: i32 = ids_to_cursors.iter().find(|id_to_cursor| id_to_cursor.id == finger_id).unwrap().cursor;

    server.remove_cursor(cursor);
}

fn video(video_path: &str, background: &core::Mat, ids_to_cursors: &mut Vec<IdToCursor>, server: &mut Server) {
    let mut cap = videoio::VideoCapture::from_file(video_path, videoio::CAP_ANY).unwrap();

    let mut frame_counter = 0;

    let total_frames = cap
        .get(videoio::VideoCaptureProperties::CAP_PROP_FRAME_COUNT.into())
        .unwrap();
    let total_frames = total_frames as i32;


    let mut gray_background = background.clone();
    imgproc::cvt_color_def(&background, &mut gray_background, imgproc::COLOR_BGR2GRAY).unwrap();

    let mut fingers: Vec<Finger> = Vec::new();

    highgui::named_window("multi-touch", highgui::WINDOW_NORMAL).unwrap();
    highgui::resize_window("multi-touch", 720, 480).unwrap();

    loop {
        // if the last frame of the video is reached, reset the frame_counter and start again
        if frame_counter == total_frames {
            frame_counter = 0;
            cap.set(videoio::CAP_PROP_POS_FRAMES, 0.0).unwrap();
        }

        server.init_frame();

        let mut frame = core::Mat::default();
        let success = cap.read(&mut frame).unwrap();

        if success {
            let mut image = frame.clone();

            let temp_image = image.clone();
            imgproc::cvt_color_def(&temp_image, &mut image, imgproc::COLOR_BGR2GRAY).unwrap();

            let mut finger_coordinates: Vec<(i32, i32)> = Vec::new();
            image = prepare_image(&image, &gray_background);
            let ellipse_image = detect_fingers(&image, &frame, &mut finger_coordinates);

            manage_fingers(&mut fingers, &finger_coordinates, ids_to_cursors, server);
            let mut final_image = ellipse_image.clone();
            for finger in fingers.clone() {
                if finger.active == 1 {
                    let finger_location = finger.history.last().unwrap();
                    let _ = imgproc::put_text(
                        &mut final_image,
                        &finger.id.unwrap().to_string(),
                        core::Point::new(finger_location.0, finger_location.1),
                        imgproc::FONT_HERSHEY_PLAIN,
                        1.0,
                        core::Scalar::new(255.0, 0.0, 0.0, 0.0),
                        2,
                        imgproc::LINE_8,
                        false,
                    );
                }
            }

            frame_counter += 1;
            if frame_counter == total_frames {
                ids_to_cursors.clear();
                fingers.iter().for_each(|finger| tuio_remove_finger(finger.id.unwrap(), server, ids_to_cursors));
                fingers.clear();
            }
            server.commit_frame();

            highgui::imshow("multi-touch", &final_image).unwrap();
            let key = highgui::wait_key(50).unwrap();
            // let key = highgui::wait_key(0).unwrap();
            if key == 27 {
                // escape key
                break;
            }

        }
    }
}

fn manage_fingers(fingers: &mut Vec<Finger>, finger_coordinates: &Vec<(i32, i32)>,ids_to_cursors: &mut Vec<IdToCursor>, server: &mut Server) {
    // No Existing fingers
    if fingers.is_empty() {
        for (i, finger_coordinate) in finger_coordinates.iter().enumerate() {
            let new_finger = Finger::new(Some(i as u32), vec![*finger_coordinate], vec![normalise_coordinates(*finger_coordinate)], 0);
            fingers.push(new_finger.clone());
            tuio_new_finger(new_finger.clone(), server, ids_to_cursors);
        }
        return;
    }

    // Existing fingers
    for finger_coordinate in finger_coordinates {
        let nearest_finger = find_nearest_fingers(*finger_coordinate, fingers.clone(), 250);
        if nearest_finger.id.is_none() {
            let new_finger = Finger::new(Some(ids_to_cursors.len() as u32), vec![*finger_coordinate], vec![normalise_coordinates(*finger_coordinate)], 0);
            fingers.push(new_finger.clone());
            tuio_new_finger(new_finger.clone(), server, ids_to_cursors);
        } else {
            let nearest_finger = fingers
                .iter_mut()
                .find(|finger| finger.id == nearest_finger.id)
                .unwrap();
            nearest_finger.history.push(*finger_coordinate);
            nearest_finger.normalised_history.push(normalise_coordinates(*finger_coordinate));
            nearest_finger.active = 0;
            tuio_update_finger(nearest_finger.clone(), server, ids_to_cursors);
        }
    }

    //Inactive fingers
    let mut temp_fingers: Vec<Finger> = Vec::new(); // to avoid borrowing error

    fingers.iter_mut().for_each(|finger| {
        if finger.active < 10 {
            finger.active += 1;
            temp_fingers.push(finger.clone());
        } else if finger.active == 10 {
            tuio_remove_finger(finger.id.unwrap(), server, ids_to_cursors);
        }
    });
    *fingers = temp_fingers;
}

// Detects the contours of the fingers in the image
fn detect_fingers(
    image: &core::Mat,
    original_frame: &core::Mat,
    finger_coordinates: &mut Vec<(i32, i32)>,
) -> core::Mat {
    let mut contours = core::Vector::<core::Vector<core::Point>>::new();
    let mut hierarchy = core::Vector::<core::Vec4i>::new();
    imgproc::find_contours_with_hierarchy_def(
        &image,
        &mut contours,
        &mut hierarchy,
        imgproc::RETR_CCOMP,
        imgproc::CHAIN_APPROX_SIMPLE,
    )
        .unwrap();

    let mut temp_original_frame = original_frame.clone();

    if hierarchy.is_empty() {
        return temp_original_frame;
    }

    let mut idx: i32 = 0;
    while idx >= 0 {
        let con = contours.get(idx.try_into().unwrap()).unwrap();
        idx = hierarchy.get(idx.try_into().unwrap()).unwrap()[0];

        if contour_area(&con, false).unwrap() > 30.00 && con.len() > 4 {
            let ellipse = imgproc::fit_ellipse_direct(&con).unwrap();

            let center_32 = core::Point::new(ellipse.center.x as i32, ellipse.center.y as i32);

            let major_axis = (ellipse.size.width / 2.0) as f64;
            let minor_axis = (ellipse.size.height / 2.0) as f64;

            let size_i32 = Size::from((major_axis as i32, minor_axis as i32));

            if major_axis > minor_axis * 2.5 || minor_axis > major_axis * 2.5 {
                // println!("Ellipse is too long");
                continue;
            }

            let ellipse_area = std::f64::consts::PI * (major_axis / 2.0) * (minor_axis / 2.0);
            if !(5.0..=150.0).contains(&ellipse_area) {
                // println!("Ellipse too big / small");
                continue;
            }

            finger_coordinates.push((center_32.x, center_32.y));

            imgproc::ellipse(
                &mut temp_original_frame,
                center_32,
                size_i32,
                ellipse.angle.into(),
                0.0,
                360.0,
                core::Scalar::new(255.0, 0.0, 0.0, 0.0),
                1,
                8,
                0,
            )
                .unwrap();
        }
    }
    temp_original_frame
}

// Prepare image for finger contour detection
fn prepare_image(image: &core::Mat, background: &core::Mat) -> core::Mat {
    // subtract background from image
    let temp_image = image.clone();
    let mut img_subtraction = image.clone();
    //core::absdiff(&temp_image, &background, &mut img_subtraction).unwrap();
    core::subtract(
        &temp_image,
        &background,
        &mut img_subtraction,
        &core::no_array(),
        -1,
    )
        .unwrap();

    // first blur
    let temp_image = img_subtraction.clone();
    let mut img_blur = core::Mat::default();
    imgproc::blur(
        &temp_image,
        &mut img_blur,
        Size::from((20, 20)),
        Default::default(),
        0,
    )
        .unwrap();
    // subtract blur from subtraction
    let mut img_subtraction2 = core::Mat::default();

    //core::absdiff(&img_subtraction, &img_blur, &mut img_subtraction2).unwrap();

    core::subtract(
        &img_subtraction,
        &img_blur,
        &mut img_subtraction2,
        &core::no_array(),
        -1,
    )
        .unwrap();
    let img_blur2 = img_subtraction2.clone();
    /*
    // second blur
    img_blur2 = core::Mat::default();
    imgproc::blur(
        &img_subtraction2,
        &mut img_blur2,
        Size::from((1, 1)),
        Default::default(),
        0,
    )
    .unwrap();
    */
    // grayscale threshold
    let mut img_gray = core::Mat::default();
    imgproc::threshold(
        &img_blur2,
        &mut img_gray,
        12.0,
        255.0,
        imgproc::THRESH_BINARY,
    )
        .unwrap();
    // third blur
    let mut img_blur3 = core::Mat::default();
    imgproc::blur(
        &img_gray,
        &mut img_blur3,
        Size::from((5, 5)),
        Default::default(),
        0,
    )
        .unwrap();

    img_blur3
}

fn find_nearest_fingers(
    finger_coordinates: (i32, i32),
    fingers: Vec<Finger>,
    distance_to_finger_threshold: i32,
) -> Finger {
    let mut current_min_distance = 999999; // arbitrary large number
    let mut nearest_finger = Finger::new(None, Vec::new(), Vec::new(),0);

    for finger in fingers {
        let finger_location = finger.history.last().unwrap();

        let distance = (finger_coordinates.0 - finger_location.0).pow(2)
            + (finger_coordinates.1 - finger_location.1).pow(2);

        if distance < current_min_distance && distance < distance_to_finger_threshold {
            current_min_distance = distance;
            nearest_finger = finger.clone();
        }
    }
    nearest_finger
}

fn normalise_coordinates(coordinate: (i32, i32)) -> (f32, f32){
    let max_x = 752;
    let max_y = 480;

    let normalised_x = coordinate.0 as f32 / max_x as f32;
    let normalised_y = coordinate.1 as f32 / max_y as f32;

    (normalised_x, normalised_y)
}
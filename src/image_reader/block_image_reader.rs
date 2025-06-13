use std::fs::File;
use std::io::Read;

use docx_rs::{DrawingPosition, Pic};

pub fn get_image(cube_amt: i32) -> Pic {
    let mut file_name = String::new();
    let height: i32;
    let width: i32;

    match cube_amt {
        10 => {
            file_name = String::from("./img/10 Rod.png");
            height = 135;
            width = 25
        }
        1..=5 => {
            file_name.push_str("./img/Cube ");
            file_name.push_str(&cube_amt.to_string());
            file_name.push_str(".png");

            height = 20 * cube_amt;
            width = 22;
        }
        6..=9 => {
            file_name.push_str("./img/Cube ");
            file_name.push_str(&cube_amt.to_string());
            file_name.push_str(".png");

            height = 100;
            width = 44;
        }
        _ => {
            panic!("No image for amount {cube_amt}");
        }
    }

    let mut file = File::open(file_name).unwrap();
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf).unwrap();

    Pic::new(&buf).size((width * 9525) as u32, (height * 9525) as u32)
}

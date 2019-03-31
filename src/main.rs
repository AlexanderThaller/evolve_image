extern crate chrono;
extern crate image;
extern crate imageproc;
extern crate rand;

use chrono::prelude::*;
use image::{
    DynamicImage,
    GenericImage,
    GenericImageView,
    Pixel,
};
use rand::Rng;
use std::fs;
use std::path::Path;

fn image_diff(img1: &DynamicImage, img2: &DynamicImage) -> f64 {
    imageproc::stats::root_mean_squared_error(img1, img2)
}

fn main() {
    let target = image::open("target.png").expect("Cannot load target image");
    let output = Path::new("output.png");

    let (mut img1, mut img2) = if output.is_file() {
        println!("found output.png resuming with that");
        let img = image::open(output).unwrap();
        (img.clone(), img)
    } else {
        (
            DynamicImage::new_rgb8(target.width(), target.height()),
            DynamicImage::new_rgb8(target.width(), target.height()),
        )
    };

    use std::collections::HashSet;

    let mut colours = HashSet::new();

    for pixel in target.pixels() {
        let rgba = pixel.2.to_rgba();

        colours.insert(rgba);
    }

    let colours = colours.iter().cloned().collect::<Vec<_>>();

    let mut rng = rand::thread_rng();

    let mut i = 0;
    loop {
        /*let pos: (i32, i32) = (
            rng.gen_range(0, target.width() as i32),
            rng.gen_range(0, target.height() as i32),
        );
        let colour = rng.choose(&colours).unwrap();

        imageproc::drawing::draw_filled_circle_mut(&mut img1, pos, 5, *colour);*/

        let start: (f32, f32) = (
            rng.gen_range(0.0, target.width() as f32),
            rng.gen_range(0.0, target.height() as f32),
        );
        let end: (f32, f32) = (
            rng.gen_range(0.0, target.width() as f32),
            rng.gen_range(0.0, target.height() as f32),
        );
        let colour = rng.choose(&colours).unwrap();

        imageproc::drawing::draw_line_segment_mut(&mut img1, start, end, *colour);

        if image_diff(&target, &img1) < image_diff(&target, &img2) {
            img2.copy_from(&img1, 0, 0);
        } else {
            img1.copy_from(&img2, 0, 0);
        }

        if i % 100 == 0 {
            let diff = image_diff(&target, &img2);
            let timestamp = Utc::now();
            println!("time: {}, iteration: {}, diff: {}", timestamp, i, diff);

            img2.save(".tmp.png").expect("can not save image to tmp.png");

            fs::create_dir_all("saves").expect("can not create saves folder");
            fs::copy(".tmp.png", &format!("saves/{}_{}.png", timestamp, i)).expect("can not copy tmp png to saves png");
            fs::rename(".tmp.png", "output.png").expect("can not rename tmp png to output png");
        }

        i += 1;
    }
}

// Image processing module.

mod console;

wit_bindgen_rust::export!("./image_module.wit");

use console::Console;
use image_module::AlgorithmDescription;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct ImageModule;

impl image_module::ImageModule for ImageModule {
    fn algorithms(_locale: String) -> Vec<AlgorithmDescription> {
        vec![
            AlgorithmDescription {
                name: "invert".into(),
                description: "Invert the colors".into(),
            },
            AlgorithmDescription {
                name: "grayscale".into(),
                description: "Turn into grayscale".into(),
            },
        ]
    }

    fn process_image(algorithm: String, data: Vec<u8>, width: u32, height: u32) -> Option<Vec<u8>> {
        Console::log(&format!(
            "Processing image: {} {}x{} ({} bytes)",
            algorithm,
            width,
            height,
            data.len()
        ));

        if algorithm == "invert" {
            let mut res = Vec::with_capacity(data.len());
            unsafe {
                res.set_len(data.len());
            }

            let mut i = 0;
            data.chunks(4).for_each(|rgba| {
                unsafe {
                    *res.get_unchecked_mut(i) = 255 - rgba.get_unchecked(0);
                    *res.get_unchecked_mut(i + 1) = 255 - rgba.get_unchecked(1);
                    *res.get_unchecked_mut(i + 2) = 255 - rgba.get_unchecked(2);
                    *res.get_unchecked_mut(i + 3) = *rgba.get_unchecked(3);
                }
                i += 4;
            });

            Some(res)
        } else if algorithm == "grayscale" {
            let mut res = Vec::with_capacity(data.len());
            unsafe {
                res.set_len(data.len());
            }

            let mut i = 0;
            data.chunks(4).for_each(|rgba| {
                unsafe {
                    let gray = (*rgba.get_unchecked(0) as f32 * 0.07
                        + *rgba.get_unchecked(1) as f32 * 0.72
                        + *rgba.get_unchecked(2) as f32 * 0.21)
                        .clamp(0.0, 255.0) as u8;

                    *res.get_unchecked_mut(i) = gray;
                    *res.get_unchecked_mut(i + 1) = gray;
                    *res.get_unchecked_mut(i + 2) = gray;
                    *res.get_unchecked_mut(i + 3) = *rgba.get_unchecked(3);
                }
                i += 4;
            });

            Some(res)
        } else {
            Console::error(&format!("Rust: no such algorithm: {}", algorithm));
            None
        }
    }
}

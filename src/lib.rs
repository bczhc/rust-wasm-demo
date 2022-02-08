mod utils;

use crate::utils::set_panic_hook;
use crate::MyError::FormatError;
use image::{
    ColorType, DynamicImage, EncodableLayout, GenericImage, GenericImageView, ImageBuffer,
    ImageError, ImageFormat, Pixel, Rgba,
};
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use std::convert::TryInto;
use std::ffi::CString;
use std::io::{Cursor, Read, Write};
use std::iter::{repeat, repeat_with, FromIterator};
use std::net::TcpStream;
use std::time::SystemTime;
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi, Wasm64, WasmAbi};
use wasm_bindgen::describe::WasmDescribe;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let string = format!("{}, {}", "hello", "world");
    alert(&string)
}

#[derive(Debug)]
pub enum MyError {
    ImageError(ImageError),
    FormatError,
    UnknownMediaType,
}

type MyResult<T> = Result<T, MyError>;

impl From<ImageError> for MyError {
    fn from(e: ImageError) -> Self {
        Self::ImageError(e)
    }
}

pub fn convert(data: &[u8], output_format: &str) -> MyResult<String> {
    let mut writer = Cursor::new(Vec::new());
    let mut reader = Cursor::new(data);

    let guess_format = ImageFormat::from_extension(output_format);
    if let None = guess_format {
        return Err(FormatError);
    }
    let guess_format = guess_format.unwrap();

    let mut image = image::load(&mut reader, image::guess_format(data)?)?;
    image.invert();

    image.write_to(&mut writer, guess_format)?;

    let data = writer.into_inner();
    compose_base64_src(&data, &guess_format)
}

fn compose_base64_src(data: &[u8], format: &ImageFormat) -> MyResult<String> {
    let type_name = image_media_type(format)?;
    Ok(format!(
        "data:{};base64, {}",
        type_name,
        base64::encode(data)
    ))
}

fn image_media_type(format: &ImageFormat) -> MyResult<&'static str> {
    let mt_name = match format {
        ImageFormat::Png => "image/png",
        ImageFormat::Jpeg => "image/jpeg",
        ImageFormat::Gif => "image/gif",
        ImageFormat::WebP => "image/webp",
        ImageFormat::Pnm => "image/pbm",
        ImageFormat::Tiff => "image/tiff",
        ImageFormat::Tga => "image/tga",
        ImageFormat::Dds => "image/vnd-ms.dds",
        ImageFormat::Bmp => "image/bmp",
        ImageFormat::Ico => "image/vnd.microsoft.icon",
        ImageFormat::Hdr => "image/vnd.radiance",
        ImageFormat::OpenExr => "image/x-exr",
        ImageFormat::Avif => "image/avif",
        _ => return Err(MyError::UnknownMediaType),
    };
    Ok(mt_name)
}

#[wasm_bindgen]
pub fn call(data: &[u8], output_format: &str) -> Result<String, String> {
    set_panic_hook();

    let result = convert(data, output_format);
    if let Err(e) = result {
        Err(format!("{:?}", e))
    } else {
        Ok(result.unwrap())
    }
}

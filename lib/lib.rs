pub mod c;
pub use c::{captcha, HEIGHT, IMG_SIZE, WIDTH};

use std::io::Write;

pub fn gif<W: Write>(writer: &mut W) -> Box<str> {
  let (word, mut img) = unsafe { captcha() };

  if let Ok(mut encoder) = gif::Encoder::new(writer, WIDTH, HEIGHT, &[0, 0, 0, 0xFF, 0xFF, 0xFF]) {
    for i in img.iter_mut() {
      if *i == 255 {
        *i = 1;
      }
    }

    let frame = gif::Frame::from_indexed_pixels(WIDTH, HEIGHT, &img, None);

    let _ = encoder.write_frame(&frame);
  }
  let word = unsafe { std::string::String::from_utf8_unchecked(word.to_vec()) };
  word.into()
}

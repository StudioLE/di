use crate::prelude::*;
use fast_image_resize::images::Image;
use fast_image_resize::{FilterType, IntoImageView, ResizeAlg, ResizeOptions, Resizer};
use image::codecs::gif::GifEncoder;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::codecs::webp::WebPEncoder;
use image::{DynamicImage, ExtendedColorType, ImageEncoder, ImageFormat, ImageReader};
use std::fs::write;

const RESIZE_ALGORITHM: ResizeAlg = ResizeAlg::Interpolation(FilterType::CatmullRom);

pub struct Resize {
    format: ImageFormat,
    image: DynamicImage,
    color_type: ExtendedColorType,
}

impl Resize {
    pub fn new(path: &PathBuf) -> Result<Resize, Report<ResizeError>> {
        let reader = ImageReader::open(path)
            .change_context(ResizeError::Open)?
            .with_guessed_format()
            .change_context(ResizeError::Format)?;
        let format = reader.format().ok_or(ResizeError::Format)?;
        let image = reader.decode().change_context(ResizeError::Decode)?;
        let color_type = image.color().into();
        Ok(Self {
            format,
            image,
            color_type,
        })
    }

    pub fn to_file(&self, path: &Path, width: u32, height: u32) -> Result<(), Report<ResizeError>> {
        let bytes = self.to_bytes(width, height)?;
        let extension = self
            .format
            .extensions_str()
            .first()
            .expect("should be at least one image extension");
        let path = path.with_extension(extension);
        write(&path, bytes)
            .change_context(ResizeError::Write)
            .attach_path(&path)
    }

    fn to_bytes(&self, width: u32, height: u32) -> Result<Vec<u8>, Report<ResizeError>> {
        let mut target = Image::new(
            width,
            height,
            self.image
                .pixel_type()
                .expect("source image should have a pixel type"),
        );
        let mut resizer = Resizer::new();
        let options = ResizeOptions::default()
            .resize_alg(RESIZE_ALGORITHM)
            .fit_into_destination(None);
        resizer
            .resize(&self.image, &mut target, &options)
            .change_context(ResizeError::Resize)?;
        let mut buffer = Vec::new();
        let result = match self.format {
            ImageFormat::Png => PngEncoder::new(&mut buffer).write_image(
                target.buffer(),
                width,
                height,
                self.color_type,
            ),
            ImageFormat::Jpeg => JpegEncoder::new(&mut buffer).write_image(
                target.buffer(),
                width,
                height,
                self.color_type,
            ),
            ImageFormat::Gif => GifEncoder::new(&mut buffer).write_image(
                target.buffer(),
                width,
                height,
                self.color_type,
            ),
            ImageFormat::WebP => WebPEncoder::new_lossless(&mut buffer).write_image(
                target.buffer(),
                width,
                height,
                self.color_type,
            ),
            format => {
                let report = Report::new(ResizeError::Format).attach(format!("Format: {format:?}"));
                return Err(report);
            }
        };
        result.change_context(ResizeError::Encode)?;
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "uses httpbin.org"]
    pub async fn resize_jpeg() {
        // Arrange
        let services = ServiceProvider::new();
        let http = services
            .get_service::<HttpClient>()
            .await
            .expect("should be able to get HttpClient");
        let formats = vec!["jpeg", "png", "webp"];
        for format in formats {
            eprintln!("format: {format}");
            let url = Url::parse(&format!("https://httpbin.org/image/{format}"))
                .expect("url should be valid");
            let path = http
                .get(&url, None)
                .await
                .expect("get image should not fail");
            let _logger = init_test_logger();

            // Act
            let result = Resize::new(&path).assert_ok_debug().to_bytes(100, 100);

            // Assert
            let bytes = result.assert_ok_debug();
            assert!(!bytes.is_empty());
        }
    }
}

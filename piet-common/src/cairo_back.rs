// allows e.g. raw_data[dst_off + x * 4 + 2] = buf[src_off + x * 4 + 0];
#![allow(clippy::identity_op)]

//! Support for piet Cairo back-end.

use cairo::{Context, Format, ImageSurface};
#[cfg(feature = "png")]
use png::{ColorType, Encoder};
#[cfg(feature = "png")]
use std::fs::File;
#[cfg(feature = "png")]
use std::io::BufWriter;
use std::marker::PhantomData;
use std::path::Path;

use piet::{ErrorKind, ImageFormat};
#[doc(hidden)]
pub use piet_cairo::*;

/// The `RenderContext` for the Cairo backend, which is selected.
pub type Piet<'a> = CairoRenderContext<'a>;

/// The associated brush type for this backend.
///
/// This type matches `RenderContext::Brush`
pub type Brush = piet_cairo::Brush;

/// The associated text factory for this backend.
///
/// This type matches `RenderContext::Text`
pub type PietText<'a> = CairoText<'a>;

/// The associated font type for this backend.
///
/// This type matches `RenderContext::Text::Font`
pub type PietFont = CairoFont;

/// The associated font builder for this backend.
///
/// This type matches `RenderContext::Text::FontBuilder`
pub type PietFontBuilder<'a> = CairoFontBuilder;

/// The associated text layout type for this backend.
///
/// This type matches `RenderContext::Text::TextLayout`
pub type PietTextLayout = CairoTextLayout;

/// The associated text layout builder for this backend.
///
/// This type matches `RenderContext::Text::TextLayoutBuilder`
pub type PietTextLayoutBuilder<'a> = CairoTextLayoutBuilder;

/// The associated image type for this backend.
///
/// This type matches `RenderContext::Image`
pub type Image = ImageSurface;

/// A struct that can be used to create bitmap render contexts.
///
/// In the case of Cairo, being a software renderer, no state is needed.
pub struct Device;

/// A struct provides a `RenderContext` and then can have its bitmap extracted.
pub struct BitmapTarget<'a> {
    surface: ImageSurface,
    cr: Context,
    phantom: PhantomData<&'a ()>,
}

impl Device {
    /// Create a new device.
    pub fn new() -> Result<Device, piet::Error> {
        Ok(Device)
    }

    /// Create a new bitmap target.
    pub fn bitmap_target(
        &mut self,
        width: usize,
        height: usize,
        pix_scale: f64,
    ) -> Result<BitmapTarget, piet::Error> {
        let surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32).unwrap();
        let cr = Context::new(&surface);
        cr.scale(pix_scale, pix_scale);
        let phantom = Default::default();
        Ok(BitmapTarget {
            surface,
            cr,
            phantom,
        })
    }
}

impl<'a> BitmapTarget<'a> {
    /// Get a piet `RenderContext` for the bitmap.
    ///
    /// Note: caller is responsible for calling `finish` on the render
    /// context at the end of rendering.
    pub fn render_context(&mut self) -> CairoRenderContext {
        CairoRenderContext::new(&mut self.cr)
    }

    /// Get raw RGBA pixels from the bitmap.
    pub fn into_raw_pixels(mut self, fmt: ImageFormat) -> Result<Vec<u8>, piet::Error> {
        self.get_raw_pixels(fmt)
    }

    /// Get raw RGBA pixels without consuming the bitmap.
    pub fn get_raw_pixels(&mut self, fmt: ImageFormat) -> Result<Vec<u8>, piet::Error> {
        // TODO: convert other formats.
        if fmt != ImageFormat::RgbaPremul {
            return Err(piet::new_error(ErrorKind::NotSupported));
        }

        // A bad hack; because we take self by reference we can't move cr (even
        // temporarily) so instead we put in a fake cr while we're extracting
        // data from the surface.
        //
        // I don't know of a better way to create a `Context` that we'll never
        // use; if the fields on `Context` were public we could just construct
        // one with a null pointer.
        let surface = ImageSurface::create(Format::ARgb32, 0, 0).unwrap();
        let temp = Context::new(&surface);
        drop(std::mem::replace(&mut self.cr, temp));

        self.surface.flush();
        let stride = self.surface.get_stride() as usize;
        let width = self.surface.get_width() as usize;
        let height = self.surface.get_height() as usize;
        let mut raw_data = vec![0; width * height * 4];
        let buf = self
            .surface
            .get_data()
            .map_err(Into::<Box<dyn std::error::Error>>::into)?;
        for y in 0..height {
            let src_off = y * stride;
            let dst_off = y * width * 4;
            for x in 0..width {
                raw_data[dst_off + x * 4 + 0] = buf[src_off + x * 4 + 2];
                raw_data[dst_off + x * 4 + 1] = buf[src_off + x * 4 + 1];
                raw_data[dst_off + x * 4 + 2] = buf[src_off + x * 4 + 0];
                raw_data[dst_off + x * 4 + 3] = buf[src_off + x * 4 + 3];
            }
        }

        drop(buf);
        self.cr = Context::new(&self.surface);

        Ok(raw_data)
    }

    /// Save bitmap to RGBA PNG file
    #[cfg(feature = "png")]
    pub fn save_to_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), piet::Error> {
        let height = self.surface.get_height();
        let width = self.surface.get_width();
        let image = self.get_raw_pixels(ImageFormat::RgbaPremul)?;
        let file = BufWriter::new(File::create(path).map_err(|e| Into::<Box<_>>::into(e))?);
        let mut encoder = Encoder::new(file, width as u32, height as u32);
        encoder.set_color(ColorType::RGBA);
        encoder
            .write_header()
            .map_err(|e| Into::<Box<_>>::into(e))?
            .write_image_data(&image)
            .map_err(|e| Into::<Box<_>>::into(e))?;
        Ok(())
    }

    /// Stub for feature is missing
    #[cfg(not(feature = "png"))]
    pub fn save_to_file<P: AsRef<Path>>(&mut self, _path: P) -> Result<(), piet::Error> {
        Err(piet::new_error(ErrorKind::MissingFeature))
    }
}

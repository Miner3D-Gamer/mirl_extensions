// TODO: Implement me!

use mirl_core::{Buffer, graphics::rgba_list_to_argb_list};

// #[cfg(not(target_arch = "wasm32"))]
// #[cfg(feature = "glfw")]
// #[cfg(feature = "std")]
// use glfw::PixelImage;
/// Convert a Buffer into a `glfw::PixelImage`
#[inline(always)]
#[must_use]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn buffer_to_pixel_image(buffer: &mirl_core::Buffer) -> glfw::PixelImage {
    glfw::PixelImage {
        width: buffer.width as u32,
        height: buffer.height as u32,
        pixels: mirl_core::graphics::switch_red_and_blue_list(&buffer.data),
    }
}
/// Convert a `glfw::PixelImage` into a Buffer
#[inline(always)]
#[must_use]
#[allow(clippy::missing_panics_doc, clippy::unwrap_used)]
pub fn pixel_image_to_buffer(pixel_image: &glfw::PixelImage) -> Buffer {
    unsafe {
        Buffer::new(
            (pixel_image.width as usize, pixel_image.height as usize),
            rgba_list_to_argb_list(&pixel_image.pixels),
        )
        .unwrap_unchecked()
    }
}

impl crate::FromPatch<Buffer> for glfw::PixelImage {
    fn from_value(buffer: Buffer) -> Self {
        buffer_to_pixel_image(&buffer)
    }
}

impl crate::FromPatch<glfw::PixelImage> for Buffer {
    fn from_value(pixel_image: glfw::PixelImage) -> Self {
        pixel_image_to_buffer(&pixel_image)
    }
}

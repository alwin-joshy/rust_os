use bootloader_api::info::{FrameBuffer, PixelFormat};
use embedded_graphics::{pixelcolor::{PixelColor, RgbColor}, geometry::{OriginDimensions, Size, Point}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct Display {
    framebuffer: FrameBuffer,
}


impl From<embedded_graphics::geometry::Point> for Position {
    fn from(value: embedded_graphics::geometry::Point) -> Self  {
        Position  { x : value.x as usize, y : value.y as usize}
    }
}

impl Display {
    pub fn new(framebuffer: FrameBuffer) -> Display {
        Self { framebuffer }
    }

    fn draw_pixel<C: RgbColor>(&mut self, pixel: embedded_graphics::Pixel<C>) {
        // ignore any pixels that are out of bounds.
        let (width, height) = {
            let info = self.framebuffer.info();
            (info.width, info.height)
        };
        
        let x = pixel.0.x as usize;
        let y = pixel.0.y as usize;
        if x < width && y < height {
            let color = Color { red: pixel.1.r(), green: pixel.1.g(), blue: pixel.1.b()};
            set_pixel_in(&mut self.framebuffer, Position {x, y}, color);
        }
    }
}

impl OriginDimensions for Display {
    fn size(&self) -> embedded_graphics::prelude::Size {
        let info = self.framebuffer.info();
        Size {width : info.width as u32, height : info.height as u32}
    }
}

impl embedded_graphics::draw_target::DrawTarget for Display {
    type Color = embedded_graphics::pixelcolor::Rgb888;

    /// Drawing operations can never fail.
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        for pixel in pixels.into_iter() {
            self.draw_pixel(pixel);
        }
        Ok(())
    }
}

pub fn set_pixel_in(framebuffer: &mut FrameBuffer, position: Position, color : Color) {
    let info = framebuffer.info();
    let byte_offset = ((position.y * info.stride) + position.x) * info.bytes_per_pixel;

    let pixel_buffer = &mut framebuffer.buffer_mut()[byte_offset..];
    match info.pixel_format {
        PixelFormat::Rgb => {
            pixel_buffer[0] = color.red;
            pixel_buffer[1] = color.green;
            pixel_buffer[2] = color.blue;
        },
        PixelFormat::Bgr => {
            pixel_buffer[0] = color.blue;
            pixel_buffer[1] = color.green;
            pixel_buffer[2] = color.red;
        }, 
        PixelFormat::U8 => {
            /* Simple greyscale transform */
            pixel_buffer[0] = color.red / 3 + color.green / 3 + color.blue / 3;
        },
        _ => panic!("Unknown pixel format")
    }
}
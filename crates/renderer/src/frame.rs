use crate::error::{RendererError, RendererResult};

pub struct Frame<'a> {
    pixels: &'a mut [u8],
    width: u32,
    height: u32,
}

impl<'a> Frame<'a> {
    pub(crate) fn new(pixels: &'a mut [u8], width: u32, height: u32) -> RendererResult<Self> {
        let expected = (width as usize)
            .checked_mul(height as usize)
            .and_then(|pixels| pixels.checked_mul(4))
            .ok_or(RendererError::InvalidFramebuffer)?;

        if width == 0 || height == 0 || pixels.len() != expected {
            return Err(RendererError::InvalidFramebuffer);
        }

        Ok(Self {
            pixels,
            width,
            height
        })
    } 

    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }

    pub fn clear(&mut self, color: [u8; 4]) {
        for pixel in self.pixels.chunks_exact_mut(4) {
            pixel.copy_from_slice(&color);
        }
    }

    pub fn fill_span(&mut self, x_start: i64, x_end: i64, y: i64, color: [u8; 4]) {
        if y < 0 || y >= self.height as i64 {
            return;
        }

        let left = x_start.max(0);
        let right = x_end.min(self.width as i64 - 1);

        if left > right {
            return;
        }

        for x in left..=right {
            let index = (y as usize * self.width as usize + x as usize) * 4;
            self.pixels[index..index + 4].copy_from_slice(&color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_matching_buffer() {
        let mut buf = vec![0; 4 * 3 * 4];
        assert!(Frame::new(&mut buf, 4, 3).is_ok());
    }

    #[test]
    fn rejects_wrong_buffer_size() {
        let mut too_small = vec![0; 100];
        let mut too_big = vec![0; 20 * 20 * 4 + 4];

        assert!(matches!(Frame::new(&mut too_small, 20, 20), Err(RendererError::InvalidFramebuffer)));
        assert!(matches!(Frame::new(&mut too_big, 20, 20), Err(RendererError::InvalidFramebuffer)));
    }

    #[test]
    fn rejects_zero_dimensions() {
        let mut empty: Vec<u8> = Vec::new();

        assert!(matches!(Frame::new(&mut empty, 0, 10), Err(RendererError::InvalidFramebuffer)));
        assert!(matches!(Frame::new(&mut empty, 10, 0), Err(RendererError::InvalidFramebuffer)));
    }

    #[test]
    fn clear_fills_every_pixel() {
        let mut buf = vec![0; 3 * 2 * 4];
        Frame::new(&mut buf, 3, 2).unwrap().clear([1, 2, 3, 4]);

        assert!(buf.chunks_exact(4).all(|p| p == [1, 2, 3, 4]));
    }

    #[test]
    fn fill_span_clips_to_frame() {
        let mut buf = vec![0; 4 * 2 * 4];
        {
            let mut frame = Frame::new(&mut buf, 4, 2).unwrap();
            frame.fill_span(-10, 10, 1, [9, 9, 9, 9]); // wider than the frame
            frame.fill_span(0, 3, 5, [7, 7, 7, 7]); // row out of bounds: ignored
        }

        let (row0, row1) = buf.split_at(4 * 4);
        assert!(row0.iter().all(|&b| b == 0));
        assert!(row1.iter().all(|&b| b == 9));
    }
}

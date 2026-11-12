use crate::{error::{RendererError, RendererResult}, frame::Frame, shapes::drawable::Drawable};

pub struct Circle {
    radius: f32,
}

impl Circle {
    pub const MAX_RADIUS: f32 = 10_000.0;

    pub fn new(radius: f32) -> RendererResult<Self> {
        if !(0.5..=Self::MAX_RADIUS).contains(&radius) {
            return Err(RendererError::InvalidRadius);
        }

        Ok(Self {radius} )
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Drawable for Circle {
    fn draw(
        &self,
        frame: &mut Frame,
        cx: i64,
        cy: i64,
    ) -> RendererResult<()>
    {
       draw_circle(frame, cx, cy, self.radius) 
    }
}

const CIRCLE_COLOR: [u8; 4] = [255, 255, 255, 1];

fn draw_circle(
    frame: &mut Frame,
    cx: i64,
    cy: i64,
    radius: f32,
) -> RendererResult<()>{

    let radius = radius.round() as i32;

    let mut x: i64 = 0;
    let mut y = radius as i64;
    let mut decision = 1 - y;

    while x <= y {
        frame.fill_span(cx - x, cx + x, cy + y, CIRCLE_COLOR);
        frame.fill_span(cx - x, cx + x, cy - y, CIRCLE_COLOR);
        frame.fill_span(cx - y, cx + y, cy + x, CIRCLE_COLOR);
        frame.fill_span(cx - y, cx + y, cy - x, CIRCLE_COLOR);

        x += 1;

        if decision < 0 {
            decision += 2 * x + 1;
        } else {
            y -= 1;
            decision += 2 * (x - y) + 1;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const WIDTH: u32 = 20;
    const HEIGHT: u32 = 20;

    /// Draws a circle into a fresh blank buffer and returns the result plus the buffer.
    fn render(cx: i64, cy: i64, radius: f32) -> (RendererResult<()>, Vec<u8>) {
        let mut buf = vec![0; (WIDTH * HEIGHT * 4) as usize];
        let result = {
            let mut frame = Frame::new(&mut buf, WIDTH, HEIGHT).unwrap();
            draw_circle(&mut frame, cx, cy, radius)
        }; // frame dropped here, so `buf` can be read again
        (result, buf)
    }

    fn is_pixel_colored(buf: &[u8], x: u32, y: u32) -> bool {
        let index = ((y * WIDTH + x) * 4) as usize;
        buf[index..index + 4] == CIRCLE_COLOR
    }

    #[test]
    fn draws_circle_at_expected_position() {
        let (result, buf) = render(10, 10, 3.0);

        assert!(result.is_ok());
        assert!(is_pixel_colored(&buf, 10, 10));
        assert!(!is_pixel_colored(&buf, 0, 0));
    }

    #[test]
    fn fills_exactly_up_to_the_radius() {
        let (result, buf) = render(10, 10, 3.0);
        assert!(result.is_ok());

        // edges at distance r are filled
        assert!(is_pixel_colored(&buf, 13, 10));
        assert!(is_pixel_colored(&buf, 7, 10));
        assert!(is_pixel_colored(&buf, 10, 13));
        assert!(is_pixel_colored(&buf, 10, 7));

        // one past the radius is not
        assert!(!is_pixel_colored(&buf, 14, 10));
        assert!(!is_pixel_colored(&buf, 10, 14));

        // bounding-box corners are outside the circle
        assert!(!is_pixel_colored(&buf, 13, 13));
        assert!(!is_pixel_colored(&buf, 7, 7));

        // diagonal inside the circle is filled
        assert!(is_pixel_colored(&buf, 12, 12));
    }

    fn is_invalid_radius(radius: f32) -> bool {
        matches!(Circle::new(radius), Err(RendererError::InvalidRadius))
    }

    #[test]
    fn rejects_zero_radius() {
        assert!(is_invalid_radius(0.0));
    }

    #[test]
    fn rejects_negative_radius() {
        assert!(is_invalid_radius(-5.0));
    }

    #[test]
    fn rejects_non_finite_radius() {
        assert!(is_invalid_radius(f32::NAN));
        assert!(is_invalid_radius(f32::INFINITY));
        assert!(is_invalid_radius(f32::NEG_INFINITY));
    }

    #[test]
    fn rejects_radius_above_maximum() {
        assert!(is_invalid_radius(Circle::MAX_RADIUS + 1.0));
    }

    #[test]
    fn radius_boundaries() {
        assert!(is_invalid_radius(0.3)); // would round to 0
        assert!(Circle::new(0.5).is_ok());
        assert!(Circle::new(Circle::MAX_RADIUS).is_ok());
    }

    #[test]
    fn draws_circle_partially_outside_framebuffer() {
        let (result, buf) = render(0, 0, 5.0);

        assert!(result.is_ok());
        assert!(is_pixel_colored(&buf, 0, 0));
    }

    #[test]
    fn circle_is_symmetric() {
        let (result, buf) = render(10, 10, 5.0);
        assert!(result.is_ok());

        for dy in 0..=5 {
            for dx in 0..=5 {
                let colored = is_pixel_colored(&buf, 10 + dx, 10 + dy);
                assert_eq!(colored, is_pixel_colored(&buf, 10 - dx, 10 + dy));
                assert_eq!(colored, is_pixel_colored(&buf, 10 + dx, 10 - dy));
                assert_eq!(colored, is_pixel_colored(&buf, 10 - dx, 10 - dy));
                assert_eq!(colored, is_pixel_colored(&buf, 10 + dy, 10 + dx)); // diagonal mirror
            }
        }
    }

    #[test]
    fn handles_circle_completely_outside_framebuffer() {
        let (result, buf) = render(100, 100, 5.0);

        assert!(result.is_ok());
        assert!(buf.iter().all(|&byte| byte == 0));
    }
}

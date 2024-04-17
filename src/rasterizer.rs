use crate::{canvas::Canvas, color::Color};

#[derive(Clone, Copy, Debug)]
pub struct CanvasPoint {
    x: i32,
    y: i32,
}

impl CanvasPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub fn interpolate(start: CanvasPoint, end: CanvasPoint) -> Vec<CanvasPoint> {
    if start.x == end.x {
        return vec![start];
    }
    let mut values = Vec::new();
    let a = (end.y - start.y) as f32 / (end.x - start.x) as f32;
    let mut y = start.y as f32;
    for x in start.x..=end.x {
        values.push(CanvasPoint::new(x, y as i32));
        y += a;
    }
    values
}

pub fn draw_line(canvas: &mut Canvas, start: CanvasPoint, end: CanvasPoint, color: Color) {
    let mut start = start;
    let mut end = end;
    if (end.x - start.x).abs() > (end.y - start.y).abs() {
        if start.x > end.x {
            std::mem::swap(&mut start, &mut end);
        }
        let points = interpolate(start, end);
        for point in points {
            canvas.put_pixel(point.x, point.y, color);
        }
    } else {
        if start.y > end.y {
            std::mem::swap(&mut start, &mut end);
        }
        let points = interpolate(
            CanvasPoint::new(start.y, start.x),
            CanvasPoint::new(end.y, end.x),
        );
        for point in points {
            canvas.put_pixel(point.y, point.x, color);
        }
    }
}

use glam::Vec2;

use crate::{canvas::Canvas, color::Color};

pub struct Vertex {
    pos: Vec2,
    h: f32,
}

impl Vertex {
    pub fn new(pos: Vec2, h: f32) -> Self {
        Self { pos, h }
    }
}

fn interpolate(start: (i32, f32), end: (i32, f32)) -> Vec<f32> {
    if start.0 == end.0 {
        return vec![start.1];
    }
    let mut values = Vec::new();
    let m = (end.1 - start.1) / (end.0 - start.0) as f32;
    let mut y = start.1;
    for _ in start.0..=end.0 {
        values.push(y);
        y += m;
    }
    values
}

pub fn draw_line(canvas: &mut Canvas, a: Vec2, b: Vec2, color: Color) {
    let mut start = (a.x as i32, a.y as i32);
    let mut end = (b.x as i32, b.y as i32);
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    if dx.abs() > dy.abs() {
        if start.0 > end.0 {
            std::mem::swap(&mut start, &mut end);
        }
        let ys = interpolate((start.0, start.1 as f32), (end.0, end.1 as f32));
        for x in start.0..=end.0 {
            let index = (x - start.0) as usize;
            canvas.put_pixel(x, ys[index] as i32, color);
        }
    } else {
        if start.1 > end.1 {
            std::mem::swap(&mut start, &mut end);
        }
        let xs = interpolate((start.1, start.0 as f32), (end.1, end.0 as f32));
        for y in start.1..=end.1 {
            let index = (y - start.1) as usize;
            canvas.put_pixel(xs[index] as i32, y, color);
        }
    }
}

pub fn draw_wireframe_triangle(canvas: &mut Canvas, a: Vec2, b: Vec2, c: Vec2, color: Color) {
    draw_line(canvas, a, b, color);
    draw_line(canvas, b, c, color);
    draw_line(canvas, c, a, color);
}

pub fn draw_filled_triangle(canvas: &mut Canvas, a: Vec2, b: Vec2, c: Vec2, color: Color) {
    // Sort the points so that a.y <= b.y <= c.y
    let mut a = a;
    let mut b = b;
    let mut c = c;
    if b.y < a.y {
        std::mem::swap(&mut b, &mut a);
    }
    if c.y < a.y {
        std::mem::swap(&mut c, &mut a);
    }
    if c.y < b.y {
        std::mem::swap(&mut c, &mut b);
    }

    // Compute the x coordinates of the triangle edges
    let mut x_ab = interpolate((a.y as i32, a.x), (b.y as i32, b.x));
    let x_bc = interpolate((b.y as i32, b.x), (c.y as i32, c.x));
    let x_ac = interpolate((a.y as i32, a.x), (c.y as i32, c.x));

    // Concatenate the short sides
    x_ab.pop();
    let x_abc: Vec<f32> = x_ab.iter().chain(x_bc.iter()).copied().collect();

    // Determine which is left and which is right
    let x_left;
    let x_right;
    let middle = x_abc.len() / 2;
    if x_ac[middle] < x_abc[middle] {
        x_left = x_ac;
        x_right = x_abc;
    } else {
        x_left = x_abc;
        x_right = x_ac;
    }

    // Draw the horizontal segments
    for y in a.y as i32..=c.y as i32 {
        let index = (y - a.y as i32) as usize;
        for x in x_left[index] as i32..=x_right[index] as i32 {
            canvas.put_pixel(x, y, color);
        }
    }
}

pub fn draw_shaded_triangle(canvas: &mut Canvas, a: Vertex, b: Vertex, c: Vertex, color: Color) {
    // Sort the points so that a.y <= b.y <= c.y
    let mut a = a;
    let mut b = b;
    let mut c = c;
    if b.pos.y < a.pos.y {
        std::mem::swap(&mut b, &mut a);
    }
    if c.pos.y < a.pos.y {
        std::mem::swap(&mut c, &mut a);
    }
    if c.pos.y < b.pos.y {
        std::mem::swap(&mut c, &mut b);
    }

    // Compute the x coordinates and h values of the triangle edges
    let mut x_ab = interpolate((a.pos.y as i32, a.pos.x), (b.pos.y as i32, b.pos.x));
    let mut h_ab = interpolate((a.pos.y as i32, a.h), (b.pos.y as i32, b.h));

    let x_bc = interpolate((b.pos.y as i32, b.pos.x), (c.pos.y as i32, c.pos.x));
    let h_bc = interpolate((b.pos.y as i32, b.h), (c.pos.y as i32, c.h));

    let x_ac = interpolate((a.pos.y as i32, a.pos.x), (c.pos.y as i32, c.pos.x));
    let h_ac = interpolate((a.pos.y as i32, a.h), (c.pos.y as i32, c.h));

    // Concatenate the short sides
    x_ab.pop();
    let x_abc: Vec<f32> = x_ab.iter().chain(x_bc.iter()).copied().collect();

    h_ab.pop();
    let h_abc: Vec<f32> = h_ab.iter().chain(h_bc.iter()).copied().collect();

    // Determine which is left and which is right
    let x_left;
    let x_right;
    let h_left;
    let h_right;
    let middle = x_abc.len() / 2;
    if x_ac[middle] < x_abc[middle] {
        x_left = x_ac;
        h_left = h_ac;

        x_right = x_abc;
        h_right = h_abc;
    } else {
        x_left = x_abc;
        h_left = h_abc;

        x_right = x_ac;
        h_right = h_ac;
    }

    // Draw the horizontal segments
    for y in a.pos.y as i32..=c.pos.y as i32 {
        let index = (y - a.pos.y as i32) as usize;
        let h_segment = interpolate(
            (x_left[index] as i32, h_left[index]),
            (x_right[index] as i32, h_right[index]),
        );
        for x in x_left[index] as i32..=x_right[index] as i32 {
            let index = (x - x_left[index] as i32) as usize;
            let shaded_color = color * h_segment[index] as f32;
            canvas.put_pixel(x, y, shaded_color);
        }
    }
}

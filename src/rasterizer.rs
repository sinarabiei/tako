use glam::Vec2;

use crate::{canvas::Canvas, color::Color};

fn interpolate(start: (i32, f32), end: (i32, f32)) -> Vec<i32> {
    if start.0 == end.0 {
        return vec![start.0];
    }
    let mut values = Vec::new();
    let m = (end.1 - start.1) / (end.0 - start.0) as f32;
    let mut y = start.1;
    for _ in start.0..=end.0 {
        values.push(y as i32);
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
            canvas.put_pixel(x, ys[index], color);
        }
    } else {
        if start.1 > end.1 {
            std::mem::swap(&mut start, &mut end);
        }
        let xs = interpolate((start.1, start.0 as f32), (end.1, end.0 as f32));
        for y in start.1..=end.1 {
            let index = (y - start.1) as usize;
            canvas.put_pixel(xs[index], y, color);
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
    let x_abc: Vec<i32> = x_ab.iter().chain(x_bc.iter()).copied().collect();

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
        for x in x_left[index]..=x_right[index] {
            canvas.put_pixel(x, y, color);
        }
    }
}

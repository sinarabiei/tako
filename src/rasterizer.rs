use crate::{canvas::Canvas, color::Color};
use glam::{Mat4, Vec2, Vec3};

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

pub struct Camera {
    pub position: Vec3,
    pub orientation: Mat4,
    pub width: f32,
    pub height: f32,
    pub d: f32,
}

impl Camera {
    pub fn new(position: Vec3, orientation: Mat4) -> Self {
        // Right-handed system
        let orientation = orientation.transpose();
        Self {
            position,
            orientation,
            width: 1.0,
            height: 1.0,
            d: 1.0,
        }
    }
}

pub fn view_to_canvas(
    canvas_width: u32,
    canvas_height: u32,
    camera: &Camera,
    x: f32,
    y: f32,
) -> Vec2 {
    Vec2::new(
        x * canvas_width as f32 / camera.width,
        y * canvas_height as f32 / camera.height,
    )
}

pub fn project_vertex(
    canvas_width: u32,
    canvas_height: u32,
    camera: &Camera,
    vertex: Vec3,
) -> Vec2 {
    view_to_canvas(
        canvas_width,
        canvas_height,
        camera,
        vertex.x * camera.d / vertex.z,
        vertex.y * camera.d / vertex.z,
    )
}

pub struct Triangle {
    pub indices: [usize; 3],
    pub color: Color,
}

impl Triangle {
    pub fn new(indices: [usize; 3], color: Color) -> Self {
        Self { indices, color }
    }
}

pub fn render_object(
    canvas: &mut Canvas,
    camera: &Camera,
    vertices: &[Vec3],
    triangles: &[Triangle],
) {
    let canvas_width = canvas.width();
    let canvas_height = canvas.height();
    let mut projected = Vec::new();
    for vertex in vertices.iter() {
        let translation = Vec3::new(-1.5, 0.0, 7.0);
        let vertex_translated = *vertex + translation;
        projected.push(project_vertex(
            canvas_width,
            canvas_height,
            camera,
            vertex_translated,
        ));
    }
    for triangle in triangles.iter() {
        render_triangle(canvas, triangle, &projected);
    }
}

fn render_triangle(canvas: &mut Canvas, triangle: &Triangle, projected: &[Vec2]) {
    draw_wireframe_triangle(
        canvas,
        projected[triangle.indices[0]],
        projected[triangle.indices[1]],
        projected[triangle.indices[2]],
        triangle.color,
    );
}

pub struct Model {
    vertices: Vec<Vec3>,
    triangles: Vec<Triangle>,
}

impl Model {
    pub fn new(vertices: Vec<Vec3>, triangles: Vec<Triangle>) -> Self {
        Self {
            vertices,
            triangles,
        }
    }
}

pub struct Instance<'a> {
    model: &'a Model,
    pub scale: Vec3,
    pub rotation: Mat4,
    pub translation: Vec3,
    transform: Mat4,
}

impl<'a> Instance<'a> {
    pub fn new(model: &'a Model, scale: Vec3, rotation: Mat4, translation: Vec3) -> Self {
        // Right-handed system
        let rotation = rotation.transpose();
        Self {
            model,
            scale,
            rotation,
            translation,
            transform: Mat4::from_translation(translation) * rotation * Mat4::from_scale(scale),
        }
    }
}

pub fn render_scene(canvas: &mut Canvas, camera: &Camera, scene: &[Instance]) {
    for instance in scene.iter() {
        render_instance(canvas, camera, instance);
    }
}

pub fn render_instance(canvas: &mut Canvas, camera: &Camera, instance: &Instance) {
    let canvas_width = canvas.width();
    let canvas_height = canvas.height();
    let mut projected = Vec::new();
    let model = instance.model;
    let camera_transform_mat =
        camera.orientation.transpose() * Mat4::from_translation(camera.position * -1.0);
    for vertex in model.vertices.iter() {
        let vertex_transformed =
            (camera_transform_mat * instance.transform).mul_vec4(vertex.extend(1.0));
        projected.push(project_vertex(
            canvas_width,
            canvas_height,
            camera,
            vertex_transformed.truncate(),
        ));
    }
    for triangle in model.triangles.iter() {
        render_triangle(canvas, triangle, &projected);
    }
}

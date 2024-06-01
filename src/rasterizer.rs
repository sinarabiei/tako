use std::collections::HashSet;

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
    pub planes: [Plane; 5],
}

impl Camera {
    pub fn new(position: Vec3, orientation: Mat4, planes: [Plane; 5]) -> Self {
        // Right-handed system
        let orientation = orientation.transpose();
        Self {
            position,
            orientation,
            width: 1.0,
            height: 1.0,
            d: 1.0,
            planes,
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

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub struct Plane {
    normal: Vec3, // normalized, unit length, points to "inside" clipping volume
    d: f32,       // signed distance from the origin to the plane
}

impl Plane {
    pub fn new(normal: Vec3, d: f32) -> Self {
        Self { normal, d: -d }
    }
}

fn signed_distance(plane: Plane, point: Vec3) -> f32 {
    plane.normal.dot(point) + plane.d
}

fn bounding_sphere(vertices: &[Vec3]) -> (Vec3, f32) {
    let center: Vec3 = vertices.iter().sum::<Vec3>() / vertices.len() as f32;
    let mut radius = 0.0;
    for &vertex in vertices {
        let distance = (vertex - center).length();
        if distance > radius {
            radius = distance;
        }
    }
    (center, radius)
}

fn segment_plane_intersection(plane: Plane, a: Vec3, b: Vec3) -> Vec3 {
    let ab = b - a;
    let t = (-plane.d - plane.normal.dot(a)) / plane.normal.dot(ab);
    a + t * ab
}

fn clip_triangle(
    vertices: &mut Vec<Vec3>,
    triangles: &mut Vec<Triangle>,
    plane: Plane,
    triangle: Triangle,
) {
    let vertices_indices = [
        (vertices[triangle.indices[0]], triangle.indices[0]),
        (vertices[triangle.indices[1]], triangle.indices[1]),
        (vertices[triangle.indices[2]], triangle.indices[2]),
    ];
    let mut front_vertices_count = 0;
    let mut front_vertices_indices = Vec::new();
    let mut behind_vertices_indices = Vec::new();
    for (vertex, index) in vertices_indices {
        let signed_dist = signed_distance(plane, vertex);
        if signed_dist >= 0.0 {
            front_vertices_indices.push((vertex, index));
            front_vertices_count += 1;
        } else {
            behind_vertices_indices.push((vertex, index));
        }
    }
    if front_vertices_count == 3 {
        triangles.push(triangle);
    } else if front_vertices_count == 2 {
        let (c, _) = behind_vertices_indices[0];
        let (a, a_index) = front_vertices_indices[0];
        let (b, b_index) = front_vertices_indices[1];
        let a_prime = segment_plane_intersection(plane, a, c);
        vertices.push(a_prime);
        let a_prime_index = vertices.len() - 1;
        let b_prime = segment_plane_intersection(plane, b, c);
        vertices.push(b_prime);
        let b_prime_index = vertices.len() - 1;
        triangles.push(Triangle::new(
            [a_index, b_index, a_prime_index],
            triangle.color,
        ));
        triangles.push(Triangle::new(
            [a_prime_index, b_index, b_prime_index],
            triangle.color,
        ));
    } else if front_vertices_count == 1 {
        let (a, a_index) = front_vertices_indices[0];
        let (b, _) = behind_vertices_indices[0];
        let (c, _) = behind_vertices_indices[1];
        let b_prime = segment_plane_intersection(plane, a, b);
        vertices.push(b_prime);
        let b_prime_index = vertices.len() - 1;
        let c_prime = segment_plane_intersection(plane, a, c);
        vertices.push(c_prime);
        let c_prime_index = vertices.len() - 1;
        triangles.push(Triangle::new(
            [a_index, b_prime_index, c_prime_index],
            triangle.color,
        ));
    }
}

fn clip_triangles_against_plane(
    vertices: &mut Vec<Vec3>,
    plane: Plane,
    triangles: &[Triangle],
) -> Vec<Triangle> {
    let mut clipped_triangles = Vec::new();
    for &triangle in triangles.iter() {
        clip_triangle(vertices, &mut clipped_triangles, plane, triangle)
    }
    clipped_triangles
}

fn clip_instance_against_plane(
    vertices: &mut Vec<Vec3>,
    triangles: &[Triangle],
    plane: Plane,
) -> Option<Vec<Triangle>> {
    let mut indices = HashSet::new();
    for &triangle in triangles.iter() {
        indices.insert(triangle.indices[0]);
        indices.insert(triangle.indices[1]);
        indices.insert(triangle.indices[2]);
    }
    let mut vertices_sphere = Vec::new();
    for &index in indices.iter() {
        vertices_sphere.push(vertices[index]);
    }
    let (center, radius) = bounding_sphere(&vertices_sphere);
    let distance = signed_distance(plane, center);
    if distance > radius {
        Some(triangles.to_vec())
    } else if distance < -radius {
        None
    } else {
        let clipped_triangles = clip_triangles_against_plane(vertices, plane, triangles);
        Some(clipped_triangles)
    }
}

// `d` is signed distance from the origin to the plane
// for any point `n.p + d` is the signed distance from plane to the point
// `n` is the normal of the plane such that it points "inside" the clipping volume
// camera- width: 1, height: 1, d: sqrt(5) / 2, fov: 90 degrees

pub fn render_scene(canvas: &mut Canvas, camera: &Camera, scene: &[Instance]) {
    for instance in scene.iter() {
        render_instance(canvas, camera, instance);
    }
}

fn render_instance(canvas: &mut Canvas, camera: &Camera, instance: &Instance) {
    let canvas_width = canvas.width();
    let canvas_height = canvas.height();
    let mut vertices_camera_space = Vec::with_capacity(instance.model.vertices.len());
    let model = instance.model;
    let camera_transform_mat =
        camera.orientation.transpose() * Mat4::from_translation(camera.position * -1.0);
    for vertex in model.vertices.iter() {
        let vertex_camera_space =
            (camera_transform_mat * instance.transform).mul_vec4(vertex.extend(1.0));
        vertices_camera_space.push(vertex_camera_space.truncate());
    }
    if let Some(clipped_triangles) =
        clip_instance(&mut vertices_camera_space, &model.triangles, &camera.planes)
    {
        let mut projected_vertices = Vec::with_capacity(vertices_camera_space.len());
        for &vertex in vertices_camera_space.iter() {
            projected_vertices.push(project_vertex(canvas_width, canvas_height, camera, vertex));
        }
        for triangle in clipped_triangles.iter() {
            render_triangle(canvas, triangle, &projected_vertices);
        }
    }
}

fn clip_instance(
    vertices: &mut Vec<Vec3>,
    triangles: &[Triangle],
    planes: &[Plane],
) -> Option<Vec<Triangle>> {
    let mut clipped_triangles = triangles.to_vec();
    for &plane in planes {
        match clip_instance_against_plane(vertices, &clipped_triangles, plane) {
            None => return None,
            Some(triangles) => clipped_triangles = triangles,
        }
    }
    Some(clipped_triangles)
}

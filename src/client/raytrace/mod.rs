mod geometry;
mod objects;

use geometry::*
pub use objects;

pub fn render<S, L>(scene: S,
                    focal_distance: f32,
                    x: f32, y: f32) -> (u8, u8, u8, u8)
    where S: objects::Object {

    let direction = Vector3D::new(x, y, focal_distance);
    let ray = Ray { origin: -direction, direction: direction.normalized() };

    match trace_ray(scene, ray) {
        Some(Color {r: r, g: g, b: b}) => ((r * 255.0) as u8,
                                           (g * 255.0) as u8,
                                           (b * 255.0) as u8,
                                           255)
        None => (0, 0, 0, 0)
    }
}

fn trace_ray<S, L>(scene: S, ray: Ray) -> Option<Color>
    where S: objects::Object {

    let intersections = scene.get_intersections(ray);

    if intersections.size() == 0 {
        return None
    }

    let intersection_point = ray.origin + intersections[0] * ray.direction;

    let normal = scene.get_normal(intersection_point);
    let color = scene.get_color(intersection_point);

    color * dot(normal, ray.direction)
}

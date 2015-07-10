use geometry::*;

type Color = (f32, f32, f32);

pub trait Object {
    fn get_intersections(&self, ray: Ray) -> Vec<f32>;
    fn get_normal(&self, point: Vector3D) -> Vector3D;
    fn get_color(&self, point: Vector3D) -> Color {
        Color { r: 0.5, g: 0.5, b: 0.5 }
    }

    fn translate(self, translation: Vector3D) -> Transformation<Self> {
        Transformation::translate(self, translation)
    }
    fn scale(self, scale: f32) -> Transformation<Self> {
        Transformation::scale(self, scale)
    }
    fn set_surface<S>(self, surface: S) -> SurfaceProxy<Self, S>
        where S: Fn(Vector3D) -> Color {
        SurfaceProxy::<S>::new(self, surface)
    }
}

pub struct Sphere { }

impl Object for Sphere {
    fn get_intersections(&self, ray: Ray) -> Vec<f32> {
        //let a = 1;
        let b = dot(ray.origin, ray.direction);
        let c = ray.origin.len_squared() - 1;

        let D = b * b - c;

        if D < 0 {
            vec!();
        } else {
            let sqrtD = sqrt(D);
            vec!(-b - sqrtD, -b + sqrtD)
        }
    }

    fn get_normal(&self, point: Vector3D) -> Vector3D {
        point
    }
}

struct Transformation<O: Object> {
    object: O,
    translation: Vector3D,
    scale: f32
}

impl Transformation<O: Object> {
    pub fn translate(object: O, translation: Vector3D) -> Self {
        Transformation { object: object, translation: translation, scale: 1 }
    }

    pub fn scale(object: O, scale: f32) -> Self {
        Transformation { object: object, translation: Vector3D::new(0.0, 0.0, 0.0), scale: scale }
    }

    fn to_object_coordinates(&self, point: Vector3D) -> Vector3D {
        point * self.scale + self.translation
    }

    fn to_world_coordinates(&self, point: Vector3D) -> Vector3D {
        (point - self.translation) / self.scale
    }
}

impl Object for Transformation<O: Object> {
    fn get_intersections(&self, ray: Ray) -> Vec<f32> {
        let mut transformed_ray = Ray { origin: to_world_coordinates(ray.origin),
                                        direction: ray.direction };

        let original_intersections = self.o.get_intersections(transformed_ray);

        let new_vec = Vec::with_capacity(original_intersections.size());
        for intersection in original_intersections {
            new_vec.push(intersection * self.scale);
        }
        new_vec
    }

    fn get_normal(&self, point: Vector3D) -> Vector3D {
        self.object.get_normal(self.to_object_coordinates(point))
    }

    fn get_color(&self, point: Vector3D) -> Color {
        self.object.get_color(self.to_object_coordinates(point))
    }
}

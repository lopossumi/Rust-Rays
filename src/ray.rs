use crate::sphere::*;
use crate::vector::*;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn at(&self, distance: f64) -> Point3 {
        self.origin + distance * self.direction
    }

    pub fn color(&self) -> Color {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
        let t = sphere.hit(self);
        if t > 0.0 {
            let n = (self.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
            return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
        }
    
        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);

        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::*;

    macro_rules! assert_vec3_equal {
        ($expected:expr, $actual:expr) => {
            let tolerance = 0.0001;
            assert_approx_eq!($expected.x, $actual.x, tolerance);
            assert_approx_eq!($expected.y, $actual.y, tolerance);
            assert_approx_eq!($expected.z, $actual.z, tolerance);
        };
    }

    #[test]
    fn at_distance() {
        let ray = Ray::new(Point3::new(1.0, 1.0, 1.0), Vec3::new(3.0, 4.0, 0.0));
        let position = ray.at(5.0);
        let expected = Vec3::new(16.0, 21.0, 1.0);
        assert_vec3_equal!(expected, position);
    }
}

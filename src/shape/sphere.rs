use shape::Shape;
use point::Point;
use std::fmt;

#[derive(Debug)]
pub struct Sphere {
    center: Point,
    radius: f32
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(center: {}, radius: {})", self.center, self.radius)
    }
}

impl Shape for Sphere { }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sphere_should_assign_center_and_radius() {
        let point = Point::new(3.0, 4.4, 1.0);
        let sphere = Sphere::new(point.clone(), 22.1);

        assert_eq!(point, sphere.center);
        assert_eq!(22.1, sphere.radius);
    }

    #[test]
    fn sphere_print_debug() {
        let sphere = Sphere::new(Point::new(3.0, 4.4, 1.0), 22.1);

        assert_eq!("Sphere { center: Point { x: 3, y: 4.4, z: 1 }, radius: 22.1 }",
            format!("{:?}", sphere));
    }

    #[test]
    fn sphere_print_display() {
        let sphere = Sphere::new(Point::new(6.3, 10.0, -5.0), 22.1);

        assert_eq!("(center: (6.3, 10, -5), radius: 22.1)", format!("{}", sphere));
    }
}

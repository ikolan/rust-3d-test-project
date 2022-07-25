use nalgebra::{Isometry3, Perspective3, Point2, Point3, Unit, Vector3};

#[derive(Debug)]
pub(crate) struct Triangle {
    pub(crate) vertices: [Point3<f64>; 3],
}

#[derive(Debug)]
pub(crate) struct ScreenTriangle {
    pub(crate) vertices: [Point2<i32>; 3],
}

impl Triangle {
    pub(crate) fn new(a: Point3<f64>, b: Point3<f64>, c: Point3<f64>) -> Self {
        Self {
            vertices: [a, b, c],
        }
    }

    pub(crate) fn normal(&self) -> Unit<Vector3<f64>> {
        Unit::new_normalize(
            (self.vertices[1] - self.vertices[0]).cross(&(self.vertices[2] - self.vertices[0])),
        )
    }

    pub(crate) fn apply_camera_transform(&self, camera: &Isometry3<f64>) -> Triangle {
        let translated = self.vertices.map(|v| {
            camera.rotation
                * Point3::new(
                    v.x + camera.translation.x,
                    v.y - camera.translation.y,
                    v.z + camera.translation.z,
                )
        });

        Triangle::new(translated[0], translated[1], translated[2])
    }

    pub(crate) fn drawable(
        &self,
        camera: &Isometry3<f64>,
        normal: Option<Unit<Vector3<f64>>>,
    ) -> bool {
        let normal = normal.unwrap_or_else(|| self.normal());

        self.vertices[0].z >= crate::Z_NEAR
            && self.vertices[1].z >= crate::Z_NEAR
            && self.vertices[2].z >= crate::Z_NEAR
            && normal.dot(&(camera.rotation.inverse() * self.vertices[0].coords)) < 0.
    }

    pub(crate) fn project_on_screen_space(
        &self,
        perspective: &Perspective3<f64>,
    ) -> ScreenTriangle {
        ScreenTriangle {
            vertices: self
                .vertices
                .map(|v| perspective.project_point(&v))
                .map(|v| {
                    Point2::new(
                        (((v.x + 1.) / 2.) * crate::SCREEN_SIZE.0 as f64) as i32,
                        (((v.y + 1.) / 2.) * crate::SCREEN_SIZE.1 as f64) as i32,
                    )
                }),
        }
    }
}

impl ScreenTriangle {
    pub(crate) fn on_screen(&self) -> bool {
        let v = self.vertices.map(|v| {
            v.x >= 0 && v.x < crate::SCREEN_SIZE.0 && v.y >= 0 && v.y < crate::SCREEN_SIZE.1
        });

        v[0] || v[1] || v[2]
    }
}

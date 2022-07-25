use crate::triangle::Triangle;
use nalgebra::{Point3, Unit, Vector3};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub(crate) struct Mesh {
    pub(crate) vertices: Vec<Point3<f64>>,
    pub(crate) triangles: Vec<[usize; 3]>,
    pub(crate) normals: Vec<Unit<Vector3<f64>>>,
}

impl Mesh {
    pub(crate) fn triangles_resolved(&self) -> Vec<Triangle> {
        let mut array: Vec<Triangle> = Vec::new();
        for triangle in self.triangles.iter() {
            array.push(Triangle::new(
                *self.vertices.get(triangle[0]).unwrap(),
                *self.vertices.get(triangle[1]).unwrap(),
                *self.vertices.get(triangle[2]).unwrap(),
            ))
        }
        array
    }

    pub(crate) fn triangles_count(&self) -> usize {
        self.triangles.len()
    }

    fn process_normal(&mut self) {
        self.normals = self
            .triangles_resolved()
            .iter()
            .map(|t| t.normal())
            .collect();
    }

    pub(crate) fn from_file() -> Mesh {
        let mut result = Mesh {
            vertices: vec![],
            triangles: vec![],
            normals: vec![],
        };
        let reader = BufReader::new(File::open(env!("MODEL_PATH")).unwrap());

        for line in reader.lines() {
            let line = line.unwrap();
            let elems: Vec<&str> = line.split(' ').collect();

            match elems[0] {
                "v" => {
                    result.vertices.push(Point3::new(
                        elems[1].parse().unwrap(),
                        elems[2].parse().unwrap(),
                        elems[3].parse().unwrap(),
                    ));
                }
                "f" => {
                    result.triangles.push([
                        elems[1].parse::<usize>().unwrap() - 1,
                        elems[2].parse::<usize>().unwrap() - 1,
                        elems[3].parse::<usize>().unwrap() - 1,
                    ]);
                }
                _ => {}
            }
        }

        result.process_normal();

        result
    }
}

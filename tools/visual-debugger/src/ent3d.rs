use math::matrix::*;

#[derive(Clone, Debug, Default)]
pub struct Ent3D {
    faces: Vec<u32>,
    vertices: Vec<Vec3>,
}

pub enum Error {
    FaceIndexError,
}

impl Ent3D {
    pub fn get_vertices(&self) -> Vec<Vec3> {
        self.vertices.clone()
    }

    pub fn get_indices(&self) -> Result<Vec<Vec<u32>>, Error> {
        let mut result: Vec<Vec<u32>> = vec![];

        let mut iter = self.faces.iter();

        loop {
            let elem = iter.next();
            if elem.is_none() {
                break;
            }

            let elem = *elem.unwrap();

            let mut indices = Vec::<u32>::new();
            for _ in 0..elem {
                let index = iter.next().ok_or(Error::FaceIndexError)?;
                indices.push(*index);
            }

            result.push(indices);
        }

        Ok(result)
    }
}
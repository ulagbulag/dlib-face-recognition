use std::hash::Hash;

use super::encoding::FaceEncoding;

#[derive(Default)]
pub struct FaceComparer {
    keys: Vec<String>,
    values: Vec<FaceEncoding>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FaceObject<'a> {
    id: usize,
    name: &'a str,
}

impl FaceComparer {
    pub fn insert(&mut self, name: String, value: FaceEncoding) {
        self.keys.push(name);
        self.values.push(value);
    }

    pub fn find(&self, face: &FaceEncoding) -> Option<FaceObject<'_>> {
        const TOLERANCE: f64 = 0.6;

        if let Some((index, x)) = self
            .values
            .iter()
            .enumerate()
            .map(|(i, f)| (i, f.distance(face)))
            .min_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
        {
            if x <= TOLERANCE {
                Some(FaceObject {
                    id: index,
                    name: &self.keys[index],
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

use std::collections::HashMap;

use super::encoding::FaceEncoding;

#[derive(Default)]
pub struct FaceComparer {
    seed: usize,
    names: HashMap<usize, String>,
    values: HashMap<usize, FaceEncoding>,
}

impl FaceComparer {
    pub fn insert(&mut self, name: String, value: FaceEncoding) {
        let name_str = name.as_str();
        if let Some((&key, _)) = self.names.iter().find(|(_, n)| n.as_str() == name_str) {
            self.values.insert(key, value);
        } else {
            self.names.insert(self.seed, name);
            self.values.insert(self.seed, value);
            self.seed += 1;
        }
    }

    pub fn find(&self, face: &FaceEncoding) -> Option<usize> {
        const TOLERANCE: f64 = 0.6;

        if let Some((key, x)) = self
            .values
            .iter()
            .map(|(i, f)| (i, f.distance(face)))
            .min_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
        {
            if x <= TOLERANCE {
                Some(*key)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_name_unchecked(&self, key: &usize) -> &str {
        &self.names[key]
    }

    pub fn remove_key(&mut self, key: &usize) {
        self.names.remove(key);
        self.values.remove(key);
    }

    pub fn remove_name(&mut self, name: &str) {
        if let Some((&key, _)) = self.names.iter().find(|(_, n)| n.as_str() == name) {
            self.remove_key(&key);
        }
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }
}

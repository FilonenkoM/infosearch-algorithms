use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

type Term = String;
type DF = usize;

#[derive(Debug)]
pub struct DocumentVector {
    path: PathBuf,
    map: BTreeMap<Term, DF>,
}
impl DocumentVector {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            map: BTreeMap::new(),
        }
    }
    pub fn from_path(path: &Path) -> Option<Self> {
        let mut dv = Self::new(path.to_owned());
        dv.extend_from_path(path);
        if dv.map.len() < 1 {
            return None;
        }
        Some(dv)
    }
    pub fn from_query(query: &str) -> Self {
        let mut dv = Self::new(PathBuf::new());
        dv.extend_from_str(query.to_owned());
        dv
    }
    #[inline]
    pub fn map(&self) -> &BTreeMap<Term, DF> {
        &self.map
    }
    #[inline]
    pub fn map_mut(&mut self) -> &mut BTreeMap<Term, DF> {
        &mut self.map
    }
    #[inline]
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    pub fn extend_from_path(&mut self, path: &Path) -> &mut Self {
        match fs::read_to_string(path) {
            Ok(contents) => {
                let contents = contents.to_lowercase();
                self.extend_from_str(contents);
            }
            Err(e) => println!("{}", e),
        }
        self
    }
    pub fn extend_from_str(&mut self, contents: String) {
        for term in contents.split(|c: char| !c.is_alphanumeric()) {
            match self.map.get_mut(term) {
                Some(df) => *df += 1,
                None => {
                    self.map.insert(term.to_owned(), 1);
                }
            }
        }
    }
    pub fn len(&self) -> f32 {
        if self.map.len() == 0 {
            return 0.;
        }
        (self.map.values().map(|v| v * v).sum::<usize>() as f32).sqrt()
    }
    pub fn mul(&self, rhs: &Self) -> usize {
        let mut answer = 0;
        let mut lhs_iter = self.map.iter();
        let mut rhs_iter = rhs.map.iter();
        let mut p_1 = lhs_iter.next();
        let mut p_2 = rhs_iter.next();
        while p_1 != None && p_2 != None {
            let lhs_val = p_1.unwrap();
            let rhs_val = p_2.unwrap();
            match lhs_val.0.cmp(rhs_val.0) {
                Ordering::Equal => {
                    answer += lhs_val.1 * rhs_val.1;
                    p_1 = lhs_iter.next();
                    p_2 = rhs_iter.next();
                }
                Ordering::Less => p_1 = lhs_iter.next(),
                Ordering::Greater => p_2 = rhs_iter.next(),
            }
        }
        answer
    }

    pub fn sim(&self, rhs: &Self) -> f32 {
        if self.len() == 0. || rhs.len() == 0. {
            return 0.;
        }
        (self.mul(rhs) as f32) / (self.len() * rhs.len())
    }
}

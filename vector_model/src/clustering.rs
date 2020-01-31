use crate::document_vector::DocumentVector;
use rand::prelude::*;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

pub struct Cluster {
    documents: Vec<DocumentVector>,
}
impl Cluster {
    pub fn documents(&self) -> &Vec<DocumentVector> {
        &self.documents
    }
    pub fn documents_mut(&mut self) -> &mut Vec<DocumentVector> {
        &mut self.documents
    }
    pub fn new() -> Cluster {
        Cluster {
            documents: Vec::new(),
        }
    }
    pub fn from_dir(dir: &Path) -> Cluster {
        let mut cluster = Cluster::new();
        if let Ok(dir) = fs::read_dir(dir) {
            for entry in dir {
                if let Ok(entry) = entry {
                    if let Some(vector) = DocumentVector::from_path(&entry.path()) {
                        cluster.documents.push(vector);
                    }   
                }
            }
        }
        cluster
    }
    pub fn clusteing(&mut self) -> Vec<Cluster> {
        let mut rng = rand::thread_rng();
        let len = (self.documents.len() as f32).sqrt() as usize;
        let mut random_indices: BTreeSet<usize> = BTreeSet::new();
        while random_indices.len() != len {
            let mut random: usize = rng.gen();
            random %= len;
            random_indices.insert(random);
        }
        let mut clusters: Vec<Cluster> = Vec::new();
        for random_index in random_indices {
            let mut cluster = Cluster::new();
            cluster.documents.push(self.documents.remove(random_index));
            clusters.push(cluster);
        }
        while ! self.documents.is_empty() {
            let last = self.documents.pop().unwrap();
            let closest_index = Self::closest(&clusters, &last);
            clusters[closest_index].documents.push(last);
        }
        clusters
    }
    fn closest(clusters: &Vec<Cluster>, document: &DocumentVector) -> usize {
        let mut closest_value = clusters[0].documents[0].sim(document);
        let mut closest_index = 0;
        for i in 1..clusters.len() {
            let c_val = clusters[i].documents[0].sim(document);
            if c_val > closest_value {
                closest_index = i;
                closest_value = c_val;
            }
        }
        closest_index
    }
}

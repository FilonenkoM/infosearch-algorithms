use crate::clustering::Cluster;
use crate::document_vector::DocumentVector;
use std::cmp::Ordering;

pub struct QueryProcessor {
    clusters: Vec<Cluster>,
}
impl QueryProcessor {
    pub fn from_clusters(clusters: Vec<Cluster>) -> QueryProcessor {
        QueryProcessor { clusters }
    }
    pub fn process_query(&self, query: &str) {
        let query_vector = DocumentVector::from_query(query);
        let mut all: Vec<&DocumentVector> = self
            .clusters
            .iter()
            .flat_map(|c: &Cluster| c.documents())
            .collect();
        all.sort_by(|a, b| {
            let diff = a.sim(&query_vector) - b.sim(&query_vector);
            if diff < 0. {
                return Ordering::Greater;
            } else if diff > 0. {
                return Ordering::Less;
            }
            Ordering::Equal
        });
        println!("10 best results (without clustering)");
        for i in 0..10 {
            if i >= all.len() {
                break;
            }
            println!(
                "{} sim {}",
                all[i].path().display(),
                all[i].sim(&query_vector)
            );
        }
    }
    pub fn process_query_clipping_clusters(&self, query: &str) {
        let query_vector = DocumentVector::from_query(query);
        let mut closest_cluster_index = 0;
        let mut closest_cluster_value = self.clusters[0].documents()[0].sim(&query_vector);
        for i in 1..self.clusters.len() {
            let c_val = self.clusters[i].documents()[0].sim(&query_vector);
            if closest_cluster_value < c_val {
                closest_cluster_value = c_val;
                closest_cluster_index = i;
            }
        }
        let mut all: Vec<&DocumentVector> = self.clusters[closest_cluster_index]
            .documents()
            .iter()
            .collect();
        all.sort_by(|a, b| {
            let diff = a.sim(&query_vector) - b.sim(&query_vector);
            if diff < 0. {
                return Ordering::Greater;
            } else if diff > 0. {
                return Ordering::Less;
            }
            Ordering::Equal
        });
        println!("10 best results (with clustering)");
        println!("closest cluster index {}", closest_cluster_index);
        println!("number of clusters {}", self.clusters.len());
        for i in 0..10 {
            if i >= all.len() {
                break;
            }
            println!(
                "{} sim {}",
                all[i].path().display(),
                all[i].sim(&query_vector)
            );
        }
    }
}

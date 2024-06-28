use std::{collections::HashMap, ops::Deref};

use once_cell::sync::Lazy;

use super::{
    cluster::{
        cluster::{ConsonantCluster, CLUSTER_MAP},
        pattern::CONSONANT_CLUSTER_PATTERNS,
    },
    pattern::{ConsonantPattern, CONSONANT_PATTERNS},
};

static CONSONANTS: Lazy<Consonant> = Lazy::new(Consonant::default);

#[derive(Debug)]
pub struct Consonant {
    consonants: HashMap<&'static str, ConsonantPattern>,
    cluster_map: &'static [ConsonantCluster],
}

impl Deref for Consonant {
    type Target = HashMap<&'static str, ConsonantPattern>;

    fn deref(&self) -> &Self::Target {
        &self.consonants
    }
}

impl Default for Consonant {
    fn default() -> Self {
        let consonants = CONSONANT_PATTERNS
            .into_iter()
            .chain(CONSONANT_CLUSTER_PATTERNS)
            .collect();

        Self {
            consonants,
            cluster_map: &CLUSTER_MAP,
        }
    }
}

impl Consonant {
    pub fn get_cluster(&self, beginning: &str, following: &str) -> Option<&ConsonantCluster> {
        self.cluster_map
            .iter()
            .find(|cluster| cluster.cluster.starts_with(beginning) && cluster.cluster.ends_with(following))
    }
}

pub fn consonants() -> &'static Consonant {
    &CONSONANTS
}

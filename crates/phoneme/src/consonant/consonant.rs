use std::{collections::HashMap, ops::Deref, sync::LazyLock};

use super::{
    cluster::{
        cluster::{CLUSTER_MAP, ConsonantCluster},
        pattern::CONSONANT_CLUSTER_PATTERNS,
    },
    pattern::{CONSONANT_PATTERNS, ConsonantPattern},
};

static CONSONANTS: LazyLock<Consonant> = LazyLock::new(Consonant::default);

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

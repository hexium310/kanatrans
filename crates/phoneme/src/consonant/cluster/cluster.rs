pub const CLUSTER_MAP: [ConsonantCluster; 13] = [
    ConsonantCluster { cluster: "ts" },
    ConsonantCluster { cluster: "ds" },
    ConsonantCluster { cluster: "mb" },
    ConsonantCluster { cluster: "mm" },
    ConsonantCluster { cluster: "mp" },
    ConsonantCluster { cluster: "by" },
    ConsonantCluster { cluster: "fy" },
    ConsonantCluster { cluster: "gy" },
    ConsonantCluster { cluster: "hhy" },
    ConsonantCluster { cluster: "ky" },
    ConsonantCluster { cluster: "ly" },
    ConsonantCluster { cluster: "my" },
    ConsonantCluster { cluster: "py" },
];

#[derive(Clone, Copy, Debug)]
pub struct ConsonantCluster {
    pub cluster: &'static str,
}

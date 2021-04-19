use nu_errors::ShellError;
use serde_derive::Deserialize;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::time::Duration;

#[derive(Debug, Copy, Clone)]
pub enum DurabilityLevel {
    None = 0x00,
    Majority = 0x01,
    MajorityAndPersistOnMaster = 0x02,
    PersistToMajority = 0x03,
}

impl Default for DurabilityLevel {
    fn default() -> Self {
        DurabilityLevel::None
    }
}

impl Display for DurabilityLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let alias = match *self {
            DurabilityLevel::None => "none",
            DurabilityLevel::Majority => "majority",
            DurabilityLevel::MajorityAndPersistOnMaster => "majorityAndPersistActive",
            DurabilityLevel::PersistToMajority => "persistToMajority",
        };

        write!(f, "{}", alias)
    }
}

impl TryFrom<&str> for DurabilityLevel {
    type Error = ShellError;

    fn try_from(alias: &str) -> Result<Self, Self::Error> {
        match alias {
            "none" => Ok(DurabilityLevel::None),
            "majority" => Ok(DurabilityLevel::Majority),
            "majorityAndPersistActive" => Ok(DurabilityLevel::MajorityAndPersistOnMaster),
            "persistToMajority" => Ok(DurabilityLevel::PersistToMajority),
            _ => Err(ShellError::untagged_runtime_error(
                "invalid durability mode",
            )),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BucketType {
    Couchbase,
    Memcached,
    Ephemeral,
}

impl TryFrom<&str> for BucketType {
    type Error = ShellError;

    fn try_from(alias: &str) -> Result<Self, Self::Error> {
        match alias {
            "couchbase" => Ok(BucketType::Couchbase),
            "membase" => Ok(BucketType::Couchbase),
            "memcached" => Ok(BucketType::Memcached),
            "ephemeral" => Ok(BucketType::Ephemeral),
            _ => Err(ShellError::untagged_runtime_error("invalid bucket type")),
        }
    }
}

impl Display for BucketType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let alias = match *self {
            BucketType::Couchbase => "membase",
            BucketType::Memcached => "memcached",
            BucketType::Ephemeral => "ephemeral",
        };

        write!(f, "{}", alias)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ConflictResolutionType {
    Timestamp,
    SequenceNumber,
}

impl TryFrom<&str> for ConflictResolutionType {
    type Error = ShellError;

    fn try_from(alias: &str) -> Result<Self, Self::Error> {
        match alias {
            "lww" => Ok(ConflictResolutionType::Timestamp),
            "seqno" => Ok(ConflictResolutionType::SequenceNumber),
            _ => Err(ShellError::untagged_runtime_error(
                "invalid conflict resolution policy",
            )),
        }
    }
}

impl Display for ConflictResolutionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let alias = match *self {
            ConflictResolutionType::Timestamp => "lww",
            ConflictResolutionType::SequenceNumber => "seqno",
        };

        write!(f, "{}", alias)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EvictionPolicy {
    Full,
    ValueOnly,
    NotRecentlyUsed,
    NoEviction,
}
impl TryFrom<&str> for EvictionPolicy {
    type Error = ShellError;

    fn try_from(alias: &str) -> Result<Self, Self::Error> {
        match alias {
            "fullEviction" => Ok(EvictionPolicy::Full),
            "valueOnly" => Ok(EvictionPolicy::ValueOnly),
            "nruEviction" => Ok(EvictionPolicy::NotRecentlyUsed),
            "noEviction" => Ok(EvictionPolicy::NoEviction),
            _ => Err(ShellError::untagged_runtime_error(
                "invalid eviction policy",
            )),
        }
    }
}

impl Display for EvictionPolicy {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let alias = match *self {
            EvictionPolicy::Full => "fullEviction",
            EvictionPolicy::ValueOnly => "valueOnly",
            EvictionPolicy::NotRecentlyUsed => "nruEviction",
            EvictionPolicy::NoEviction => "noEviction",
        };

        write!(f, "{}", alias)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CompressionMode {
    Off,
    Passive,
    Active,
}

impl TryFrom<&str> for CompressionMode {
    type Error = ShellError;

    fn try_from(alias: &str) -> Result<Self, Self::Error> {
        match alias {
            "off" => Ok(CompressionMode::Off),
            "passive" => Ok(CompressionMode::Passive),
            "active" => Ok(CompressionMode::Active),
            _ => Err(ShellError::untagged_runtime_error(
                "invalid compression mode",
            )),
        }
    }
}

impl Display for CompressionMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let alias = match *self {
            CompressionMode::Off => "off",
            CompressionMode::Passive => "passive",
            CompressionMode::Active => "active",
        };

        write!(f, "{}", alias)
    }
}

pub struct BucketSettingsBuilder {
    name: String,
    ram_quota_mb: u64,
    flush_enabled: bool,
    num_replicas: u32,
    replica_indexes: bool,
    bucket_type: BucketType,
    eviction_policy: Option<EvictionPolicy>,
    max_expiry: Duration,
    compression_mode: CompressionMode,
    durability_level: DurabilityLevel,
    conflict_resolution_type: Option<ConflictResolutionType>,
}

impl BucketSettingsBuilder {
    pub fn new<S: Into<String>>(name: S) -> BucketSettingsBuilder {
        Self {
            name: name.into(),
            ram_quota_mb: 100,
            flush_enabled: false,
            num_replicas: 1,
            replica_indexes: false,
            bucket_type: BucketType::Couchbase,
            eviction_policy: None,
            max_expiry: Duration::from_secs(0),
            compression_mode: CompressionMode::Passive,
            durability_level: DurabilityLevel::None,
            conflict_resolution_type: None,
        }
    }

    pub fn ram_quota_mb(mut self, ram_quota_mb: u64) -> BucketSettingsBuilder {
        self.ram_quota_mb = ram_quota_mb;
        self
    }

    pub fn flush_enabled(mut self, enabled: bool) -> BucketSettingsBuilder {
        self.flush_enabled = enabled;
        self
    }

    pub fn num_replicas(mut self, num_replicas: u32) -> BucketSettingsBuilder {
        self.num_replicas = num_replicas;
        self
    }

    pub fn replica_indexes(mut self, enabled: bool) -> BucketSettingsBuilder {
        self.replica_indexes = enabled;
        self
    }

    pub fn bucket_type(mut self, bucket_type: BucketType) -> BucketSettingsBuilder {
        self.bucket_type = bucket_type;
        self
    }

    pub fn eviction_policy(mut self, eviction_policy: EvictionPolicy) -> BucketSettingsBuilder {
        self.eviction_policy = Some(eviction_policy);
        self
    }

    pub fn max_expiry(mut self, max_expiry: Duration) -> BucketSettingsBuilder {
        self.max_expiry = max_expiry;
        self
    }

    pub fn compression_mode(mut self, mode: CompressionMode) -> BucketSettingsBuilder {
        self.compression_mode = mode;
        self
    }

    pub fn minimum_durability_level(
        mut self,
        durability_level: DurabilityLevel,
    ) -> BucketSettingsBuilder {
        self.durability_level = durability_level;
        self
    }

    pub fn conflict_resolution_type(
        &mut self,
        conflict_resolution_type: ConflictResolutionType,
    ) -> Result<(), ShellError> {
        self.conflict_resolution_type = Some(conflict_resolution_type);

        Ok(())
    }

    pub fn build(self) -> BucketSettings {
        BucketSettings {
            name: self.name,
            ram_quota_mb: self.ram_quota_mb,
            flush_enabled: self.flush_enabled,
            num_replicas: self.num_replicas,
            replica_indexes: self.replica_indexes,
            bucket_type: self.bucket_type,
            eviction_policy: self.eviction_policy,
            max_expiry: self.max_expiry,
            compression_mode: self.compression_mode,
            durability_level: self.durability_level,
            conflict_resolution_type: self.conflict_resolution_type,
        }
    }
}

#[derive(Debug)]
pub struct BucketSettings {
    name: String,
    ram_quota_mb: u64,
    flush_enabled: bool,
    num_replicas: u32,
    replica_indexes: bool,
    bucket_type: BucketType,
    eviction_policy: Option<EvictionPolicy>,
    max_expiry: Duration,
    compression_mode: CompressionMode,
    durability_level: DurabilityLevel,
    conflict_resolution_type: Option<ConflictResolutionType>,
}

#[derive(Debug, Deserialize)]
struct JSONControllers {
    #[serde(default)]
    flush: String,
}

#[derive(Debug, Deserialize)]
struct JSONQuota {
    ram: u64,
    #[serde(rename = "rawRAM")]
    raw_ram: u64,
}

#[derive(Debug, Deserialize)]
struct JSONBucketSettings {
    name: String,
    controllers: JSONControllers,
    quota: JSONQuota,
    #[serde(rename = "replicaNumber")]
    num_replicas: u32,
    #[serde(default)]
    #[serde(rename = "replicaIndex")]
    replica_indexes: bool,
    #[serde(rename = "bucketType")]
    bucket_type: String,
    #[serde(rename = "evictionPolicy")]
    eviction_policy: String,
    #[serde(rename = "maxTTL")]
    max_expiry: u32,
    #[serde(rename = "compressionMode")]
    compression_mode: String,
    #[serde(rename = "durabilityMinLevel", default)]
    durability_level: String,
    #[serde(rename = "conflictResolutionType")]
    conflict_resolution_type: String,
}

impl TryFrom<JSONBucketSettings> for BucketSettings {
    type Error = ShellError;

    fn try_from(settings: JSONBucketSettings) -> Result<Self, Self::Error> {
        Ok(BucketSettings {
            name: settings.name,
            ram_quota_mb: settings.quota.raw_ram / 1024 / 1024,
            flush_enabled: settings.controllers.flush != "",
            num_replicas: settings.num_replicas,
            replica_indexes: settings.replica_indexes,
            bucket_type: BucketType::try_from(settings.bucket_type.as_str())?,
            eviction_policy: Some(EvictionPolicy::try_from(settings.eviction_policy.as_str())?),
            max_expiry: Default::default(),
            compression_mode: CompressionMode::try_from(settings.compression_mode.as_str())?,
            durability_level: DurabilityLevel::try_from(settings.durability_level.as_str())?,
            conflict_resolution_type: Some(ConflictResolutionType::try_from(
                settings.conflict_resolution_type.as_str(),
            )?),
        })
    }
}

impl BucketSettings {
    pub fn as_form(&self) -> Result<Vec<(&str, String)>, ShellError> {
        if self.ram_quota_mb < 100 {
            return Err(ShellError::untagged_runtime_error(
                "ram quota must be more than 100mb",
            ));
        }
        let flush_enabled = match self.flush_enabled {
            true => "1",
            false => "0",
        };
        let replica_index_enabled = match self.replica_indexes {
            true => "1",
            false => "0",
        };
        let mut form = vec![
            ("name", self.name.clone()),
            ("ramQuotaMB", self.ram_quota_mb.to_string()),
            ("flushEnabled", flush_enabled.into()),
            ("bucketType", self.bucket_type.to_string()),
            ("compressionMode", self.compression_mode.to_string()),
        ];

        match self.durability_level {
            DurabilityLevel::None => {}
            _ => {
                form.push(("durabilityMinLevel", self.durability_level.to_string()));
            }
        }

        if let Some(conflict_type) = self.conflict_resolution_type {
            form.push(("conflictResolutionType", conflict_type.to_string()));
        }

        match self.bucket_type {
            BucketType::Couchbase => {
                if let Some(eviction_policy) = self.eviction_policy {
                    match eviction_policy {
                        EvictionPolicy::NoEviction => {
                            return Err(ShellError::untagged_runtime_error(
                                "specified eviction policy cannot be used with couchbase buckets",
                            ));
                        }
                        EvictionPolicy::NotRecentlyUsed => {
                            return Err(ShellError::untagged_runtime_error(
                                "specified eviction policy cannot be used with couchbase buckets",
                            ));
                        }
                        _ => {
                            form.push(("evictionPolicy", eviction_policy.to_string()));
                        }
                    }
                }
                form.push(("replicaNumber", self.num_replicas.to_string()));
                form.push(("replicaIndex", replica_index_enabled.into()));
            }
            BucketType::Ephemeral => {
                if let Some(eviction_policy) = self.eviction_policy {
                    match eviction_policy {
                        EvictionPolicy::Full => {
                            return Err(ShellError::untagged_runtime_error(
                                "specified eviction policy cannot be used with ephemeral buckets",
                            ));
                        }
                        EvictionPolicy::ValueOnly => {
                            return Err(ShellError::untagged_runtime_error(
                                "specified eviction policy cannot be used with ephemeral buckets",
                            ));
                        }
                        _ => {
                            form.push(("evictionPolicy", eviction_policy.to_string()));
                        }
                    }
                }
                form.push(("replicaNumber", self.num_replicas.to_string()));
            }
            BucketType::Memcached => {
                if self.num_replicas > 0 {
                    return Err(ShellError::untagged_runtime_error(
                        "field cannot be used with memcached buckets",
                    ));
                }
                if self.eviction_policy.is_some() {
                    return Err(ShellError::untagged_runtime_error(
                        "field cannot be used with memcached buckets",
                    ));
                }
                form.push(("replicaIndex", replica_index_enabled.into()));
            }
        }

        Ok(form)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ram_quota_mb(&self) -> u64 {
        self.ram_quota_mb
    }

    pub fn flush_enabled(&self) -> bool {
        self.flush_enabled
    }

    pub fn num_replicas(&self) -> u32 {
        self.num_replicas
    }

    pub fn replica_indexes(&self) -> bool {
        self.replica_indexes
    }

    pub fn bucket_type(&self) -> BucketType {
        self.bucket_type
    }

    pub fn eviction_policy(&self) -> Option<EvictionPolicy> {
        self.eviction_policy
    }

    pub fn max_expiry(&self) -> Duration {
        self.max_expiry
    }

    pub fn compression_mode(&self) -> CompressionMode {
        self.compression_mode
    }

    pub fn minimum_durability_level(&self) -> DurabilityLevel {
        self.durability_level
    }

    pub fn set_ram_quota_mb(&mut self, ram_quota_mb: u64) {
        self.ram_quota_mb = ram_quota_mb;
    }

    pub fn set_flush_enabled(&mut self, enabled: bool) {
        self.flush_enabled = enabled;
    }

    pub fn set_num_replicas(&mut self, num_replicas: u32) {
        self.num_replicas = num_replicas;
    }

    pub fn set_replica_indexes(&mut self, enabled: bool) {
        self.replica_indexes = enabled;
    }

    pub fn set_bucket_type(&mut self, bucket_type: BucketType) {
        self.bucket_type = bucket_type;
    }

    pub fn set_eviction_policy(&mut self, eviction_policy: EvictionPolicy) {
        self.eviction_policy = Some(eviction_policy);
    }

    pub fn set_max_expiry(&mut self, max_expiry: Duration) {
        self.max_expiry = max_expiry;
    }

    pub fn set_compression_mode(&mut self, mode: CompressionMode) {
        self.compression_mode = mode;
    }

    pub fn set_minimum_durability_level(&mut self, durability_level: DurabilityLevel) {
        self.durability_level = durability_level;
    }
}

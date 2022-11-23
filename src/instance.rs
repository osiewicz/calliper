#![allow(unused)]
use super::utils;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct Instance {
    pub(crate) valgrind_path: String,
    pub(crate) cache: Option<CacheOptions>,
    pub(crate) branch_sim: bool,
    pub(crate) is_aslr_enabled: bool,
    pub(crate) cleanup_files: bool,
    pub(crate) parallelism: u64,
    pub(crate) collect_bus: bool,
    pub(crate) collect_atstart: bool,
}

impl Instance {
    fn new() -> Self {
        Self::default()
    }
    pub fn valgrind(mut self, path: impl Into<String>) -> Self {
        self.valgrind_path = path.into();
        self
    }
    pub fn cache(mut self, settings: impl Into<Option<CacheOptions>>) -> Self {
        self.cache = settings.into();
        self
    }
    pub fn branch_sim(mut self, is_enabled: bool) -> Self {
        self.branch_sim = is_enabled;
        self
    }
    pub fn aslr(mut self, is_enabled: bool) -> Self {
        self.is_aslr_enabled = is_enabled;
        self
    }
    pub fn cleanup_files(mut self, is_enabled: bool) -> Self {
        self.cleanup_files = is_enabled;
        self
    }
    pub fn parallelism(mut self, parallelism: u64) -> Self {
        self.parallelism = parallelism;
        self
    }
    pub fn collect_bus(mut self, is_enabled: bool) -> Self {
        self.collect_bus = is_enabled;
        self
    }
    pub fn collect_atstart(mut self, is_enabled: bool) -> Self {
        self.collect_atstart = is_enabled;
        self
    }
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd)]
pub struct CacheOptions {
    pub first_level_data: Option<CacheParameters>,
    pub first_level_code: Option<CacheParameters>,
    pub last_level: Option<CacheParameters>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct CacheParameters {
    pub size: usize,
    pub associativity: usize,
    pub line_size: usize,
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            valgrind_path: "valgrind".to_owned(),
            cache: None,
            branch_sim: false,
            is_aslr_enabled: false,
            cleanup_files: true,
            parallelism: 1,
            collect_bus: false,
            collect_atstart: false,
        }
    }
}

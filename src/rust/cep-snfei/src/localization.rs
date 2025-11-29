/// Localization Functor - Jurisdiction-Specific Transforms.
///
/// The Localization Functor L transforms raw entity names using
/// jurisdiction-specific rules before universal normalization:
///
/// ```markdown
///     L: RawLocal → IntermediateCanonical
///     N: IntermediateCanonical → FinalCanonical
///
///     SNFEI = Hash(N(L(raw_data)))
/// ```
///
/// Dependencies required for the localization logic
// Required for static initialization of the global config map
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::ErrorKind;
use std::path::PathBuf;
use std::sync::Mutex;

// =============================================================================
// DATA STRUCTURES
// =============================================================================

/// Represents a single localization transformation rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalizationRule {
    pub pattern: String,
    pub replacement: String,
    pub is_regex: bool,
    // Context is used to restrict the rule application (e.g., "name", "address")
    pub context: Option<String>,
}

impl Default for LocalizationRule {
    fn default() -> Self {
        LocalizationRule {
            pattern: String::new(),
            replacement: String::new(),
            is_regex: false,
            context: None,
        }
    }
}

/// Configuration loaded for a specific jurisdiction, potentially merged from parents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalizationConfig {
    pub jurisdiction: String,
    pub parent: Option<String>,
    // Maps abbreviation -> full form
    pub abbreviations: HashMap<String, String>,
    // Maps specific name -> canonical name
    pub agency_names: HashMap<String, String>,
    // Maps raw type name -> canonical type code
    pub entity_types: HashMap<String, String>,
    pub rules: Vec<LocalizationRule>,
    pub stop_words: HashSet<String>,
}

impl Default for LocalizationConfig {
    fn default() -> Self {
        LocalizationConfig {
            jurisdiction: "unknown".to_string(),
            parent: None,
            abbreviations: HashMap::new(),
            agency_names: HashMap::new(),
            entity_types: HashMap::new(),
            rules: Vec::new(),
            stop_words: HashSet::new(),
        }
    }
}

// =============================================================================
// BUILT-IN CONFIGURATION (Emulating Python's BUILT_IN_CONFIGS constants)
// =============================================================================

/// Helper to convert array of pairs where elements implement AsRef<str> to HashMap<String, String>.
/// This resolves the E0277 trait bound errors encountered with the previous generic bounds.
fn map_from_pairs<T: AsRef<str>, U: AsRef<str>>(pairs: &[(T, U)]) -> HashMap<String, String> {
    pairs
        .iter()
        // Use as_ref().to_string() to convert the referenced string-like type to an owned String.
        .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string()))
        .collect()
}

fn create_us_base_config() -> LocalizationConfig {
    LocalizationConfig {
        jurisdiction: "US".to_string(),
        parent: None,
        abbreviations: map_from_pairs(&[
            ("doj", "department of justice"),
            ("dod", "department of defense"),
            ("hhs", "department of health and human services"),
            ("hud", "department of housing and urban development"),
            ("doe", "department of energy"),
            ("ed", "department of education"),
            ("dot", "department of transportation"),
            ("dhs", "department of homeland security"),
            ("usda", "united states department of agriculture"),
            ("epa", "environmental protection agency"),
            ("fda", "food and drug administration"),
            ("fcc", "federal communications commission"),
            ("ftc", "federal trade commission"),
            ("sec", "securities and exchange commission"),
            ("irs", "internal revenue service"),
            ("ssa", "social security administration"),
            ("va", "veterans administration"),
            ("nasa", "national aeronautics and space administration"),
            ("fbi", "federal bureau of investigation"),
            ("cia", "central intelligence agency"),
            ("nsa", "national security agency"),
        ]),
        entity_types: map_from_pairs(&[
            ("k-12", "kindergarten through twelfth grade"),
            ("k12", "kindergarten through twelfth grade"),
            ("501c3", "five oh one c three"),
            ("501(c)(3)", "five oh one c three"),
            ("501c4", "five oh one c four"),
            ("501(c)(4)", "five oh one c four"),
        ]),
        // Python's US_BASE_CONFIG didn't include stop_words, so we omit them here to match the source.
        ..Default::default()
    }
}

// Global static map holding the built-in configurations
lazy_static! {
    // NOTE: This map stores ownership (LocalizationConfig) to be cloned into the registry cache.
    pub static ref BUILT_IN_CONFIGS: HashMap<&'static str, LocalizationConfig> = {
        let mut m = HashMap::new();

        let us_base = create_us_base_config();
        m.insert("us", us_base.clone());

        m.insert("us/ca", LocalizationConfig {
            jurisdiction: "US/CA".to_string(),
            parent: Some("US".to_string()),
            abbreviations: map_from_pairs(&[
                ("caltrans", "california department of transportation"),
                ("calpers", "california public employees retirement system"),
                ("calstrs", "california state teachers retirement system"),
                ("uc", "university of california"),
                ("csu", "california state university"),
                ("lausd", "los angeles unified school district"),
                ("sfusd", "san francisco unified school district"),
                ("ousd", "oakland unified school district"),
            ]),
            agency_names: map_from_pairs(&[
                ("dmv", "department of motor vehicles"),
                ("edd", "employment development department"),
                ("ftb", "franchise tax board"),
                ("boe", "board of equalization"),
                ("cdcr", "california department of corrections and rehabilitation"),
                ("chp", "california highway patrol"),
            ]),
            ..Default::default()
        });

        m.insert("us/ny", LocalizationConfig {
            jurisdiction: "US/NY".to_string(),
            parent: Some("US".to_string()),
            abbreviations: map_from_pairs(&[
                ("mta", "metropolitan transportation authority"),
                ("nycha", "new york city housing authority"),
                ("nypd", "new york police department"),
                ("fdny", "fire department new york"),
                ("suny", "state university of new york"),
                ("cuny", "city university of new york"),
                ("dot", "department of transportation"), // NY DOT
                ("dec", "department of environmental conservation"),
            ]),
            agency_names: map_from_pairs(&[
                ("port authority", "port authority of new york and new jersey"),
                ("thruway", "new york state thruway authority"),
                ("lirr", "long island rail road"),
            ]),
            ..Default::default()
        });

        let ca_base = LocalizationConfig {
            jurisdiction: "CA".to_string(),
            parent: None,
            abbreviations: map_from_pairs(&[
                ("rcmp", "royal canadian mounted police"),
                ("cra", "canada revenue agency"),
                ("cbsa", "canada border services agency"),
            ]),
            entity_types: map_from_pairs(&[
                ("ltée", "limitee"), ("ltee", "limitee"),
                ("inc", "incorporated"), ("enr", "enregistree"),
            ]),
            ..Default::default()
        };
        m.insert("ca", ca_base.clone());

        m.insert("ca/on", LocalizationConfig {
            jurisdiction: "CA/ON".to_string(),
            parent: Some("CA".to_string()),
            abbreviations: map_from_pairs(&[
                ("ttc", "toronto transit commission"),
                ("omb", "ontario municipal board"),
                ("wsib", "workplace safety and insurance board"),
                ("lcbo", "liquor control board of ontario"),
            ]),
            ..Default::default()
        });

        m.insert("ca/qc", LocalizationConfig {
            jurisdiction: "CA/QC".to_string(),
            parent: Some("CA".to_string()),
            abbreviations: map_from_pairs(&[
                ("stm", "societe de transport de montreal"),
                ("saaq", "societe de assurance automobile du quebec"),
                ("hydro-quebec", "hydro quebec"),
            ]),
            entity_types: map_from_pairs(&[
                ("limitée", "limitee"),
                ("incorporée", "incorporated"),
                ("société", "societe"),
                ("compagnie", "company"),
            ]),
            rules: vec![
                LocalizationRule { pattern: "é".to_string(), replacement: "e".to_string(), ..Default::default() },
                LocalizationRule { pattern: "è".to_string(), replacement: "e".to_string(), ..Default::default() },
                LocalizationRule { pattern: "ê".to_string(), replacement: "e".to_string(), ..Default::default() },
                LocalizationRule { pattern: "à".to_string(), replacement: "a".to_string(), ..Default::default() },
                LocalizationRule { pattern: "â".to_string(), replacement: "a".to_string(), ..Default::default() },
                LocalizationRule { pattern: "ô".to_string(), replacement: "o".to_string(), ..Default::default() },
                LocalizationRule { pattern: "û".to_string(), replacement: "u".to_string(), ..Default::default() },
                LocalizationRule { pattern: "ç".to_string(), replacement: "c".to_string(), ..Default::default() },
            ],
            ..Default::default()
        });
        m
    };
}

// =============================================================================
// LOCALIZATION FUNCTOR IMPLEMENTATION
// =============================================================================

impl LocalizationConfig {
    /// Apply jurisdiction-specific transformations to a name.
    /// Matches the Python implementation `apply_to_name`.
    pub fn apply_to_name(&self, name: &str) -> String {
        let mut result = name.to_lowercase();

        // 1. Agency name expansions (Word boundary matching)
        for (abbrev, full) in &self.agency_names {
            // Python: pattern = r"\b" + re.escape(abbrev.lower()) + r"\b"
            // Since `abbrev` is already lowercased in the static map definition, we can use it directly.
            let escaped_abbrev = regex::escape(abbrev);
            let pattern = format!(r"\b{}\b", escaped_abbrev);

            // Note: In production, these Regex objects should be compiled once, outside the loop.
            if let Ok(re) = Regex::new(&pattern) {
                result = re.replace_all(&result, full.as_str()).to_string();
            }
        }

        // 2. Abbreviations (Tokenization)
        let tokens: Vec<&str> = result.split_whitespace().collect();
        let expanded: Vec<String> = tokens
            .iter()
            .map(|token| {
                // Look up in the map (keys are already lowercase)
                if let Some(expanded_form) = self.abbreviations.get(*token) {
                    expanded_form.clone() // Value is already lowercased in the map
                } else {
                    token.to_string()
                }
            })
            .collect();
        result = expanded.join(" ");

        // 3. Entity types (Word boundary matching)
        for (local_type, canonical_type) in &self.entity_types {
            // Python: pattern = r"\b" + re.escape(local_type.lower()) + r"\b"
            let escaped_type = regex::escape(local_type);
            let pattern = format!(r"\b{}\b", escaped_type);

            if let Ok(re) = Regex::new(&pattern) {
                result = re.replace_all(&result, canonical_type.as_str()).to_string();
            }
        }

        // 4. Custom rules
        for rule in &self.rules {
            if rule.is_regex {
                // Python uses re.IGNORECASE, but input is already lowercased.
                if let Ok(re) = Regex::new(&rule.pattern) {
                    // Replacement is already lowercased in the rule definition
                    result = re
                        .replace_all(&result, rule.replacement.as_str())
                        .to_string();
                }
            } else {
                // Simple replace (pattern and replacement are lowercased)
                result = result.replace(&rule.pattern, &rule.replacement);
            }
        }

        result
    }
}

// =============================================================================
// LOCALIZATION REGISTRY (Matching Python Logic)
// =============================================================================

/// Registry for loading and caching localization configurations.
#[derive(Debug)]
pub struct LocalizationRegistry {
    /// Optional path to localization YAML files
    pub config_dir: Option<PathBuf>,
    /// Cache of loaded configs, including merged ones
    cache: HashMap<String, LocalizationConfig>,
}

impl LocalizationRegistry {
    /// Initialize the registry, pre-populating the cache with built-in configurations.
    pub fn new(config_dir: Option<PathBuf>) -> Self {
        // Initialize cache with a clone of the static BUILT_IN_CONFIGS
        let cache = BUILT_IN_CONFIGS
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();

        Self { config_dir, cache }
    }

    /// Load config from YAML file. Simulates Python's `_load_yaml`.
    fn load_yaml(&mut self, jurisdiction: &str) -> Option<LocalizationConfig> {
        if self.config_dir.is_none() {
            return None;
        }

        // --- Python-matching logic for path derivation ---
        let config_path = {
            // For a real implementation, we would need to safely unwrap or handle the PathBuf
            let config_dir = self.config_dir.as_ref().unwrap();

            if jurisdiction.contains('/') {
                // Region-level: us/ca -> us/ca.yaml
                let parts: Vec<&str> = jurisdiction.split('/').collect();
                if parts.len() == 2 {
                    config_dir.join(parts[0]).join(format!("{}.yaml", parts[1]))
                } else {
                    return None;
                }
            } else {
                // Country-level: us -> us/base.yaml
                config_dir.join(jurisdiction).join("base.yaml")
            }
        };

        // If the file exists, we would attempt to load and parse the YAML here.
        // Rust's File::open() and yaml::from_str() would be used.
        // We simulate the logic by checking existence and logging failure.

        // Simulating the file existence check and logging failure
        // NOTE: PathBuf::exists() might not work correctly in all environments,
        // but we keep the logic to match the Python intent.
        if config_path.exists() {
            // Placeholder: Assume successful load and parsing.
            let loaded_config = LocalizationConfig {
                jurisdiction: jurisdiction.to_string(),
                parent: if jurisdiction.contains('/') {
                    jurisdiction.split('/').next().map(|s| s.to_string())
                } else {
                    None
                },
                // In a real impl, this would be populated from the file.
                ..Default::default()
            };
            self.cache
                .insert(jurisdiction.to_string(), loaded_config.clone());
            Some(loaded_config)
        } else {
            // Simulating "Warning: Failed to load localization YAML..." log
            // We use PathBuf::to_string_lossy() to match Python's f-string printing behavior
            println!(
                "Warning: Failed to load localization YAML {}: {}",
                config_path.to_string_lossy(),
                ErrorKind::NotFound
            );
            None
        }
    }

    /// Merge child config with parent (child overrides parent). Matches Python's `merge_configs`.
    fn merge_configs(
        child: &LocalizationConfig,
        parent: &LocalizationConfig,
    ) -> LocalizationConfig {
        // 1-3. Maps: Child overrides parent (extend)
        let mut merged_abbrevs = parent.abbreviations.clone();
        merged_abbrevs.extend(child.abbreviations.clone());

        let mut merged_agencies = parent.agency_names.clone();
        merged_agencies.extend(child.agency_names.clone());

        let mut merged_types = parent.entity_types.clone();
        merged_types.extend(child.entity_types.clone());

        // 4. Rules: Parent rules come first, then child rules (concatenate)
        let mut merged_rules = parent.rules.clone();
        merged_rules.extend(child.rules.clone());

        // 5. Stop Words: Union of both sets
        let merged_stop_words = parent
            .stop_words
            .union(&child.stop_words)
            .cloned()
            .collect();

        LocalizationConfig {
            jurisdiction: child.jurisdiction.clone(),
            // Use parent's jurisdiction as the parent identifier (Python implementation detail)
            parent: Some(parent.jurisdiction.clone()),
            abbreviations: merged_abbrevs,
            agency_names: merged_agencies,
            entity_types: merged_types,
            rules: merged_rules,
            stop_words: merged_stop_words,
        }
    }

    /// Get localization config for a jurisdiction, with merging and fallback.
    /// Matches Python's `get_config`. Requires `&mut self` because the cache is mutated.
    pub fn get_config(&mut self, jurisdiction: &str) -> LocalizationConfig {
        // 1. Normalize to lowercase
        let jurisdiction = jurisdiction.to_lowercase();

        // 2. Check if we have a merged config cached
        let cache_key = format!("_merged_{}", jurisdiction);
        if self.cache.contains_key(&cache_key) {
            // MUST clone since the Python version returns a new object
            return self.cache.get(&cache_key).unwrap().clone();
        }

        // 3. Try to get the base config for this jurisdiction
        let config = if self.cache.contains_key(&jurisdiction) {
            // Found in built-in or previously loaded base cache
            Some(self.cache.get(&jurisdiction).unwrap().clone())
        } else if self.config_dir.is_some() {
            // Attempt to load from YAML file (calls self.load_yaml)
            self.load_yaml(&jurisdiction)
        } else {
            // Cannot load and not in cache
            None
        };

        let config = match config {
            Some(c) => c,
            None => {
                // 4. If no config found, fall back to parent
                if jurisdiction.contains('/') {
                    // Python uses rsplit("/", 1)[0] to get the parent code (e.g., "us/ca" -> "us")
                    let parent = jurisdiction
                        .rsplit_once('/')
                        .map(|(parent, _)| parent)
                        .unwrap();
                    return self.get_config(parent); // Recursive call
                }
                // 5. Return empty config as last resort
                return LocalizationConfig {
                    jurisdiction: jurisdiction,
                    parent: None,
                    ..Default::default()
                };
            }
        };

        // 6. If config has a parent, merge with parent config
        if let Some(parent_code) = &config.parent.clone() {
            // Clone parent_code to own it
            // Note: The Python code expects the parent code to be title-cased in the config
            // struct but lowercased for the registry lookup.
            // We use the parent code from the config structure directly for the next lookup.
            let parent_config = self.get_config(parent_code);
            let merged = Self::merge_configs(&config, &parent_config);

            // Cache the merged result
            self.cache.insert(cache_key, merged.clone());
            return merged;
        }

        // 7. If no parent, return the base config
        config
    }
}

// =============================================================================
// GLOBAL REGISTRY AND CONVENIENCE FUNCTIONS
// =============================================================================

// Mock implementation of _find_localization_dir, as filesystem access is outside this scope
fn find_localization_dir() -> Option<PathBuf> {
    // In a real Rust project, this would search for the 'localization' folder.
    None
}

// Global registry instance (mutable via get_config, similar to Python's class instance)
lazy_static! {
    static ref GLOBAL_REGISTRY: Mutex<LocalizationRegistry> = {
        let localization_dir = find_localization_dir();
        Mutex::new(LocalizationRegistry::new(localization_dir))
    };
}

/// Get localization config for a jurisdiction (convenience function).
pub fn get_localization_config(jurisdiction: &str) -> LocalizationConfig {
    // Must lock the mutex because the internal cache is mutated
    GLOBAL_REGISTRY.lock().unwrap().get_config(jurisdiction)
}

/// Apply localization transforms to a name.
/// This is the Localization Functor L. Matches Python's `apply_localization`.
pub fn apply_localization(name: &str, jurisdiction: &str) -> String {
    let config = get_localization_config(jurisdiction);
    config.apply_to_name(name)
}

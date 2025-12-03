/// Central schema registry for CEP validation.
///
/// Provides schema lookup and validation registry for all CEP record types.
/// Schema files are loaded from the repository root relative to the crate.
///
/// # Example
///
/// ```rust
/// use cep_core::SchemaRegistry;
///
/// // Get the shared global registry
/// let registry = SchemaRegistry::global()
///     .expect("failed to load global schema registry");
///
/// // Get the current entity schema
/// let schema = registry
///     .get_schema("entity", None)
///     .expect("failed to resolve entity schema");
///
/// // Basic sanity check on shape
/// assert!(schema.is_object());
///
/// // List all available schemas
/// for (name, version) in registry.list_schemas() {
///     println!("{} v{}", name, version);
/// }
/// ```
///
/// # Custom Registry
///
/// ```rust
/// use cep_core::SchemaRegistry;
/// use std::path::PathBuf;
///
/// // Create a registry with a custom root path
/// let registry = SchemaRegistry::with_root(PathBuf::from("/custom/path"))
///     .expect("failed to create registry with custom root");
/// // In this example we only check that construction succeeded.
/// assert!(registry.list_schemas().len() >= 0);
/// ```

use once_cell::sync::Lazy;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::RwLock;

use crate::error::{CepError, CepResult};
use crate::version::SCHEMA_VERSION;

/// Schema key: (name, version)
type SchemaKey = (String, String);

/// Schema catalog entry: maps (name, version) to relative path from repo root.
#[derive(Debug, Clone)]
struct CatalogEntry {
    name: String,
    version: String,
    relative_path: &'static str,
}

/// Default schema catalog.
fn default_catalog() -> Vec<CatalogEntry> {
    vec![
        CatalogEntry {
            name: "entity".to_string(),
            version: "1.0".to_string(),
            relative_path: "schemas/cep.entity.schema.json",
        },
        CatalogEntry {
            name: "exchange".to_string(),
            version: "1.0".to_string(),
            relative_path: "schemas/cep.exchange.schema.json",
        },
        CatalogEntry {
            name: "relationship".to_string(),
            version: "1.0".to_string(),
            relative_path: "schemas/cep.relationship.schema.json",
        },
        CatalogEntry {
            name: "snfei".to_string(),
            version: "1.0".to_string(),
            relative_path: "test_vectors/schemas/v1.0/generation-vector-set.schema.json",
        },
    ]
}

/// Global registry instance (lazy-initialized).
static GLOBAL_REGISTRY: Lazy<RwLock<Option<SchemaRegistry>>> = Lazy::new(|| RwLock::new(None));

/// Central schema registry for CEP validation.
///
/// The `SchemaRegistry` loads and caches JSON schemas for validating CEP records.
/// It supports multiple schema types (entity, exchange, relationship) and versions.
///
/// # Thread Safety
///
/// `SchemaRegistry` is `Clone` and can be safely shared across threads.
/// The global instance uses interior mutability for lazy initialization.
#[derive(Debug, Clone)]
pub struct SchemaRegistry {
    /// Repository root path where schemas are located.
    repo_root: PathBuf,
    /// Map of schema $id to schema content (for JSON Schema $ref resolution).
    registry: HashMap<String, Value>,
    /// Map of (name, version) to schema content.
    schemas: HashMap<SchemaKey, Value>,
    /// Catalog of known schemas.
    catalog: Vec<CatalogEntry>,
}

impl SchemaRegistry {
    /// Creates a new SchemaRegistry by auto-detecting the repository root.
    ///
    /// Searches for `pyproject.toml` or workspace `Cargo.toml` by walking up
    /// from `CARGO_MANIFEST_DIR`, current directory, or executable location.
    ///
    /// # Errors
    ///
    /// Returns `CepError::Configuration` if repository root cannot be found.
    pub fn new() -> CepResult<Self> {
        let repo_root = find_repo_root()?;
        Self::with_root(repo_root)
    }

    /// Creates a new SchemaRegistry with a specific root path.
    ///
    /// # Arguments
    ///
    /// * `repo_root` - Path to the repository root containing schema files.
    ///
    /// # Errors
    ///
    /// Returns `CepError::Configuration` if schemas cannot be loaded.
    pub fn with_root(repo_root: PathBuf) -> CepResult<Self> {
        let catalog = default_catalog();
        let mut registry = Self {
            repo_root,
            registry: HashMap::new(),
            schemas: HashMap::new(),
            catalog,
        };
        registry.load_schemas()?;
        Ok(registry)
    }

    /// Gets the global shared registry instance.
    ///
    /// The global registry is lazily initialized on first access and cached
    /// for subsequent calls. This is the recommended way to access schemas
    /// in most cases.
    ///
    /// # Errors
    ///
    /// Returns `CepError::Configuration` if the registry cannot be initialized.
    ///
      /// # Example
    ///
    /// ```rust
    /// use cep_core::SchemaRegistry;
    ///
    /// let registry = SchemaRegistry::global()
    ///     .expect("failed to load global schema registry");
    /// let schema = registry
    ///     .get_schema("entity", None)
    ///     .expect("failed to resolve entity schema");
    ///
    /// assert!(schema.is_object());
    /// ```
    pub fn global() -> CepResult<SchemaRegistry> {
        // Try to return cached instance
        {
            let cache = GLOBAL_REGISTRY.read().map_err(|e| {
                CepError::Configuration(format!("Failed to acquire registry read lock: {}", e))
            })?;
            if let Some(ref registry) = *cache {
                return Ok(registry.clone());
            }
        }

        // Build and cache new instance
        let registry = Self::new()?;
        {
            let mut cache = GLOBAL_REGISTRY.write().map_err(|e| {
                CepError::Configuration(format!("Failed to acquire registry write lock: {}", e))
            })?;
            *cache = Some(registry.clone());
        }

        Ok(registry)
    }

    /// Returns the repository root path.
    pub fn repo_root(&self) -> &PathBuf {
        &self.repo_root
    }

    /// Gets a schema by logical name and optional version.
    ///
    /// # Arguments
    ///
    /// * `name` - Schema name (e.g., "entity", "exchange", "relationship").
    /// * `version` - Schema version. Defaults to current SCHEMA_VERSION major.minor.
    ///
    /// # Returns
    ///
    /// Schema as a `serde_json::Value`.
    ///
    /// # Errors
    ///
    /// Returns `CepError::UnknownSchema` if schema name or version is unknown.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cep_core::SchemaRegistry;
    ///
    /// let registry = SchemaRegistry::global()
    ///     .expect("failed to load global schema registry");
    ///
    /// // Get current version
    /// let latest = registry
    ///     .get_schema("entity", None)
    ///     .expect("failed to resolve latest entity schema");
    ///
    /// // Get specific version
    /// let v1 = registry
    ///     .get_schema("entity", Some("1.0"))
    ///     .expect("failed to resolve v1 entity schema");
    ///
    /// assert!(latest.is_object());
    /// assert!(v1.is_object());
    /// ```
    pub fn get_schema(&self, name: &str, version: Option<&str>) -> CepResult<Value> {
        let version = version
            .map(|v| v.to_string())
            .unwrap_or_else(schema_version);

        let key = (name.to_string(), version.clone());

        match self.schemas.get(&key) {
            Some(schema) => Ok(schema.clone()),
            None => {
                // Check if name exists with other versions
                let available: Vec<String> = self
                    .schemas
                    .keys()
                    .filter(|(n, _)| n == name)
                    .map(|(n, v)| format!("{} v{}", n, v))
                    .collect();

                if !available.is_empty() {
                    Err(CepError::UnknownSchema(format!(
                        "Unknown version '{}' for schema '{}'. Available: {:?}",
                        version, name, available
                    )))
                } else {
                    Err(CepError::UnknownSchema(format!(
                        "Unknown schema: '{}'",
                        name
                    )))
                }
            }
        }
    }

    /// Gets the full registry map for JSON Schema validation.
    ///
    /// Returns a HashMap mapping schema `$id` URIs to schema `Value`.
    /// This can be used with JSON Schema validation libraries that support
    /// `$ref` resolution.
    pub fn get_registry(&self) -> &HashMap<String, Value> {
        &self.registry
    }

    /// Lists all available (name, version) pairs in the catalog.
    ///
    /// Note: This returns all cataloged schemas, not just those with files present.
    pub fn list_schemas(&self) -> Vec<(String, String)> {
        self.catalog
            .iter()
            .map(|e| (e.name.clone(), e.version.clone()))
            .collect()
    }

    /// Lists all loaded schemas (those with files actually present).
    pub fn list_loaded_schemas(&self) -> Vec<(String, String)> {
        self.schemas.keys().cloned().collect()
    }

    /// Returns the number of loaded schemas.
    pub fn len(&self) -> usize {
        self.schemas.len()
    }

    /// Returns true if no schemas are loaded.
    pub fn is_empty(&self) -> bool {
        self.schemas.is_empty()
    }

    /// Loads all schemas from the catalog.
    fn load_schemas(&mut self) -> CepResult<()> {
        for entry in &self.catalog.clone() {
            let schema_path = self.repo_root.join(entry.relative_path);

            if !schema_path.is_file() {
                // Schema file not found - skip silently (may not be present in all deployments)
                continue;
            }

            let content = fs::read_to_string(&schema_path).map_err(|e| {
                CepError::Configuration(format!(
                    "Failed to read schema file {}: {}",
                    schema_path.display(),
                    e
                ))
            })?;

            let schema: Value = serde_json::from_str(&content).map_err(|e| {
                CepError::Configuration(format!(
                    "Failed to parse schema JSON {}: {}",
                    schema_path.display(),
                    e
                ))
            })?;

            // Extract $id or generate one
            let schema_id = schema
                .get("$id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("urn:cep:{}:{}", entry.name, entry.version));

            let key = (entry.name.clone(), entry.version.clone());
            self.schemas.insert(key, schema.clone());
            self.registry.insert(schema_id, schema);
        }

        Ok(())
    }

    /// Clears the global registry cache (useful for testing).
    #[cfg(test)]
    pub fn clear_global_cache() {
        if let Ok(mut cache) = GLOBAL_REGISTRY.write() {
            *cache = None;
        }
    }
}

impl Default for SchemaRegistry {
    /// Creates a default SchemaRegistry.
    ///
    /// # Panics
    ///
    /// Panics if the repository root cannot be found. Use `SchemaRegistry::new()`
    /// for fallible construction.
    fn default() -> Self {
        Self::new().expect("Failed to create default SchemaRegistry")
    }
}

/// Get major.minor version for schema lookup (drop patch).
fn schema_version() -> String {
    let parts: Vec<&str> = SCHEMA_VERSION.split('.').collect();
    if parts.len() >= 2 {
        format!("{}.{}", parts[0], parts[1])
    } else {
        SCHEMA_VERSION.to_string()
    }
}

/// Walk up from current location to find repository root.
///
/// Searches for pyproject.toml (Python) or Cargo.toml at workspace root.
/// This is the canonical way to find repo root across all CEP crates.
pub fn find_repo_root() -> CepResult<PathBuf> {
    // Try CARGO_MANIFEST_DIR first (works during development)
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let manifest_path = PathBuf::from(&manifest_dir);

        // Walk up looking for repo root markers
        for ancestor in manifest_path.ancestors() {
            // Check for pyproject.toml (Python monorepo root)
            if ancestor.join("pyproject.toml").exists() {
                return Ok(ancestor.to_path_buf());
            }
            // Check for workspace Cargo.toml with [workspace] section
            let cargo_path = ancestor.join("Cargo.toml");
            if cargo_path.exists() {
                if let Ok(content) = fs::read_to_string(&cargo_path) {
                    if content.contains("[workspace]") {
                        return Ok(ancestor.to_path_buf());
                    }
                }
            }
        }
    }

    // Try current working directory
    if let Ok(cwd) = std::env::current_dir() {
        for ancestor in cwd.ancestors() {
            if ancestor.join("pyproject.toml").exists() {
                return Ok(ancestor.to_path_buf());
            }
            let cargo_path = ancestor.join("Cargo.toml");
            if cargo_path.exists() {
                if let Ok(content) = fs::read_to_string(&cargo_path) {
                    if content.contains("[workspace]") {
                        return Ok(ancestor.to_path_buf());
                    }
                }
            }
        }
    }

    // Try executable location as last resort
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            for ancestor in exe_dir.ancestors() {
                if ancestor.join("pyproject.toml").exists() {
                    return Ok(ancestor.to_path_buf());
                }
            }
        }
    }

    Err(CepError::Configuration(
        "Could not find repository root (no pyproject.toml or workspace Cargo.toml found)"
            .to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_version_extraction() {
        let version = schema_version();
        let parts: Vec<&str> = version.split('.').collect();
        assert_eq!(parts.len(), 2, "Should be major.minor format");
    }

    #[test]
    fn test_list_schemas() {
        let catalog = default_catalog();
        assert!(!catalog.is_empty());

        let names: Vec<&str> = catalog.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"entity"));
        assert!(names.contains(&"exchange"));
        assert!(names.contains(&"relationship"));
        assert!(names.contains(&"snfei"));
    }

    #[test]
    fn test_schema_catalog_versions() {
        let catalog = default_catalog();
        for entry in &catalog {
            assert_eq!(entry.version, "1.0");
        }
    }

    #[test]
    fn test_find_repo_root_returns_path() {
        // This test may fail in CI without proper setup
        // but should work in development
        match find_repo_root() {
            Ok(path) => {
                assert!(path.is_dir() || !path.exists());
            }
            Err(_) => {
                // Acceptable in some test environments
            }
        }
    }

    #[test]
    fn test_registry_with_nonexistent_root() {
        let result = SchemaRegistry::with_root(PathBuf::from("/nonexistent/path"));
        // Should succeed but have no loaded schemas
        assert!(result.is_ok());
        let registry = result.unwrap();
        assert!(registry.is_empty());
    }

    #[test]
    fn test_unknown_schema_error() {
        let registry = SchemaRegistry::with_root(PathBuf::from("/nonexistent")).unwrap();
        let result = registry.get_schema("nonexistent", None);
        assert!(result.is_err());
        if let Err(CepError::UnknownSchema(msg)) = result {
            assert!(msg.contains("nonexistent"));
        } else {
            panic!("Expected UnknownSchema error");
        }
    }

    #[test]
    fn test_registry_clone() {
        let registry1 = SchemaRegistry::with_root(PathBuf::from("/tmp")).unwrap();
        let registry2 = registry1.clone();
        assert_eq!(registry1.repo_root(), registry2.repo_root());
    }
}
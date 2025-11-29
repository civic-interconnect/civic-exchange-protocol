/// Universal Normalization Pipeline.
///
/// The Normalizing Functor N transforms the category of Raw Entity Data
/// into the category of Canonical Entity Data:
///
/// ```markdown
///     N: RawEntity → CanonicalEntity
/// ```
/// Where N preserves identity (same entity always maps to same canonical form)
/// and composition (N(L(x)) = N ∘ L(x) where L is the localization functor).

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use unicode_normalization::UnicodeNormalization;

// =============================================================================
// LEGAL SUFFIX EXPANSIONS
// =============================================================================

lazy_static! {
    /// Legal entity suffixes: ALWAYS expand to full form
    static ref LEGAL_SUFFIX_EXPANSIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        // Corporations
        m.insert("inc", "incorporated");
        m.insert("inc.", "incorporated");
        m.insert("incorp", "incorporated");
        m.insert("corp", "corporation");
        m.insert("corp.", "corporation");
        m.insert("co", "company");
        m.insert("co.", "company");
        // Limited liability
        m.insert("llc", "limited liability company");
        m.insert("l.l.c.", "limited liability company");
        m.insert("l.l.c", "limited liability company");
        m.insert("pllc", "professional limited liability company");
        m.insert("p.l.l.c.", "professional limited liability company");
        // Limited
        m.insert("ltd", "limited");
        m.insert("ltd.", "limited");
        m.insert("ltda", "limitada");
        // Partnership
        m.insert("lp", "limited partnership");
        m.insert("l.p.", "limited partnership");
        m.insert("llp", "limited liability partnership");
        m.insert("l.l.p.", "limited liability partnership");
        // Professional
        m.insert("pc", "professional corporation");
        m.insert("p.c.", "professional corporation");
        m.insert("pa", "professional association");
        m.insert("p.a.", "professional association");
        // Other
        m.insert("plc", "public limited company");
        m.insert("p.l.c.", "public limited company");
        m.insert("na", "national association");
        m.insert("n.a.", "national association");
        m
    };

    /// Common abbreviations: ALWAYS expand
    static ref COMMON_ABBREVIATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        // Organizational
        m.insert("assn", "association");
        m.insert("assoc", "association");
        m.insert("dept", "department");
        m.insert("div", "division");
        m.insert("org", "organization");
        m.insert("inst", "institute");
        m.insert("ctr", "center");
        m.insert("cntr", "center");
        m.insert("comm", "commission");
        m.insert("cmte", "committee");
        m.insert("bd", "board");
        m.insert("auth", "authority");
        m.insert("agcy", "agency");
        m.insert("admin", "administration");
        m.insert("corp", "corporation");
        m.insert("svcs", "services");
        m.insert("svc", "service");
        // Educational
        m.insert("sch", "school");
        m.insert("schl", "school");
        m.insert("dist", "district");
        m.insert("usd", "unified school district");
        m.insert("isd", "independent school district");
        m.insert("elem", "elementary");
        m.insert("univ", "university");
        m.insert("coll", "college");
        m.insert("acad", "academy");
        // Geographic/Government
        m.insert("natl", "national");
        m.insert("nat", "national");
        m.insert("intl", "international");
        m.insert("int", "international");
        m.insert("fed", "federal");
        m.insert("govt", "government");
        m.insert("gov", "government");
        m.insert("muni", "municipal");
        m.insert("metro", "metropolitan");
        m.insert("reg", "regional");
        m.insert("cty", "county");
        m.insert("twp", "township");
        m.insert("vlg", "village");
        m.insert("boro", "borough");
        // Directional (for addresses mostly, but sometimes in names)
        m.insert("st", "saint");
        m.insert("mt", "mount");
        m.insert("ft", "fort");
        m.insert("pt", "point");
        // Medical/Health
        m.insert("hosp", "hospital");
        m.insert("med", "medical");
        m.insert("hlth", "health");
        m.insert("pharm", "pharmaceutical");
        m
    };

    /// Stop words to remove (after normalization)
    static ref STOP_WORDS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        s.insert("the");
        s.insert("of");
        s.insert("a");
        s.insert("an");
        s.insert("and");
        s.insert("for");
        s.insert("in");
        s.insert("on");
        s.insert("at");
        s.insert("to");
        s.insert("by");
        s
    };

    /// US Postal abbreviations for addresses
    static ref US_ADDRESS_EXPANSIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        // Street types
        m.insert("st", "street");
        m.insert("st.", "street");
        m.insert("ave", "avenue");
        m.insert("ave.", "avenue");
        m.insert("blvd", "boulevard");
        m.insert("blvd.", "boulevard");
        m.insert("dr", "drive");
        m.insert("dr.", "drive");
        m.insert("rd", "road");
        m.insert("rd.", "road");
        m.insert("ln", "lane");
        m.insert("ln.", "lane");
        m.insert("ct", "court");
        m.insert("ct.", "court");
        m.insert("pl", "place");
        m.insert("pl.", "place");
        m.insert("cir", "circle");
        m.insert("cir.", "circle");
        m.insert("pkwy", "parkway");
        m.insert("hwy", "highway");
        m.insert("expy", "expressway");
        m.insert("trl", "trail");
        m.insert("way", "way");
        // Directional
        m.insert("n", "north");
        m.insert("n.", "north");
        m.insert("s", "south");
        m.insert("s.", "south");
        m.insert("e", "east");
        m.insert("e.", "east");
        m.insert("w", "west");
        m.insert("w.", "west");
        m.insert("ne", "northeast");
        m.insert("nw", "northwest");
        m.insert("se", "southeast");
        m.insert("sw", "southwest");
        m
    };

    /// Regex patterns for secondary unit designators
    static ref SECONDARY_UNIT_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"(?i)\b(suite|ste|ste\.)\s*#?\s*\w+").unwrap(),
        Regex::new(r"(?i)\b(apartment|apt|apt\.)\s*#?\s*\w+").unwrap(),
        Regex::new(r"(?i)\b(unit)\s*#?\s*\w+").unwrap(),
        Regex::new(r"(?i)\b(floor|flr|fl)\s*#?\s*\d+").unwrap(),
        Regex::new(r"(?i)\b(room|rm)\s*#?\s*\w+").unwrap(),
        Regex::new(r"(?i)\b(building|bldg)\s*#?\s*\w+").unwrap(),
        Regex::new(r"#\s*\d+\s*$").unwrap(),
    ];

    /// Date parsing patterns
    static ref DATE_PATTERNS: Vec<(Regex, &'static str)> = vec![
        (Regex::new(r"^(\d{4})-(\d{2})-(\d{2})$").unwrap(), "ISO"),
        (Regex::new(r"^(\d{1,2})/(\d{1,2})/(\d{4})$").unwrap(), "US"),
        (Regex::new(r"^(\d{1,2})-(\d{1,2})-(\d{4})$").unwrap(), "US_DASH"),
        (Regex::new(r"^(\d{4})$").unwrap(), "YEAR"),
    ];
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/// Convert Unicode to ASCII equivalent.
fn to_ascii(text: &str) -> String {
    // First, apply NFD normalization to decompose characters
    let normalized: String = text.nfkd().collect();
    
    // Then filter out combining marks and convert special chars
    let mut result = String::with_capacity(normalized.len());
    for c in normalized.chars() {
        match c {
            // Keep ASCII alphanumeric and space
            c if c.is_ascii_alphanumeric() || c == ' ' => result.push(c),
            // Common replacements
            'ß' => result.push_str("ss"),
            'æ' | 'Æ' => result.push_str("ae"),
            'œ' | 'Œ' => result.push_str("oe"),
            'ø' | 'Ø' => result.push('o'),
            'ð' | 'Ð' => result.push('d'),
            'þ' | 'Þ' => result.push_str("th"),
            // Skip combining marks (diacritics)
            c if c.is_ascii() => result.push(c),
            c if !c.is_alphanumeric() => {} // Skip non-alphanumeric Unicode
            _ => {} // Skip combining marks from NFD
        }
    }
    result
}

/// Remove all punctuation from text.
fn remove_punctuation(text: &str) -> String {
    text.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect()
}

/// Collapse multiple whitespace to single space and trim.
fn collapse_whitespace(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut prev_space = true; // Start true to trim leading
    
    for c in text.chars() {
        if c.is_whitespace() {
            if !prev_space {
                result.push(' ');
                prev_space = true;
            }
        } else {
            result.push(c);
            prev_space = false;
        }
    }
    
    // Trim trailing space
    if result.ends_with(' ') {
        result.pop();
    }
    result
}

/// Expand a single token if it matches known abbreviations.
fn expand_token(token: &str) -> String {
    let lower = token.to_lowercase();
    
    // Check legal suffixes first (highest priority)
    if let Some(&expansion) = LEGAL_SUFFIX_EXPANSIONS.get(lower.as_str()) {
        return expansion.to_string();
    }
    
    // Check common abbreviations
    if let Some(&expansion) = COMMON_ABBREVIATIONS.get(lower.as_str()) {
        return expansion.to_string();
    }
    
    lower
}

/// Expand all abbreviations in text.
fn expand_abbreviations(text: &str) -> String {
    text.split_whitespace()
        .map(expand_token)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Remove stop words from text.
fn remove_stop_words(text: &str, preserve_initial: bool) -> String {
    let tokens: Vec<&str> = text.split_whitespace().collect();
    if tokens.is_empty() {
        return String::new();
    }
    
    let mut result = Vec::with_capacity(tokens.len());
    for (i, token) in tokens.iter().enumerate() {
        // Optionally preserve first word even if it's a stop word
        if i == 0 && preserve_initial && STOP_WORDS.contains(token) {
            // Skip it anyway in v1.1 (changed from v1.0)
            continue;
        }
        if !STOP_WORDS.contains(token) {
            result.push(*token);
        }
    }
    
    result.join(" ")
}

// =============================================================================
// PUBLIC NORMALIZATION FUNCTIONS
// =============================================================================

/// Apply the universal normalization pipeline to a legal name.
///
/// Pipeline (in order):
/// 1. Convert to lowercase
/// 2. ASCII transliteration (é→e, ñ→n, etc.)
/// 3. Remove punctuation (ALL punctuation including hyphens)
/// 4. Collapse whitespace
/// 5. Expand abbreviations (inc→incorporated, usd→unified school district)
/// 6. Remove stop words (the, of, a, an, and, for, in, on, at, to, by)
/// 7. Final trim
///
/// # Arguments
/// * `name` - Raw legal name from source system
///
/// # Returns
/// Normalized name suitable for SNFEI hashing
///
/// # Example
/// ```
/// use cep_snfei::normalize_legal_name;
/// 
/// let normalized = normalize_legal_name("The Springfield Unified Sch. Dist., Inc.");
/// assert_eq!(normalized, "springfield unified school district incorporated");
/// ```
pub fn normalize_legal_name(name: &str) -> String {
    if name.is_empty() {
        return String::new();
    }
    
    // 1. Lowercase
    let text = name.to_lowercase();
    
    // 2. ASCII transliteration
    let text = to_ascii(&text);
    
    // 3. Remove punctuation
    let text = remove_punctuation(&text);
    
    // 4. Collapse whitespace
    let text = collapse_whitespace(&text);
    
    // 5. Expand abbreviations
    let text = expand_abbreviations(&text);
    
    // 6. Remove stop words
    let text = remove_stop_words(&text, false);
    
    // 7. Final collapse and trim
    collapse_whitespace(&text)
}

/// Normalize a street address for SNFEI hashing.
///
/// Pipeline:
/// 1. Lowercase
/// 2. ASCII transliteration
/// 3. Remove secondary unit designators (apt, suite, floor, etc.)
/// 4. Remove punctuation
/// 5. Expand postal abbreviations (st→street, ave→avenue, n→north)
/// 6. Collapse whitespace
///
/// # Arguments
/// * `address` - Raw street address
///
/// # Returns
/// Normalized address string
///
/// # Example
/// ```
/// use cep_snfei::normalize_address;
/// 
/// let normalized = normalize_address("123 N. Main St., Suite 400");
/// assert_eq!(normalized, "123 north main street");
/// ```
pub fn normalize_address(address: &str) -> String {
    if address.is_empty() {
        return String::new();
    }
    
    // 1. Lowercase
    let mut text = address.to_lowercase();
    
    // 2. ASCII transliteration
    text = to_ascii(&text);
    
    // 3. Remove secondary unit designators
    for pattern in SECONDARY_UNIT_PATTERNS.iter() {
        text = pattern.replace_all(&text, "").to_string();
    }
    
    // 4. Remove punctuation
    text = remove_punctuation(&text);
    
    // 5. Collapse whitespace first
    text = collapse_whitespace(&text);
    
    // 6. Expand postal abbreviations
    let tokens: Vec<String> = text
        .split_whitespace()
        .map(|t| {
            US_ADDRESS_EXPANSIONS
                .get(t)
                .map(|&s| s.to_string())
                .unwrap_or_else(|| t.to_string())
        })
        .collect();
    text = tokens.join(" ");
    
    // 7. Final trim
    text.trim().to_string()
}

/// Normalize a registration date to ISO 8601 format.
///
/// Returns None if date cannot be parsed.
///
/// # Arguments
/// * `date_str` - Date string in various formats
///
/// # Returns
/// ISO 8601 date string (YYYY-MM-DD) or None
///
/// # Example
/// ```
/// use cep_snfei::normalize_registration_date;
/// 
/// assert_eq!(normalize_registration_date("03/15/1990"), Some("1990-03-15".to_string()));
/// assert_eq!(normalize_registration_date("1985"), Some("1985-01-01".to_string()));
/// assert_eq!(normalize_registration_date(""), None);
/// ```
pub fn normalize_registration_date(date_str: &str) -> Option<String> {
    let date_str = date_str.trim();
    if date_str.is_empty() {
        return None;
    }
    
    for (pattern, fmt) in DATE_PATTERNS.iter() {
        if let Some(caps) = pattern.captures(date_str) {
            match *fmt {
                "ISO" => {
                    // Already in ISO format
                    return Some(date_str.to_string());
                }
                "US" | "US_DASH" => {
                    // MM/DD/YYYY or MM-DD-YYYY
                    let month: u32 = caps.get(1)?.as_str().parse().ok()?;
                    let day: u32 = caps.get(2)?.as_str().parse().ok()?;
                    let year: u32 = caps.get(3)?.as_str().parse().ok()?;
                    if month >= 1 && month <= 12 && day >= 1 && day <= 31 {
                        return Some(format!("{:04}-{:02}-{:02}", year, month, day));
                    }
                }
                "YEAR" => {
                    // Year only - default to Jan 1
                    let year: u32 = caps.get(1)?.as_str().parse().ok()?;
                    return Some(format!("{:04}-01-01", year));
                }
                _ => {}
            }
        }
    }
    
    None
}

// =============================================================================
// CANONICAL INPUT
// =============================================================================

/// Normalized input for SNFEI hashing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalInput {
    pub legal_name_normalized: String,
    pub address_normalized: Option<String>,
    pub country_code: String,
    pub registration_date: Option<String>,
}

impl CanonicalInput {
    /// Generate the concatenated string for hashing.
    ///
    /// Format:
    ///     legal_name_normalized|address_normalized|country_code|registration_date
    ///
    /// Empty/None fields are included as empty strings to maintain
    /// consistent field positions.
    pub fn to_hash_string(&self) -> String {
        let parts = [
            self.legal_name_normalized.as_str(),
            self.address_normalized.as_deref().unwrap_or(""),
            self.country_code.as_str(),
            self.registration_date.as_deref().unwrap_or(""),
        ];
        parts.join("|")
    }
}

/// Build a canonical input structure from raw entity data.
///
/// # Arguments
/// * `legal_name` - Raw legal name
/// * `country_code` - ISO 3166-1 alpha-2 country code
/// * `address` - Optional street address
/// * `registration_date` - Optional registration/formation date
///
/// # Returns
/// CanonicalInput with all fields normalized
pub fn build_canonical_input(
    legal_name: &str,
    country_code: &str,
    address: Option<&str>,
    registration_date: Option<&str>,
) -> CanonicalInput {
    CanonicalInput {
        legal_name_normalized: normalize_legal_name(legal_name),
        address_normalized: address.map(normalize_address).filter(|s| !s.is_empty()),
        country_code: country_code.to_uppercase(),
        registration_date: registration_date.and_then(normalize_registration_date),
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_legal_name_basic() {
        assert_eq!(
            normalize_legal_name("Springfield School District"),
            "springfield school district"
        );
    }

    #[test]
    fn test_normalize_legal_name_uppercase() {
        assert_eq!(
            normalize_legal_name("SPRINGFIELD SCHOOL DISTRICT"),
            "springfield school district"
        );
    }

    #[test]
    fn test_normalize_legal_name_abbreviations() {
        assert_eq!(
            normalize_legal_name("Springfield USD"),
            "springfield unified school district"
        );
    }

    #[test]
    fn test_normalize_legal_name_legal_suffix() {
        assert_eq!(
            normalize_legal_name("Acme Corp Inc"),
            "acme corporation incorporated"
        );
    }

    #[test]
    fn test_normalize_legal_name_llc() {
        assert_eq!(
            normalize_legal_name("Smith Consulting LLC"),
            "smith consulting limited liability company"
        );
    }

    #[test]
    fn test_normalize_legal_name_stop_words() {
        assert_eq!(
            normalize_legal_name("The Boston Foundation"),
            "boston foundation"
        );
    }

    #[test]
    fn test_normalize_legal_name_accents() {
        assert_eq!(
            normalize_legal_name("Société Générale"),
            "societe generale"
        );
    }

    #[test]
    fn test_normalize_legal_name_punctuation() {
        assert_eq!(
            normalize_legal_name("Acme Corp., Inc."),
            "acme corporation incorporated"
        );
    }

    #[test]
    fn test_normalize_legal_name_empty() {
        assert_eq!(normalize_legal_name(""), "");
    }

    #[test]
    fn test_normalize_address_basic() {
        assert_eq!(
            normalize_address("123 Main Street"),
            "123 main street"
        );
    }

    #[test]
    fn test_normalize_address_abbreviations() {
        assert_eq!(
            normalize_address("123 N. Main St."),
            "123 north main street"
        );
    }

    #[test]
    fn test_normalize_address_suite_removal() {
        assert_eq!(
            normalize_address("123 Main St, Suite 400"),
            "123 main street"
        );
    }

    #[test]
    fn test_normalize_address_apt_removal() {
        assert_eq!(
            normalize_address("456 Oak Ave, Apt 2B"),
            "456 oak avenue"
        );
    }

    #[test]
    fn test_normalize_registration_date_iso() {
        assert_eq!(
            normalize_registration_date("2024-03-15"),
            Some("2024-03-15".to_string())
        );
    }

    #[test]
    fn test_normalize_registration_date_us() {
        assert_eq!(
            normalize_registration_date("03/15/2024"),
            Some("2024-03-15".to_string())
        );
    }

    #[test]
    fn test_normalize_registration_date_year_only() {
        assert_eq!(
            normalize_registration_date("1985"),
            Some("1985-01-01".to_string())
        );
    }

    #[test]
    fn test_normalize_registration_date_empty() {
        assert_eq!(normalize_registration_date(""), None);
    }

    #[test]
    fn test_canonical_input_hash_string() {
        let input = CanonicalInput {
            legal_name_normalized: "springfield unified school district".to_string(),
            address_normalized: Some("123 main street".to_string()),
            country_code: "US".to_string(),
            registration_date: Some("1990-03-15".to_string()),
        };
        assert_eq!(
            input.to_hash_string(),
            "springfield unified school district|123 main street|US|1990-03-15"
        );
    }

    #[test]
    fn test_canonical_input_hash_string_minimal() {
        let input = CanonicalInput {
            legal_name_normalized: "acme corporation".to_string(),
            address_normalized: None,
            country_code: "US".to_string(),
            registration_date: None,
        };
        assert_eq!(input.to_hash_string(), "acme corporation||US|");
    }

    #[test]
    fn test_build_canonical_input() {
        let input = build_canonical_input(
            "Springfield USD #12",
            "US",
            Some("123 N. Main St."),
            Some("03/15/1990"),
        );
        assert_eq!(input.legal_name_normalized, "springfield unified school district 12");
        assert_eq!(input.address_normalized, Some("123 north main street".to_string()));
        assert_eq!(input.country_code, "US");
        assert_eq!(input.registration_date, Some("1990-03-15".to_string()));
    }
}
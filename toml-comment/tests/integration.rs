use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use toml_comment::TomlComment;
use toml_comment::TomlCommentDefault;

/// Application settings
#[derive(Serialize, TomlComment)]
struct AppConfig {
    /// The application name
    name: String,
    /// Whether debug mode is enabled
    debug: bool,
    /// Maximum retry count
    max_retries: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "myapp".to_string(),
            debug: false,
            max_retries: 3,
        }
    }
}

#[test]
fn basic_struct() {
    let toml = AppConfig::default_toml();
    let expected = "\
# Application settings
# The application name
name = \"myapp\"
# Whether debug mode is enabled
debug = false
# Maximum retry count
max_retries = 3
";
    assert_eq!(toml, expected);
}

#[derive(Serialize, TomlComment)]
struct ServerConfig {
    /// Port to listen on
    port: u16,
    /// Bind address
    host: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "127.0.0.1".to_string(),
        }
    }
}

/// Root config
#[derive(Serialize, TomlComment)]
struct RootConfig {
    server: ServerConfig,
}

impl Default for RootConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
        }
    }
}

#[test]
fn nested_struct() {
    let toml = RootConfig::default_toml();
    let expected = "\
# Root config

[server]
# Port to listen on
port = 8080
# Bind address
host = \"127.0.0.1\"
";
    assert_eq!(toml, expected);
}

#[derive(Serialize, TomlComment)]
struct WithOption {
    /// Always present
    name: String,
    /// Sometimes present
    extra: Option<String>,
}

impl Default for WithOption {
    fn default() -> Self {
        Self {
            name: "hello".to_string(),
            extra: None,
        }
    }
}

#[test]
fn option_none_omitted() {
    let toml = WithOption::default_toml();
    let expected = "\
# Always present
name = \"hello\"
";
    assert_eq!(toml, expected);
}

#[test]
fn option_some_included() {
    let cfg = WithOption {
        name: "hello".to_string(),
        extra: Some("world".to_string()),
    };
    let toml = cfg.to_commented_toml();
    let expected = "\
# Always present
name = \"hello\"
# Sometimes present
extra = \"world\"
";
    assert_eq!(toml, expected);
}

#[derive(Serialize, TomlComment)]
struct WithVec {
    /// Allowed origins
    origins: Vec<String>,
}

impl Default for WithVec {
    fn default() -> Self {
        Self {
            origins: vec!["localhost".to_string(), "example.com".to_string()],
        }
    }
}

#[test]
fn vec_field() {
    let toml = WithVec::default_toml();
    let expected = "\
# Allowed origins
origins = [\"localhost\", \"example.com\"]
";
    assert_eq!(toml, expected);
}

#[derive(Serialize, TomlComment)]
struct NoComments {
    name: String,
    count: u32,
}

impl Default for NoComments {
    fn default() -> Self {
        Self {
            name: "test".to_string(),
            count: 42,
        }
    }
}

#[test]
fn no_comment_fields() {
    let toml = NoComments::default_toml();
    let expected = "\
name = \"test\"
count = 42
";
    assert_eq!(toml, expected);
}

#[test]
fn non_default_values() {
    let cfg = AppConfig {
        name: "custom".to_string(),
        debug: true,
        max_retries: 10,
    };
    let toml = cfg.to_commented_toml();
    assert!(toml.contains("name = \"custom\""));
    assert!(toml.contains("debug = true"));
    assert!(toml.contains("max_retries = 10"));
}

#[derive(Serialize, TomlComment)]
struct LoggingConfig {
    /// Log level
    level: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
        }
    }
}

#[derive(Serialize, TomlComment)]
struct DatabaseConfig {
    /// Connection URL
    url: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite://data.db".to_string(),
        }
    }
}

/// Multi-section config
#[derive(Serialize, TomlComment)]
struct MultiSection {
    logging: LoggingConfig,
    database: DatabaseConfig,
}

impl Default for MultiSection {
    fn default() -> Self {
        Self {
            logging: LoggingConfig::default(),
            database: DatabaseConfig::default(),
        }
    }
}

#[test]
fn multiple_sections_separated_by_blank_line() {
    let toml = MultiSection::default_toml();
    let expected = "\
# Multi-section config

[logging]
# Log level
level = \"info\"

[database]
# Connection URL
url = \"sqlite://data.db\"
";
    assert_eq!(toml, expected);
}

#[derive(Serialize, TomlComment)]
struct Thresholds {
    /// Temperature threshold
    temperature: f64,
    /// Ratio value
    ratio: f64,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            temperature: 1.0,
            ratio: 0.75,
        }
    }
}

#[test]
fn float_formatting() {
    let toml = Thresholds::default_toml();
    assert!(toml.contains("temperature = 1.0"));
    assert!(toml.contains("ratio = 0.75"));
}



#[derive(Serialize, TomlComment)]
struct ThresholdsNoDefault {
    /// Temperature threshold
    temperature: f64,
    /// Ratio value
    ratio: f64,
}

#[test]
fn float_formatting_nodefault() {
    let toml = ThresholdsNoDefault {temperature : 0.0f64, ratio: 0.0f64}.to_commented_toml();
    assert!(toml.contains("temperature = 0.0"));
    assert!(toml.contains("ratio = 0.0"));
}

// --- Enum support ---

#[derive(Serialize)]
enum LogLevel {
    Debug,
    Info,
    Warn,
}

#[derive(Serialize, TomlComment)]
struct WithEnum {
    /// The log level
    #[toml_comment(inline)]
    level: LogLevel,
    /// App name
    name: String,
}

impl Default for WithEnum {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            name: "app".to_string(),
        }
    }
}

#[test]
fn enum_field_inline() {
    let toml = WithEnum::default_toml();
    let expected = "\
# The log level
level = \"Info\"
# App name
name = \"app\"
";
    assert_eq!(toml, expected);
}

#[derive(Serialize, TomlComment)]
struct WithOptionalEnum {
    level: Option<LogLevel>,
}

impl Default for WithOptionalEnum {
    fn default() -> Self {
        Self { level: None }
    }
}

#[test]
fn option_enum_none() {
    let toml = WithOptionalEnum::default_toml();
    assert_eq!(toml, "");
}

#[test]
fn option_enum_some() {
    let cfg = WithOptionalEnum {
        level: Some(LogLevel::Warn),
    };
    let toml = cfg.to_commented_toml();
    assert_eq!(toml, "level = \"Warn\"\n");
}

// --- Map support ---

#[derive(Serialize, TomlComment)]
struct WithHashMap {
    /// Environment variables
    env: HashMap<String, String>,
}

impl Default for WithHashMap {
    fn default() -> Self {
        let mut env = HashMap::new();
        env.insert("KEY".to_string(), "value".to_string());
        Self { env }
    }
}

#[test]
fn hashmap_leaf_values() {
    let toml = WithHashMap::default_toml();
    let expected = "\
# Environment variables
KEY = \"value\"
";
    assert_eq!(toml, expected);
}

#[derive(Serialize, TomlComment)]
struct WithBTreeMap {
    /// Sorted settings
    settings: BTreeMap<String, String>,
}

impl Default for WithBTreeMap {
    fn default() -> Self {
        let mut settings = BTreeMap::new();
        settings.insert("alpha".to_string(), "first".to_string());
        settings.insert("beta".to_string(), "second".to_string());
        Self { settings }
    }
}

#[test]
fn btreemap_deterministic_order() {
    let toml = WithBTreeMap::default_toml();
    let expected = "\
# Sorted settings
alpha = \"first\"
beta = \"second\"
";
    assert_eq!(toml, expected);
}

#[derive(Serialize, TomlComment)]
struct WithStructMap {
    entries: HashMap<String, ServerConfig>,
}

impl Default for WithStructMap {
    fn default() -> Self {
        let mut entries = HashMap::new();
        entries.insert(
            "main".to_string(),
            ServerConfig {
                port: 3000,
                host: "0.0.0.0".to_string(),
            },
        );
        Self { entries }
    }
}

#[test]
fn hashmap_struct_values() {
    let toml = WithStructMap::default_toml();
    assert!(toml.contains("main = {"));
    assert!(toml.contains("port = 3000"));
    assert!(toml.contains("host = \"0.0.0.0\""));
}

#[derive(Serialize, TomlComment)]
struct WithEmptyMap {
    /// Tags for the resource
    tags: BTreeMap<String, String>,
}

impl Default for WithEmptyMap {
    fn default() -> Self {
        Self {
            tags: BTreeMap::new(),
        }
    }
}

#[test]
fn empty_map_no_output() {
    let toml = WithEmptyMap::default_toml();
    assert_eq!(toml, "");
}

#[derive(Serialize, TomlComment)]
struct EnumAndMap {
    #[toml_comment(inline)]
    mode: LogLevel,
    labels: BTreeMap<String, String>,
}

impl Default for EnumAndMap {
    fn default() -> Self {
        let mut labels = BTreeMap::new();
        labels.insert("env".to_string(), "prod".to_string());
        Self {
            mode: LogLevel::Debug,
            labels,
        }
    }
}

#[test]
fn enum_and_map_combined() {
    let toml = EnumAndMap::default_toml();
    let expected = "\
mode = \"Debug\"
env = \"prod\"
";
    assert_eq!(toml, expected);
}

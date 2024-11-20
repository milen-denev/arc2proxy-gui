use std::{net::IpAddr, ops::Deref, str::FromStr};
use serde::Deserialize;

const DEV_LOGGING_LEVEL: &str = "debug";
const PROD_LOGGING_LEVEL: &str = "error";

#[derive(serde::Serialize)]
#[derive(Deserialize, Clone, Debug)]
pub struct ProxyConfiguration {
    pub listening_address: Option<String>,
    pub listening_port_http: Option<u16>,
    pub listening_port_https: Option<u16>,
    pub backtracing: Option<bool>,
    pub add_caching: Option<bool>,
    pub add_rate_limiting: Option<bool>,
    pub add_logging: Option<bool>,
    pub disable_default_body_limit: Option<bool>,
    pub api_key: Option<String>,
    pub azure_table_storage_key: Option<String>,
    pub proxy_rules: Vec<ProxyRuleInner>,
    pub logging_level: Option<String>,
    pub add_sql_injection_protection: Option<bool>,
    pub lets_encrypt_contact_email: Option<String>,
    pub enable_compression: Option<bool>,
    pub compression_flags: Option<String>,
    pub recv_buffer_size: Option<usize>,
    pub send_buffer_size: Option<usize>,
    pub ip_ttl: Option<u32>,
    pub tcp_keep_alive_seconds: Option<u64>,
    pub max_backlog: Option<i32>,
    pub enable_streaming: Option<bool>,
    pub nonblocking: Option<bool>,
    pub nodelay: Option<bool>,
    pub proxy_nodelay: Option<bool>,
    pub proxy_keepalive_sec: Option<u32>,
    pub proxy_timeout: Option<u16>,
    pub proxy_min_tls_version:  Option<String>
}

#[derive(serde::Serialize)]
#[derive(Clone, Debug)]
pub struct ProxyConfigurationInner {
    pub configuration_file_found: bool,
    pub listening_address: String,
    pub listening_port_http: u16,
    pub listening_port_https: u16,
    pub backtracing: bool,
    pub add_caching: bool,
    pub add_rate_limiting: bool,
    pub add_logging: bool,
    pub disable_default_body_limit: bool,
    pub api_key: Option<String>,
    pub azure_table_storage_key: Option<String>,
    pub proxy_rules: Vec<ProxyRuleInner>,
    pub logging_level: String,
    pub add_sql_injection_protection: bool,
    pub lets_encrypt_contact_email: Option<String>,
    pub enable_compression: bool,
    pub compression_flags: Option<String>,
    pub recv_buffer_size: Option<usize>,
    pub send_buffer_size: Option<usize>,
    pub ip_ttl: Option<u32>,
    pub tcp_keep_alive_seconds: Option<u64>,
    pub max_backlog: Option<i32>,
    pub enable_streaming: bool,
    pub nonblocking: bool,
    pub nodelay: bool,
    pub proxy_nodelay: bool,
    pub proxy_keepalive_sec: u32,
    pub proxy_timeout: u16,
    pub proxy_min_tls_version: String
}

impl ProxyConfiguration {
    pub fn into_inner(&self, config_file_found: bool) -> ProxyConfigurationInner {
        let listening_address = {
            if let Some(addr) = self.listening_address.clone() {
                if IpAddr::from_str(&addr).is_ok() {
                    addr
                }
                else {
                    panic!("Provided listening http address cannot be parsed.")
                }
            }
            else {
                "0.0.0.0".into()
            }
        };

        let listening_port_http: u16 = {
            if let Some(port) = self.listening_port_http {
                if port <= u16::MAX {
                    port
                } else {
                    panic!("Provided listening http port must be between 0 and {}.", u16::MAX);
                }
            } else {
                80
            }
        };

        let listening_port_https: u16 = {
            if let Some(port) = self.listening_port_https {
                if port <= u16::MAX {
                    port
                } else {
                    panic!("Provided listening http port must be between 0 and {}.", u16::MAX);
                }
            } else {
                443
            }
        };

        let logging_level = if cfg!(debug_assertions) {
            if let Some(level) = self.logging_level.clone() {
                if 
                    level.starts_with("off") || 
                    level.starts_with("trace") || 
                    level.starts_with("debug") || 
                    level.starts_with("info") || 
                    level.starts_with("warn") || 
                    level.starts_with("error") {
                    level
                }
                else {
                    panic!("Provided logging level is incorrect, select one of those: off, trace, debug, info, warn, error.")
                }
            }
            else {
                DEV_LOGGING_LEVEL.into()
            }
        } else {
            if let Some(level) = self.logging_level.clone() {
                if 
                    level.starts_with("off") || 
                    level.starts_with("trace") || 
                    level.starts_with("debug") || 
                    level.starts_with("info") || 
                    level.starts_with("warn") || 
                    level.starts_with("error") {
                    level
                }
                else {
                    panic!("Provided logging level is incorrect, select one of those: off, trace, debug, info, warn, error.")
                }
            }
            else {
                PROD_LOGGING_LEVEL.into()
            }
        };

        let add_sql_injection_protection: bool = {
            if let Some(add_sql_injection_protection) = self.add_sql_injection_protection {
                add_sql_injection_protection
            } else {
                false
            }
        };

        let enable_compression: bool = {
            if let Some(enable_compression) = self.enable_compression {
                enable_compression
            } else {
                false
            }
        };

        let enable_streaming: bool = {
            if let Some(enable_streaming) = self.enable_streaming {
                enable_streaming
            } else {
                true
            }
        };

        let nonblocking: bool = {
            if let Some(nonblocking) = self.nonblocking {
                nonblocking
            } else {
                true
            }
        };

        let nodelay: bool = {
            if let Some(nodelay) = self.nodelay {
                nodelay
            } else {
                true
            }
        };

        let proxy_nodelay: bool = {
            if let Some(proxy_nodelay) = self.proxy_nodelay {
                proxy_nodelay
            } else {
                true
            }
        };

        let proxy_keepalive_sec: u32 = {
            if let Some(proxy_keepalive_sec) = self.proxy_keepalive_sec {
                proxy_keepalive_sec
            } else {
                120
            }
        };

        let proxy_timeout: u16 = {
            if let Some(proxy_timeout) = self.proxy_timeout {
                proxy_timeout
            } else {
                45
            }
        };

        let proxy_min_tls_version: String = {
            if let Some(proxy_min_tls_version) = self.proxy_min_tls_version.clone() {
                match proxy_min_tls_version.as_str() {
                    "TLS_1_0" => proxy_min_tls_version,
                    "TLS_1_1" => proxy_min_tls_version,
                    "TLS_1_2" => proxy_min_tls_version,
                    "TLS_1_3" => proxy_min_tls_version,
                    _ => panic!("Invalid TLS value.")
                }
            } else {
                "TLS_1_3".into()
            }
        };

        ProxyConfigurationInner {
            configuration_file_found: config_file_found,
            listening_address: listening_address,
            listening_port_http: listening_port_http,
            listening_port_https: listening_port_https,
            api_key: self.api_key.clone(),
            backtracing: self.backtracing.unwrap_or(false),
            add_caching: self.add_caching.unwrap_or(true),
            add_rate_limiting: self.add_rate_limiting.unwrap_or(true),
            add_logging: self.add_logging.unwrap_or(true),
            disable_default_body_limit: self.disable_default_body_limit.unwrap_or(false),
            azure_table_storage_key: self.azure_table_storage_key.clone(),
            proxy_rules: self.proxy_rules.clone(),
            logging_level: logging_level,
            add_sql_injection_protection: add_sql_injection_protection,
            lets_encrypt_contact_email: self.lets_encrypt_contact_email.clone(),
            enable_compression: enable_compression,
            compression_flags: self.compression_flags.clone(),
            recv_buffer_size: self.recv_buffer_size.clone(),
            send_buffer_size: self.send_buffer_size.clone(),
            ip_ttl: self.ip_ttl.clone(),
            tcp_keep_alive_seconds: self.tcp_keep_alive_seconds.clone(),
            max_backlog: self.max_backlog.clone(),
            enable_streaming: enable_streaming,
            nodelay: nodelay,
            nonblocking: nonblocking,
            proxy_nodelay: proxy_nodelay,
            proxy_keepalive_sec: proxy_keepalive_sec,
            proxy_timeout: proxy_timeout,
            proxy_min_tls_version: proxy_min_tls_version
        }
    }

    pub fn default_inner() -> ProxyConfigurationInner {
        let listening_address = {
            "0.0.0.0".into()
        };

        let listening_port_http: u16 = {         
            80
        };

        let listening_port_https: u16 = {
            443
        };

        if cfg!(debug_assertions) {
            ProxyConfigurationInner {
                configuration_file_found: false,
                listening_address: listening_address,
                listening_port_http: listening_port_http,
                listening_port_https: listening_port_https,
                backtracing: true,
                add_caching: true,
                add_rate_limiting: false,
                add_logging: false,
                disable_default_body_limit: false,
                api_key: None,
                azure_table_storage_key: None,
                proxy_rules: Vec::default(),
                logging_level: DEV_LOGGING_LEVEL.into(),
                add_sql_injection_protection: false,
                lets_encrypt_contact_email: None,
                enable_compression: false,
                compression_flags: None,
                recv_buffer_size: None,
                send_buffer_size: None,
                ip_ttl: None,
                tcp_keep_alive_seconds: None,
                max_backlog: None,
                enable_streaming: true,
                nonblocking: true,
                nodelay: true,
                proxy_nodelay: true,
                proxy_keepalive_sec: 120,
                proxy_timeout: 45,
                proxy_min_tls_version: "TLS_1_3".into()
            }
        } else {
            ProxyConfigurationInner {
                configuration_file_found: false,
                listening_address: listening_address,
                listening_port_http: listening_port_http,
                listening_port_https: listening_port_https,
                backtracing: false,
                add_caching: true,
                add_rate_limiting: true,
                add_logging: true,
                disable_default_body_limit: false,
                api_key: None,
                azure_table_storage_key: None,
                proxy_rules: Vec::default(),
                logging_level: PROD_LOGGING_LEVEL.into(),
                add_sql_injection_protection: false,
                lets_encrypt_contact_email: None,
                enable_compression: false,
                compression_flags: None,
                recv_buffer_size: None,
                send_buffer_size: None,
                ip_ttl: None,
                tcp_keep_alive_seconds: None,
                max_backlog: None,
                enable_streaming: true,
                nonblocking: true,
                nodelay: true,
                proxy_nodelay: true,
                proxy_keepalive_sec: 120,
                proxy_timeout: 45,
                proxy_min_tls_version: "TLS_1_3".into()
            }
        }
    }
}

impl Default for ProxyConfigurationInner {
    fn default() -> Self {
        let listening_address = {
            "0.0.0.0".into()
        };

        let listening_port_http: u16 = {         
            80
        };

        let listening_port_https: u16 = {
            443
        };

        if cfg!(debug_assertions) {
            Self {
                configuration_file_found: false,
                listening_address: listening_address,
                listening_port_http: listening_port_http,
                listening_port_https: listening_port_https,
                backtracing: true,
                add_caching: true,
                add_rate_limiting: false,
                add_logging: false,
                disable_default_body_limit: false,
                api_key: None,
                azure_table_storage_key: None,
                proxy_rules: Vec::default(),
                logging_level: DEV_LOGGING_LEVEL.into(),
                add_sql_injection_protection: false,
                lets_encrypt_contact_email: None,
                enable_compression: false,
                compression_flags: None,
                recv_buffer_size: None,
                send_buffer_size: None,
                ip_ttl: None,
                tcp_keep_alive_seconds: None,
                max_backlog: None,
                enable_streaming: true,
                nonblocking: true,
                nodelay: true,
                proxy_nodelay: true,
                proxy_keepalive_sec: 120,
                proxy_timeout: 45,
                proxy_min_tls_version: "TLS_1_3".into()
            }
        } else {
            Self {
                configuration_file_found: false,
                listening_address: listening_address,
                listening_port_http: listening_port_http,
                listening_port_https: listening_port_https,
                backtracing: false,
                add_caching: true,
                add_rate_limiting: true,
                add_logging: true,
                disable_default_body_limit: false,
                api_key: None,
                azure_table_storage_key: None,
                proxy_rules: Vec::default(),
                logging_level: PROD_LOGGING_LEVEL.into(),
                add_sql_injection_protection: false,
                lets_encrypt_contact_email: None,
                enable_compression: false,
                compression_flags: None,
                recv_buffer_size: None,
                send_buffer_size: None,
                ip_ttl: None,
                tcp_keep_alive_seconds: None,
                max_backlog: None,
                enable_streaming: true,
                nonblocking: true,
                nodelay: true,
                proxy_nodelay: true,
                proxy_keepalive_sec: 120,
                proxy_timeout: 45,
                proxy_min_tls_version: "TLS_1_3".into()
            }
        }
    }
}

#[derive(serde::Serialize)]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct ProxyRuleInner {
    pub domain: String,
    pub max_age_seconds: u64,
    pub paths: Option<Vec<String>>,
    pub forward_addr: Option<String>,
    pub forward_ipv4: Option<String>,
    pub forward_ipv6: Option<String>,
    pub forward_port_http: Option<u16>,
    pub forward_port_https: Option<u16>,
    pub rule_type: RuleType,
    pub enable_logging: bool,
    pub path_rules: Option<Vec<PathRule>>,
    pub routing_rules: Option<RoutingRule>,
    pub ignore_query_string: bool,
    pub enable_sql_injection_protection: bool,
    pub disallowed_user_agents: Option<Vec<UserAgentRule>>,
    pub enable_compression: bool,
    pub compression_flags: Option<String>,
    pub enable_minification: bool,
    pub minification_flags: Option<String>,
    pub enable_webp_transformation: bool,
    pub webp_transformation_min_age: Option<u64>
}

#[derive(serde::Serialize)]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct PathRule {
    pub max_age_seconds: u64,
    pub path: String,
    pub match_type: MatchType,
    pub rule_type: RuleType
}

#[derive(serde::Serialize)]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub enum RuleType {
    Whitelist,
    Blacklist
}

#[derive(serde::Serialize)]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub enum MatchType {
    Contains,
    Equals,
    StartsWith,
    EndsWith,
    DoesNotContain,
    DoesNotEqual
}

#[derive(serde::Serialize)]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Default)]
pub enum RoutingMethod {
    Weighted,
    #[default]
    Priority,
    Performance
}

impl Deref for RoutingMethod {
    type Target = RoutingMethod;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Performance => &RoutingMethod::Performance,
            Self::Priority => &RoutingMethod::Priority,
            Self::Weighted => &RoutingMethod::Weighted,
        }
    }
}

#[derive(serde::Serialize)]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct RoutingRule {
    pub routing_method: RoutingMethod,
    pub routing_locations: Vec<RoutingLocation>,
    pub https_only: bool,
    pub enable_health_checks: bool,
    pub health_check_interval: u32,
    pub health_check_path: Option<String>
}

#[derive(serde::Serialize)]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct RoutingLocation {
    pub primary: Option<bool>,
    pub priority: Option<u16>,
    pub forward_addr: Option<String>,
    pub forward_ipv4: Option<String>,
    pub forward_ipv6: Option<String>,
    pub forward_port_http: Option<u16>,
    pub forward_port_https: Option<u16>
}

#[derive(serde::Serialize)]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct UserAgentRule {
    pub user_agent: String,
    pub match_type: MatchType
}
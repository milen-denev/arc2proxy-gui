// Enums
const RuleType = {
    Whitelist: 'Whitelist',
    Blacklist: 'Blacklist',
};

const MatchType = {
    Contains: 'Contains',
    Equals: 'Equals',
    StartsWith: 'StartsWith',
    EndsWith: 'EndsWith',
    DoesNotContain: 'DoesNotContain',
    DoesNotEqual: 'DoesNotEqual',
};

const RoutingMethod = {
    Weighted: 'Weighted',
    Priority: 'Priority',
    Performance: 'Performance',
};

class ProxyConfiguration {
    constructor() {
        this.listening_address = undefined; // string
        this.listening_port_http = undefined; // number
        this.listening_port_https = undefined; // number
        this.backtracing = undefined; // boolean
        this.add_caching = undefined; // boolean
        this.add_rate_limiting = undefined; // boolean
        this.add_logging = undefined; // boolean
        this.disable_default_body_limit = undefined; // boolean
        this.api_key = undefined; // string
        this.azure_table_storage_key = undefined; // string
        this.proxy_rules = []; // array of ProxyRuleInner
        this.logging_level = undefined; // string
        this.add_sql_injection_protection = undefined; // boolean
        this.lets_encrypt_contact_email = undefined; // string
        this.enable_compression = undefined; // boolean
        this.compression_flags = undefined; // string
        this.recv_buffer_size = undefined; // number
        this.send_buffer_size = undefined; // number
        this.ip_ttl = undefined; // number
        this.tcp_keep_alive_seconds = undefined; // number
        this.max_backlog = undefined; // number
    }
}
  
class ProxyConfigurationInner {
    constructor() {
        this.configuration_file_found = false; // boolean
        this.listening_address = ''; // string
        this.listening_port_http = 80; // number
        this.listening_port_https = 443; // number
        this.backtracing = false; // boolean
        this.add_caching = true; // boolean
        this.add_rate_limiting = true; // boolean
        this.add_logging = true; // boolean
        this.disable_default_body_limit = false; // boolean
        this.api_key = undefined; // string
        this.azure_table_storage_key = undefined; // string
        this.proxy_rules = []; // array of ProxyRuleInner
        this.logging_level = 'error'; // string
        this.add_sql_injection_protection = false; // boolean
        this.lets_encrypt_contact_email = undefined; // string
        this.enable_compression = false; // boolean
        this.compression_flags = undefined; // string
        this.recv_buffer_size = undefined; // number
        this.send_buffer_size = undefined; // number
        this.ip_ttl = undefined; // number
        this.tcp_keep_alive_seconds = undefined; // number
        this.max_backlog = undefined; // number
    }
}

class ProxyRuleInner {
    constructor() {
        this.domain = ''; // string
        this.max_age_seconds = 0; // number
        this.paths = []; // array of strings
        this.forward_addr = undefined; // string
        this.forward_ipv4 = undefined; // string
        this.forward_ipv6 = undefined; // string
        this.forward_port_http = undefined; // number
        this.forward_port_https = undefined; // number
        this.rule_type = RuleType.Whitelist; // RuleType
        this.enable_logging = false; // boolean
        this.path_rules = []; // array of PathRule
        this.routing_rules = undefined; // RoutingRule
        this.ignore_query_string = false; // boolean
        this.enable_sql_injection_protection = false; // boolean
        this.disallowed_user_agents = []; // array of UserAgentRule
        this.enable_compression = false; // boolean
        this.compression_flags = undefined; // string
    }
}
  
class PathRule {
    constructor() {
        this.max_age_seconds = 0; // number
        this.path = ''; // string
        this.match_type = MatchType.Equals; // MatchType
        this.rule_type = RuleType.Whitelist; // RuleType
    }
}
  
class RoutingRule {
    constructor() {
        this.routing_method = RoutingMethod.Priority; // RoutingMethod
        this.routing_locations = []; // array of RoutingLocation
        this.https_only = false; // boolean
        this.enable_health_checks = false; // boolean
        this.health_check_interval = 0; // number
        this.health_check_path = undefined; // string
    }
}
  
class RoutingLocation {
    constructor() {
        this.primary = false; // boolean
        this.priority = undefined; // number
        this.forward_addr = undefined; // string
        this.forward_ipv4 = undefined; // string
        this.forward_ipv6 = undefined; // string
        this.forward_port_http = undefined; // number
        this.forward_port_https = undefined; // number
    }
}
  
class UserAgentRule {
    constructor() {
        this.user_agent = ''; // string
        this.match_type = MatchType.Equals; // MatchType
    }
}
  
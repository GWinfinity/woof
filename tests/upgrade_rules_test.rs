//! Integration tests for Go version upgrade rules

use woofmt::config::Config;
use woofmt::rules::{get_all_rules, get_enabled_rules, parse_version, RuleCategory};

#[test]
fn test_version_parsing() {
    assert_eq!(parse_version("1.21"), Some(1021));
    assert_eq!(parse_version("1.22"), Some(1022));
    assert_eq!(parse_version("1.23"), Some(1023));
    assert_eq!(parse_version("1.24"), Some(1024));
    assert_eq!(parse_version("1.25"), Some(1025));
    assert_eq!(parse_version("2.0"), Some(2000));
    assert_eq!(parse_version("invalid"), None);
    assert_eq!(parse_version(""), None);
}

/// Get upgrade rules that have version requirements (UP1221, UP1222, etc.)
fn get_versioned_upgrade_rules() -> Vec<Box<dyn woofmt::rules::Rule>> {
    get_all_rules()
        .into_iter()
        .filter(|r| {
            let meta = r.metadata();
            meta.category == RuleCategory::Upgrade && meta.min_go_version.is_some()
        })
        .collect()
}

#[test]
fn test_upgrade_rules_have_version() {
    let versioned_rules = get_versioned_upgrade_rules();

    assert!(
        !versioned_rules.is_empty(),
        "Should have versioned upgrade rules"
    );

    for rule in &versioned_rules {
        let meta = rule.metadata();
        let version = meta.min_go_version.unwrap();
        assert!(
            version.starts_with("1."),
            "Version should start with 1.: {}",
            version
        );

        let minor: u32 = version[2..].parse().expect("Valid minor version");
        assert!(
            (22..=25).contains(&minor),
            "Version should be 1.22-1.25: {}",
            version
        );

        // Verify code starts with UP
        assert!(
            meta.code.starts_with("UP"),
            "Versioned upgrade rules should start with UP: {}",
            meta.code
        );
    }
}

#[test]
fn test_upgrade_rules_by_version() {
    let versioned_rules = get_versioned_upgrade_rules();

    let v22_count = versioned_rules
        .iter()
        .filter(|r| r.metadata().min_go_version == Some("1.22"))
        .count();
    let v23_count = versioned_rules
        .iter()
        .filter(|r| r.metadata().min_go_version == Some("1.23"))
        .count();
    let v24_count = versioned_rules
        .iter()
        .filter(|r| r.metadata().min_go_version == Some("1.24"))
        .count();
    let v25_count = versioned_rules
        .iter()
        .filter(|r| r.metadata().min_go_version == Some("1.25"))
        .count();

    assert_eq!(v22_count, 5, "Should have 5 Go 1.22 rules");
    assert_eq!(v23_count, 5, "Should have 5 Go 1.23 rules");
    assert_eq!(v24_count, 4, "Should have 4 Go 1.24 rules");
    assert_eq!(v25_count, 4, "Should have 4 Go 1.25 rules");

    let total = v22_count + v23_count + v24_count + v25_count;
    assert_eq!(total, 18, "Should have 18 total versioned upgrade rules");
}

/// Count enabled versioned upgrade rules for a given config
fn count_enabled_versioned_upgrade_rules(config: &Config) -> usize {
    get_enabled_rules(config)
        .into_iter()
        .filter(|r| {
            let meta = r.metadata();
            meta.category == RuleCategory::Upgrade && meta.min_go_version.is_some()
        })
        .count()
}

#[test]
fn test_version_filtering_go121() {
    let mut config = Config::default();
    config.global.target_go_version = "1.21".to_string();

    let count = count_enabled_versioned_upgrade_rules(&config);
    assert_eq!(count, 0, "Go 1.21 should have 0 versioned upgrade rules");
}

#[test]
fn test_version_filtering_go122() {
    let mut config = Config::default();
    config.global.target_go_version = "1.22".to_string();

    let count = count_enabled_versioned_upgrade_rules(&config);
    assert_eq!(count, 5, "Go 1.22 should have 5 versioned upgrade rules");
}

#[test]
fn test_version_filtering_go123() {
    let mut config = Config::default();
    config.global.target_go_version = "1.23".to_string();

    let count = count_enabled_versioned_upgrade_rules(&config);
    assert_eq!(
        count, 10,
        "Go 1.23 should have 10 versioned upgrade rules (5 + 5)"
    );
}

#[test]
fn test_version_filtering_go124() {
    let mut config = Config::default();
    config.global.target_go_version = "1.24".to_string();

    let count = count_enabled_versioned_upgrade_rules(&config);
    assert_eq!(
        count, 14,
        "Go 1.24 should have 14 versioned upgrade rules (5 + 5 + 4)"
    );
}

#[test]
fn test_version_filtering_go125() {
    let mut config = Config::default();
    config.global.target_go_version = "1.25".to_string();

    let count = count_enabled_versioned_upgrade_rules(&config);
    assert_eq!(
        count, 18,
        "Go 1.25 should have all 18 versioned upgrade rules"
    );
}

#[test]
fn test_specific_rule_metadata() {
    let all_rules = get_all_rules();

    // Check UP1221
    let up1221 = all_rules
        .iter()
        .find(|r| r.metadata().code == "UP1221")
        .expect("UP1221 should exist");
    assert_eq!(up1221.metadata().min_go_version, Some("1.22"));
    assert_eq!(up1221.metadata().category, RuleCategory::Upgrade);

    // Check UP1254
    let up1254 = all_rules
        .iter()
        .find(|r| r.metadata().code == "UP1254")
        .expect("UP1254 should exist");
    assert_eq!(up1254.metadata().min_go_version, Some("1.25"));
    assert_eq!(up1254.metadata().category, RuleCategory::Upgrade);
}

#[test]
fn test_other_upgrade_rules_always_enabled() {
    // Rules like GEN002, WS001 from p0_critical are always enabled
    // (they don't have min_go_version)
    let mut config = Config::default();
    config.global.target_go_version = "1.21".to_string();

    let all_enabled = get_enabled_rules(&config);
    let always_enabled_upgrades: Vec<_> = all_enabled
        .iter()
        .filter(|r| {
            let meta = r.metadata();
            meta.category == RuleCategory::Upgrade && meta.min_go_version.is_none()
        })
        .collect();

    // These should be GENxxx and WSxxx rules
    assert!(
        !always_enabled_upgrades.is_empty(),
        "Should have always-enabled upgrade rules (GEN, WS, etc.)"
    );

    for rule in &always_enabled_upgrades {
        let code = rule.metadata().code;
        assert!(
            code.starts_with("GEN") || code.starts_with("WS") || code.starts_with("FUZZ"),
            "Always-enabled upgrade rules should be GEN, WS, or FUZZ: {}",
            code
        );
    }
}

#[test]
fn test_rules_list_includes_upgrade() {
    // Verify that rules command would include upgrade rules
    let all_rules = get_all_rules();
    let upgrade_codes: Vec<_> = all_rules
        .iter()
        .filter(|r| r.metadata().category == RuleCategory::Upgrade)
        .map(|r| r.metadata().code)
        .collect();

    assert!(upgrade_codes.contains(&"UP1221"), "Should include UP1221");
    assert!(upgrade_codes.contains(&"UP1254"), "Should include UP1254");
    assert!(upgrade_codes.contains(&"GEN002"), "Should include GEN002");
}

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Rule {
    pub paths: Option<Vec<String>>,
    pub hosts: Option<Vec<String>>,
    pub methods: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GatewayAction {
    #[serde(default)]
    pub rules: Vec<Rule>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RateLimitPolicy {
    pub name: String,
    pub rate_limit_domain: String,
    pub upstream_cluster: String,
    pub hostnames: Vec<String>,
    pub gateway_actions: Vec<GatewayAction>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PluginConfiguration {
    pub rate_limit_policies: Vec<RateLimitPolicy>,
    // Deny request when faced with an irrecoverable failure.
    pub failure_mode_deny: bool,
}

fn main() {
    let config: &str = r#"{
        "failure_mode_deny": true,
        "rate_limit_policies": [
        {
            "name": "some-name",
            "rate_limit_domain": "RLS-domain",
            "upstream_cluster": "limitador-cluster",
            "hostnames": ["*.toystore.com", "example.com"],
            "gateway_actions": [
            {
                "rules": [
                {
                    "paths": ["/admin/toy"],
                    "hosts": ["cars.toystore.com"],
                    "methods": ["POST"]
                }]
            }, 
            {
            }
            ]
        }
        ]
    }"#;

    let res = serde_json::from_str::<PluginConfiguration>(config);
    if let Err(ref e) = res {
        eprintln!("{}", e);
    }
    assert!(res.is_ok());

    let plugin_config = res.unwrap();

    println!(
        "done: {}",
        plugin_config.rate_limit_policies[0].gateway_actions[0]
            .rules
            .len()
    );

    println!(
        "done: {}",
        plugin_config.rate_limit_policies[0].gateway_actions[1]
            .rules
            .len()
    );
}

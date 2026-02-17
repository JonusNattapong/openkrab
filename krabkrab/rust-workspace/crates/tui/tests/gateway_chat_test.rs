use tui::{
    resolve_gateway_connection, GatewayConfig, GatewayConnectionOptions, GatewayRemote, ResolvedConnection,
};

#[test]
fn throws_when_url_override_is_missing_explicit_credentials() {
    let opts = GatewayConnectionOptions { url: Some("wss://override.example/ws".into()), token: None, password: None };
    let config = GatewayConfig { mode: Some("local".into()), bind: None, remote: None, auth_token: None };

    let res = resolve_gateway_connection(opts, config, || 18789u16, || None, || None);
    assert!(res.is_err());
    let msg = res.err().unwrap();
    assert!(msg.contains("explicit credentials"));
}

#[test]
fn uses_explicit_token_when_url_override_set() {
    let opts = GatewayConnectionOptions { url: Some("wss://override.example/ws".into()), token: Some("explicit-token".into()), password: None };
    let config = GatewayConfig { mode: Some("local".into()), bind: None, remote: None, auth_token: None };

    let res = resolve_gateway_connection(opts, config, || 18789u16, || None, || None).unwrap();
    assert_eq!(res.url, "wss://override.example/ws");
    assert_eq!(res.token, Some("explicit-token".into()));
    assert_eq!(res.password, None);
}

#[test]
fn uses_explicit_password_when_url_override_set() {
    let opts = GatewayConnectionOptions { url: Some("wss://override.example/ws".into()), token: None, password: Some("explicit-password".into()) };
    let config = GatewayConfig { mode: Some("local".into()), bind: None, remote: None, auth_token: None };

    let res = resolve_gateway_connection(opts, config, || 18789u16, || None, || None).unwrap();
    assert_eq!(res.url, "wss://override.example/ws");
    assert_eq!(res.token, None);
    assert_eq!(res.password, Some("explicit-password".into()));
}

#[test]
fn uses_tailnet_host_when_local_bind_is_tailnet() {
    let opts = GatewayConnectionOptions { url: None, token: None, password: None };
    let config = GatewayConfig { mode: Some("local".into()), bind: Some("tailnet".into()), remote: None, auth_token: None };

    let res = resolve_gateway_connection(opts, config, || 18800u16, || Some("100.64.0.1".into()), || None).unwrap();
    assert_eq!(res.url, "ws://100.64.0.1:18800");
}

#[test]
fn uses_lan_host_when_local_bind_is_lan() {
    let opts = GatewayConnectionOptions { url: None, token: None, password: None };
    let config = GatewayConfig { mode: Some("local".into()), bind: Some("lan".into()), remote: None, auth_token: None };

    let res = resolve_gateway_connection(opts, config, || 18800u16, || None, || Some("192.168.1.42".into())).unwrap();
    assert_eq!(res.url, "ws://192.168.1.42:18800");
}

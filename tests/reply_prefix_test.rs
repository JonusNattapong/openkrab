use krabkrab::channels::reply_prefix::*;
use serde_json::json;

#[test]
fn test_response_prefix_auto_uses_identity() {
    let cfg = json!({
        "agents": { "main": { "identity": { "name": "AgentX" } } },
        "messages": { "responsePrefix": "auto" }
    });

    let (response_prefix, provider, on_model_selected) =
        create_reply_prefix_options(&cfg, "main", None, None);

    assert_eq!(response_prefix, Some("[AgentX]".to_string()));

    // Verify provider returns a context with identity name
    let ctx = provider();
    assert_eq!(ctx.identity_name, Some("AgentX".to_string()));

    // Test on_model_selected updates context
    let model_ctx = ModelSelectionContext {
        provider: "openai".to_string(),
        model: "gpt-x".to_string(),
        think_level: Some("medium".to_string()),
    };
    on_model_selected(model_ctx);
    let ctx2 = provider();
    assert_eq!(ctx2.provider.unwrap(), "openai");
    assert_eq!(ctx2.model.unwrap(), "gpt-x");
    assert_eq!(ctx2.model_full.unwrap(), "openai/gpt-x");
    assert_eq!(ctx2.thinking_level.unwrap(), "medium");
}

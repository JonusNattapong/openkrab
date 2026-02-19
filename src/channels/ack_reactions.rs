use futures::executor::block_on;
use futures::future::BoxFuture;
use std::sync::Arc;
use std::thread;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AckReactionScope {
    All,
    Direct,
    GroupAll,
    GroupMentions,
    Off,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatsAppAckReactionMode {
    Always,
    Mentions,
    Never,
}

pub struct AckReactionGateParams {
    pub scope: Option<AckReactionScope>,
    pub is_direct: bool,
    pub is_group: bool,
    pub is_mentionable_group: bool,
    pub require_mention: bool,
    pub can_detect_mention: bool,
    pub effective_was_mentioned: bool,
    pub should_bypass_mention: Option<bool>,
}

pub fn should_ack_reaction(params: &AckReactionGateParams) -> bool {
    use AckReactionScope::*;
    let scope = params.scope.clone().unwrap_or(GroupMentions);
    match scope {
        Off | None => false,
        All => true,
        Direct => params.is_direct,
        GroupAll => params.is_group,
        GroupMentions => {
            if !params.is_mentionable_group {
                return false;
            }
            if !params.require_mention {
                return false;
            }
            if !params.can_detect_mention {
                return false;
            }
            params.effective_was_mentioned || params.should_bypass_mention == Some(true)
        }
    }
}

pub struct WhatsAppParams {
    pub emoji: &'static str,
    pub is_direct: bool,
    pub is_group: bool,
    pub direct_enabled: bool,
    pub group_mode: WhatsAppAckReactionMode,
    pub was_mentioned: bool,
    pub group_activated: bool,
}

pub fn should_ack_reaction_for_whatsapp(params: &WhatsAppParams) -> bool {
    if params.emoji.is_empty() {
        return false;
    }
    if params.is_direct {
        return params.direct_enabled;
    }
    if !params.is_group {
        return false;
    }
    match params.group_mode {
        WhatsAppAckReactionMode::Never => false,
        WhatsAppAckReactionMode::Always => true,
        WhatsAppAckReactionMode::Mentions => {
            let gate = AckReactionGateParams {
                scope: Some(AckReactionScope::GroupMentions),
                is_direct: false,
                is_group: true,
                is_mentionable_group: true,
                require_mention: true,
                can_detect_mention: true,
                effective_was_mentioned: params.was_mentioned,
                should_bypass_mention: Some(params.group_activated),
            };
            should_ack_reaction(&gate)
        }
    }
}

pub fn remove_ack_reaction_after_reply(
    remove_after_reply: bool,
    ack_reaction_future: Option<BoxFuture<'static, bool>>,
    ack_reaction_value: Option<&str>,
    remove: Arc<dyn Fn() -> BoxFuture<'static, Result<(), String>> + Send + Sync>,
    on_error: Option<Arc<dyn Fn(String) + Send + Sync>>,
) {
    if !remove_after_reply {
        return;
    }
    let fut = match ack_reaction_future {
        Some(f) => f,
        None => return,
    };
    if ack_reaction_value.is_none() {
        return;
    }

    // spawn a thread to await the future and call remove if ack succeeded
    thread::spawn(move || {
        let did_ack = block_on(fut);
        if !did_ack {
            return;
        }
        let r = block_on((remove)());
        if let Err(e) = r {
            if let Some(cb) = on_error {
                cb(e);
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::{ready, BoxFuture};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    fn to_box_bool(v: bool) -> BoxFuture<'static, bool> {
        Box::pin(ready(v))
    }

    fn to_box_remove(
        counter: Arc<AtomicUsize>,
        ret_ok: bool,
    ) -> Arc<dyn Fn() -> BoxFuture<'static, Result<(), String>> + Send + Sync> {
        Arc::new(move || {
            let c = counter.clone();
            Box::pin(async move {
                c.fetch_add(1, Ordering::SeqCst);
                if ret_ok {
                    Ok(())
                } else {
                    Err("err".to_string())
                }
            })
        })
    }

    #[test]
    fn test_should_ack_reaction_direct_and_group_all() {
        let p = AckReactionGateParams {
            scope: Some(AckReactionScope::Direct),
            is_direct: true,
            is_group: false,
            is_mentionable_group: false,
            require_mention: false,
            can_detect_mention: false,
            effective_was_mentioned: false,
            should_bypass_mention: None,
        };
        assert!(should_ack_reaction(&p));

        let p2 = AckReactionGateParams {
            scope: Some(AckReactionScope::GroupAll),
            is_direct: false,
            is_group: true,
            is_mentionable_group: true,
            require_mention: false,
            can_detect_mention: false,
            effective_was_mentioned: false,
            should_bypass_mention: None,
        };
        assert!(should_ack_reaction(&p2));
    }

    #[test]
    fn test_should_ack_reaction_off() {
        let p = AckReactionGateParams {
            scope: Some(AckReactionScope::Off),
            is_direct: true,
            is_group: true,
            is_mentionable_group: true,
            require_mention: true,
            can_detect_mention: true,
            effective_was_mentioned: true,
            should_bypass_mention: None,
        };
        assert!(!should_ack_reaction(&p));
    }

    #[test]
    fn test_should_ack_reaction_for_whatsapp_modes() {
        let params = WhatsAppParams {
            emoji: "👀",
            is_direct: true,
            is_group: false,
            direct_enabled: false,
            group_mode: WhatsAppAckReactionMode::Mentions,
            was_mentioned: false,
            group_activated: false,
        };
        let res = should_ack_reaction_for_whatsapp(&params);
        assert!(!res);

        let params2 = WhatsAppParams {
            emoji: "👀",
            is_direct: false,
            is_group: true,
            direct_enabled: true,
            group_mode: WhatsAppAckReactionMode::Always,
            was_mentioned: false,
            group_activated: false,
        };
        let res2 = should_ack_reaction_for_whatsapp(&params2);
        assert!(res2);

        let params3 = WhatsAppParams {
            emoji: "👀",
            is_direct: false,
            is_group: true,
            direct_enabled: true,
            group_mode: WhatsAppAckReactionMode::Never,
            was_mentioned: true,
            group_activated: true,
        };
        let res3 = should_ack_reaction_for_whatsapp(&params3);
        assert!(!res3);
    }

    #[test]
    fn test_remove_ack_reaction_after_reply() {
        let counter = Arc::new(AtomicUsize::new(0));
        let remove = to_box_remove(counter.clone(), true);
        remove_ack_reaction_after_reply(true, Some(to_box_bool(true)), Some("👀"), remove, None);
        // give spawned thread a moment
        thread::sleep(std::time::Duration::from_millis(10));
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_remove_skips_when_no_ack() {
        let counter = Arc::new(AtomicUsize::new(0));
        let remove = to_box_remove(counter.clone(), true);
        remove_ack_reaction_after_reply(true, Some(to_box_bool(false)), Some("👀"), remove, None);
        thread::sleep(std::time::Duration::from_millis(10));
        assert_eq!(counter.load(Ordering::SeqCst), 0);
    }
}

//! WebRTC support for OpenKrab voice calling.

use anyhow::{anyhow, Result};
use axum::{
    extract::Json,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use webrtc::{
    api::{
        interceptor_registry::register_default_interceptors,
        media_engine::{MediaEngine, MIME_TYPE_OPUS},
        APIBuilder,
    },
    ice_transport::ice_server::RTCIceServer,
    interceptor::registry::Registry,
    peer_connection::{
        configuration::RTCConfiguration, peer_connection_state::RTCPeerConnectionState,
        sdp::session_description::RTCSessionDescription,
    },
    rtp_transceiver::rtp_codec::RTCRtpCodecCapability,
    track::track_local::track_local_static_sample::TrackLocalStaticSample,
    track::track_local::TrackLocal,
};

/// The incoming SDP offer from the client.
#[derive(Debug, Deserialize, Serialize)]
pub struct WebRtcOffer {
    pub sdp: String,
    pub r#type: String,
}

/// The SDP answer sent back to the client.
#[derive(Debug, Serialize, Deserialize)]
pub struct WebRtcAnswer {
    pub sdp: String,
    pub r#type: String,
}

/// Start the WebRTC signaling service as an Axum router.
pub fn webrtc_router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().route("/offer", post(handle_webrtc_offer))
}

/// Handle an incoming SDP offer and return an SDP answer.
pub async fn handle_webrtc_offer(
    Json(offer): Json<WebRtcOffer>,
) -> Result<Json<WebRtcAnswer>, String> {
    match process_offer(offer).await {
        Ok(answer) => Ok(Json(answer)),
        Err(e) => {
            tracing::error!("Failed to process WebRTC offer: {}", e);
            Err(e.to_string())
        }
    }
}

async fn process_offer(offer: WebRtcOffer) -> Result<WebRtcAnswer> {
    // 1. Setup Media Engine
    let mut m = MediaEngine::default();
    m.register_default_codecs()?;

    // 2. Setup Interceptor Registry
    let mut registry = Registry::new();
    registry = register_default_interceptors(registry, &mut m)?;

    // 3. Create API
    let api = APIBuilder::new()
        .with_media_engine(m)
        .with_interceptor_registry(registry)
        .build();

    // 4. Configure PeerConnection
    let config = RTCConfiguration {
        ice_servers: vec![RTCIceServer {
            urls: vec!["stun:stun.l.google.com:19302".to_owned()],
            ..Default::default()
        }],
        ..Default::default()
    };

    // 5. Create PeerConnection
    let peer_connection = Arc::new(api.new_peer_connection(config).await?);

    // Create an audio track to send to the user
    let audio_track = Arc::new(TrackLocalStaticSample::new(
        RTCRtpCodecCapability {
            mime_type: MIME_TYPE_OPUS.to_owned(),
            ..Default::default()
        },
        "audio".to_owned(),
        "openkrab-audio".to_owned(),
    ));

    // Add this track to the peer connection
    let rtp_sender = peer_connection
        .add_track(Arc::clone(&audio_track) as Arc<dyn TrackLocal + Send + Sync>)
        .await?;

    // Read incoming RTCP packets
    // Before these packets are returned they are processed by interceptors
    tokio::spawn(async move {
        let mut rtcp_buf = vec![0u8; 1500];
        while let Ok((_, _)) = rtp_sender.read(&mut rtcp_buf).await {}
        Result::<()>::Ok(())
    });

    // Handle PeerConnection state changes
    peer_connection.on_peer_connection_state_change(Box::new(
        move |s: RTCPeerConnectionState| {
            tracing::info!("Peer Connection State has changed: {}", s);
            if s == RTCPeerConnectionState::Failed || s == RTCPeerConnectionState::Closed {
                tracing::info!("Peer Connection finished");
            }
            Box::pin(async {})
        },
    ));

    // Handle incoming tracks
    peer_connection.on_track(Box::new(
        move |track, _receiver, _receiver_track| {
            tracing::info!("Received incoming track: {}", track.id());
            Box::pin(async {})
        },
    ));

    // 6. Set Remote Description (the Offer)
    let mut session_desc = RTCSessionDescription::default();
    session_desc.sdp_type = webrtc::peer_connection::sdp::sdp_type::RTCSdpType::Offer;
    session_desc.sdp = offer.sdp;
    
    peer_connection.set_remote_description(session_desc).await?;


    // 7. Create Answer
    let answer = peer_connection.create_answer(None).await?;

    // 8. Set Local Description (the Answer)
    // We cannot immediately return the answer, we must set it as local description first.
    // However, ICE gathering works asynchronously. We can wait for it to complete.
    let mut gather_complete = peer_connection.gathering_complete_promise().await;

    peer_connection.set_local_description(answer).await?;

    // Wait for ICE gathering to complete so the answer contains all candidates
    // Alternative: We could also return the SDP immediately and perform Trickle ICE.
    let _ = gather_complete.recv().await;

    let local_desc = peer_connection
        .local_description()
        .await
        .ok_or_else(|| anyhow!("Failed to get local description"))?;

    Ok(WebRtcAnswer {
        sdp: local_desc.sdp,
        r#type: "answer".to_string(),
    })
}

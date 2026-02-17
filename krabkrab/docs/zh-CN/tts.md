---
read_when:
  - ä¸ºå›žå¤å¯ç”¨æ–‡æœ¬è½¬è¯­éŸ³
  - é…ç½® TTS æä¾›å•†æˆ–é™åˆ¶
  - ä½¿ç”¨ /tts å‘½ä»¤
summary: å‡ºç«™å›žå¤çš„æ–‡æœ¬è½¬è¯­éŸ³ï¼ˆTTSï¼‰
title: æ–‡æœ¬è½¬è¯­éŸ³
x-i18n:
  generated_at: "2026-02-03T10:13:55Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 070ff0cc8592f64c6c9e4ddaddc7e8fba82f0692ceded6fe833ec9ba5b61e6fb
  source_path: tts.md
  workflow: 15
---

# æ–‡æœ¬è½¬è¯­éŸ³ï¼ˆTTSï¼‰

KrabKrab å¯ä»¥ä½¿ç”¨ ElevenLabsã€OpenAI æˆ– Edge TTS å°†å‡ºç«™å›žå¤è½¬æ¢ä¸ºéŸ³é¢‘ã€‚å®ƒå¯ä»¥åœ¨ä»»ä½• KrabKrab èƒ½å‘é€éŸ³é¢‘çš„åœ°æ–¹å·¥ä½œï¼›Telegram ä¼šæ˜¾ç¤ºåœ†å½¢è¯­éŸ³æ¶ˆæ¯æ°”æ³¡ã€‚

## æ”¯æŒçš„æœåŠ¡

- **ElevenLabs**ï¼ˆä¸»è¦æˆ–å¤‡ç”¨æä¾›å•†ï¼‰
- **OpenAI**ï¼ˆä¸»è¦æˆ–å¤‡ç”¨æä¾›å•†ï¼›ä¹Ÿç”¨äºŽæ‘˜è¦ï¼‰
- **Edge TTS**ï¼ˆä¸»è¦æˆ–å¤‡ç”¨æä¾›å•†ï¼›ä½¿ç”¨ `node-edge-tts`ï¼Œæ—  API å¯†é’¥æ—¶ä¸ºé»˜è®¤ï¼‰

### Edge TTS æ³¨æ„äº‹é¡¹

Edge TTS é€šè¿‡ `node-edge-tts` åº“ä½¿ç”¨ Microsoft Edge çš„åœ¨çº¿ç¥žç»ç½‘ç»œ TTS æœåŠ¡ã€‚å®ƒæ˜¯æ‰˜ç®¡æœåŠ¡ï¼ˆéžæœ¬åœ°ï¼‰ï¼Œä½¿ç”¨ Microsoft çš„ç«¯ç‚¹ï¼Œä¸éœ€è¦ API å¯†é’¥ã€‚`node-edge-tts` å…¬å¼€äº†è¯­éŸ³é…ç½®é€‰é¡¹å’Œè¾“å‡ºæ ¼å¼ï¼Œä½†å¹¶éžæ‰€æœ‰é€‰é¡¹éƒ½è¢« Edge æœåŠ¡æ”¯æŒã€‚citeturn2search0

ç”±äºŽ Edge TTS æ˜¯ä¸€ä¸ªæ²¡æœ‰å…¬å¸ƒ SLA æˆ–é…é¢çš„å…¬å…± Web æœåŠ¡ï¼Œè¯·å°†å…¶è§†ä¸ºå°½åŠ›è€Œä¸ºã€‚å¦‚æžœä½ éœ€è¦æœ‰ä¿è¯çš„é™åˆ¶å’Œæ”¯æŒï¼Œè¯·ä½¿ç”¨ OpenAI æˆ– ElevenLabsã€‚Microsoft çš„è¯­éŸ³ REST API è®°å½•äº†æ¯ä¸ªè¯·æ±‚ 10 åˆ†é’Ÿçš„éŸ³é¢‘é™åˆ¶ï¼›Edge TTS æ²¡æœ‰å…¬å¸ƒé™åˆ¶ï¼Œæ‰€ä»¥å‡è®¾ç±»ä¼¼æˆ–æ›´ä½Žçš„é™åˆ¶ã€‚citeturn0search3

## å¯é€‰å¯†é’¥

å¦‚æžœä½ æƒ³ä½¿ç”¨ OpenAI æˆ– ElevenLabsï¼š

- `ELEVENLABS_API_KEY`ï¼ˆæˆ– `XI_API_KEY`ï¼‰
- `OPENAI_API_KEY`

Edge TTS **ä¸**éœ€è¦ API å¯†é’¥ã€‚å¦‚æžœæ²¡æœ‰æ‰¾åˆ° API å¯†é’¥ï¼ŒKrabKrab é»˜è®¤ä½¿ç”¨ Edge TTSï¼ˆé™¤éžé€šè¿‡ `messages.tts.edge.enabled=false` ç¦ç”¨ï¼‰ã€‚

å¦‚æžœé…ç½®äº†å¤šä¸ªæä¾›å•†ï¼Œé¦–å…ˆä½¿ç”¨é€‰å®šçš„æä¾›å•†ï¼Œå…¶ä»–ä½œä¸ºå¤‡ç”¨é€‰é¡¹ã€‚è‡ªåŠ¨æ‘˜è¦ä½¿ç”¨é…ç½®çš„ `summaryModel`ï¼ˆæˆ– `agents.defaults.model.primary`ï¼‰ï¼Œæ‰€ä»¥å¦‚æžœä½ å¯ç”¨æ‘˜è¦ï¼Œè¯¥æä¾›å•†ä¹Ÿå¿…é¡»ç»è¿‡è®¤è¯ã€‚

## æœåŠ¡é“¾æŽ¥

- [OpenAI æ–‡æœ¬è½¬è¯­éŸ³æŒ‡å—](https://platform.openai.com/docs/guides/text-to-speech)
- [OpenAI éŸ³é¢‘ API å‚è€ƒ](https://platform.openai.com/docs/api-reference/audio)
- [ElevenLabs æ–‡æœ¬è½¬è¯­éŸ³](https://elevenlabs.io/docs/api-reference/text-to-speech)
- [ElevenLabs è®¤è¯](https://elevenlabs.io/docs/api-reference/authentication)
- [node-edge-tts](https://github.com/SchneeHertz/node-edge-tts)
- [Microsoft è¯­éŸ³è¾“å‡ºæ ¼å¼](https://learn.microsoft.com/azure/ai-services/speech-service/rest-text-to-speech#audio-outputs)

## é»˜è®¤å¯ç”¨å—ï¼Ÿ

ä¸æ˜¯ã€‚è‡ªåŠ¨ TTS é»˜è®¤**å…³é—­**ã€‚åœ¨é…ç½®ä¸­ä½¿ç”¨ `messages.tts.auto` æˆ–åœ¨æ¯ä¸ªä¼šè¯ä¸­ä½¿ç”¨ `/tts always`ï¼ˆåˆ«åï¼š`/tts on`ï¼‰å¯ç”¨å®ƒã€‚

ä¸€æ—¦ TTS å¼€å¯ï¼ŒEdge TTS **æ˜¯**é»˜è®¤å¯ç”¨çš„ï¼Œå¹¶åœ¨æ²¡æœ‰ OpenAI æˆ– ElevenLabs API å¯†é’¥æ—¶è‡ªåŠ¨ä½¿ç”¨ã€‚

## é…ç½®

TTS é…ç½®ä½äºŽ `krabkrab.json` ä¸­çš„ `messages.tts` ä¸‹ã€‚å®Œæ•´ schema åœ¨ [Gateway ç½‘å…³é…ç½®](/gateway/configuration)ä¸­ã€‚

### æœ€å°é…ç½®ï¼ˆå¯ç”¨ + æä¾›å•†ï¼‰

```json5
{
  messages: {
    tts: {
      auto: "always",
      provider: "elevenlabs",
    },
  },
}
```

### OpenAI ä¸»è¦ï¼ŒElevenLabs å¤‡ç”¨

```json5
{
  messages: {
    tts: {
      auto: "always",
      provider: "openai",
      summaryModel: "openai/gpt-4.1-mini",
      modelOverrides: {
        enabled: true,
      },
      openai: {
        apiKey: "openai_api_key",
        model: "gpt-4o-mini-tts",
        voice: "alloy",
      },
      elevenlabs: {
        apiKey: "elevenlabs_api_key",
        baseUrl: "https://api.elevenlabs.io",
        voiceId: "voice_id",
        modelId: "eleven_multilingual_v2",
        seed: 42,
        applyTextNormalization: "auto",
        languageCode: "en",
        voiceSettings: {
          stability: 0.5,
          similarityBoost: 0.75,
          style: 0.0,
          useSpeakerBoost: true,
          speed: 1.0,
        },
      },
    },
  },
}
```

### Edge TTS ä¸»è¦ï¼ˆæ—  API å¯†é’¥ï¼‰

```json5
{
  messages: {
    tts: {
      auto: "always",
      provider: "edge",
      edge: {
        enabled: true,
        voice: "en-US-MichelleNeural",
        lang: "en-US",
        outputFormat: "audio-24khz-48kbitrate-mono-mp3",
        rate: "+10%",
        pitch: "-5%",
      },
    },
  },
}
```

### ç¦ç”¨ Edge TTS

```json5
{
  messages: {
    tts: {
      edge: {
        enabled: false,
      },
    },
  },
}
```

### è‡ªå®šä¹‰é™åˆ¶ + åå¥½è·¯å¾„

```json5
{
  messages: {
    tts: {
      auto: "always",
      maxTextLength: 4000,
      timeoutMs: 30000,
      prefsPath: "~/.krabkrab/settings/tts.json",
    },
  },
}
```

### ä»…åœ¨æ”¶åˆ°è¯­éŸ³æ¶ˆæ¯åŽç”¨éŸ³é¢‘å›žå¤

```json5
{
  messages: {
    tts: {
      auto: "inbound",
    },
  },
}
```

### ç¦ç”¨é•¿å›žå¤çš„è‡ªåŠ¨æ‘˜è¦

```json5
{
  messages: {
    tts: {
      auto: "always",
    },
  },
}
```

ç„¶åŽè¿è¡Œï¼š

```
/tts summary off
```

### å­—æ®µè¯´æ˜Ž

- `auto`ï¼šè‡ªåŠ¨ TTS æ¨¡å¼ï¼ˆ`off`ã€`always`ã€`inbound`ã€`tagged`ï¼‰ã€‚
  - `inbound` ä»…åœ¨æ”¶åˆ°è¯­éŸ³æ¶ˆæ¯åŽå‘é€éŸ³é¢‘ã€‚
  - `tagged` ä»…åœ¨å›žå¤åŒ…å« `[[tts]]` æ ‡ç­¾æ—¶å‘é€éŸ³é¢‘ã€‚
- `enabled`ï¼šæ—§ç‰ˆå¼€å…³ï¼ˆdoctor å°†å…¶è¿ç§»åˆ° `auto`ï¼‰ã€‚
- `mode`ï¼š`"final"`ï¼ˆé»˜è®¤ï¼‰æˆ– `"all"`ï¼ˆåŒ…æ‹¬å·¥å…·/åˆ†å—å›žå¤ï¼‰ã€‚
- `provider`ï¼š`"elevenlabs"`ã€`"openai"` æˆ– `"edge"`ï¼ˆè‡ªåŠ¨å¤‡ç”¨ï¼‰ã€‚
- å¦‚æžœ `provider` **æœªè®¾ç½®**ï¼ŒKrabKrab ä¼˜å…ˆé€‰æ‹© `openai`ï¼ˆå¦‚æžœæœ‰å¯†é’¥ï¼‰ï¼Œç„¶åŽæ˜¯ `elevenlabs`ï¼ˆå¦‚æžœæœ‰å¯†é’¥ï¼‰ï¼Œå¦åˆ™æ˜¯ `edge`ã€‚
- `summaryModel`ï¼šç”¨äºŽè‡ªåŠ¨æ‘˜è¦çš„å¯é€‰å»‰ä»·æ¨¡åž‹ï¼›é»˜è®¤ä¸º `agents.defaults.model.primary`ã€‚
  - æŽ¥å— `provider/model` æˆ–é…ç½®çš„æ¨¡åž‹åˆ«åã€‚
- `modelOverrides`ï¼šå…è®¸æ¨¡åž‹å‘å‡º TTS æŒ‡ä»¤ï¼ˆé»˜è®¤å¼€å¯ï¼‰ã€‚
- `maxTextLength`ï¼šTTS è¾“å…¥çš„ç¡¬æ€§ä¸Šé™ï¼ˆå­—ç¬¦ï¼‰ã€‚è¶…å‡ºæ—¶ `/tts audio` ä¼šå¤±è´¥ã€‚
- `timeoutMs`ï¼šè¯·æ±‚è¶…æ—¶ï¼ˆæ¯«ç§’ï¼‰ã€‚
- `prefsPath`ï¼šè¦†ç›–æœ¬åœ°åå¥½ JSON è·¯å¾„ï¼ˆæä¾›å•†/é™åˆ¶/æ‘˜è¦ï¼‰ã€‚
- `apiKey` å€¼å›žé€€åˆ°çŽ¯å¢ƒå˜é‡ï¼ˆ`ELEVENLABS_API_KEY`/`XI_API_KEY`ã€`OPENAI_API_KEY`ï¼‰ã€‚
- `elevenlabs.baseUrl`ï¼šè¦†ç›– ElevenLabs API åŸºç¡€ URLã€‚
- `elevenlabs.voiceSettings`ï¼š
  - `stability`ã€`similarityBoost`ã€`style`ï¼š`0..1`
  - `useSpeakerBoost`ï¼š`true|false`
  - `speed`ï¼š`0.5..2.0`ï¼ˆ1.0 = æ­£å¸¸ï¼‰
- `elevenlabs.applyTextNormalization`ï¼š`auto|on|off`
- `elevenlabs.languageCode`ï¼š2 å­—æ¯ ISO 639-1ï¼ˆä¾‹å¦‚ `en`ã€`de`ï¼‰
- `elevenlabs.seed`ï¼šæ•´æ•° `0..4294967295`ï¼ˆå°½åŠ›ç¡®å®šæ€§ï¼‰
- `edge.enabled`ï¼šå…è®¸ Edge TTS ä½¿ç”¨ï¼ˆé»˜è®¤ `true`ï¼›æ—  API å¯†é’¥ï¼‰ã€‚
- `edge.voice`ï¼šEdge ç¥žç»ç½‘ç»œè¯­éŸ³åç§°ï¼ˆä¾‹å¦‚ `en-US-MichelleNeural`ï¼‰ã€‚
- `edge.lang`ï¼šè¯­è¨€ä»£ç ï¼ˆä¾‹å¦‚ `en-US`ï¼‰ã€‚
- `edge.outputFormat`ï¼šEdge è¾“å‡ºæ ¼å¼ï¼ˆä¾‹å¦‚ `audio-24khz-48kbitrate-mono-mp3`ï¼‰ã€‚
  - æœ‰æ•ˆå€¼å‚è§ Microsoft è¯­éŸ³è¾“å‡ºæ ¼å¼ï¼›å¹¶éžæ‰€æœ‰æ ¼å¼éƒ½è¢« Edge æ”¯æŒã€‚
- `edge.rate` / `edge.pitch` / `edge.volume`ï¼šç™¾åˆ†æ¯”å­—ç¬¦ä¸²ï¼ˆä¾‹å¦‚ `+10%`ã€`-5%`ï¼‰ã€‚
- `edge.saveSubtitles`ï¼šåœ¨éŸ³é¢‘æ–‡ä»¶æ—è¾¹å†™å…¥ JSON å­—å¹•ã€‚
- `edge.proxy`ï¼šEdge TTS è¯·æ±‚çš„ä»£ç† URLã€‚
- `edge.timeoutMs`ï¼šè¯·æ±‚è¶…æ—¶è¦†ç›–ï¼ˆæ¯«ç§’ï¼‰ã€‚

## æ¨¡åž‹é©±åŠ¨è¦†ç›–ï¼ˆé»˜è®¤å¼€å¯ï¼‰

é»˜è®¤æƒ…å†µä¸‹ï¼Œæ¨¡åž‹**å¯ä»¥**ä¸ºå•ä¸ªå›žå¤å‘å‡º TTS æŒ‡ä»¤ã€‚å½“ `messages.tts.auto` ä¸º `tagged` æ—¶ï¼Œéœ€è¦è¿™äº›æŒ‡ä»¤æ¥è§¦å‘éŸ³é¢‘ã€‚

å¯ç”¨åŽï¼Œæ¨¡åž‹å¯ä»¥å‘å‡º `[[tts:...]]` æŒ‡ä»¤æ¥è¦†ç›–å•ä¸ªå›žå¤çš„è¯­éŸ³ï¼ŒåŠ ä¸Šå¯é€‰çš„ `[[tts:text]]...[[/tts:text]]` å—æ¥æä¾›è¡¨è¾¾æ€§æ ‡ç­¾ï¼ˆç¬‘å£°ã€å”±æ­Œæç¤ºç­‰ï¼‰ï¼Œè¿™äº›ä»…åº”å‡ºçŽ°åœ¨éŸ³é¢‘ä¸­ã€‚

ç¤ºä¾‹å›žå¤è´Ÿè½½ï¼š

```
Here you go.

[[tts:provider=elevenlabs voiceId=pMsXgVXv3BLzUgSXRplE model=eleven_v3 speed=1.1]]
[[tts:text]](laughs) Read the song once more.[[/tts:text]]
```

å¯ç”¨æŒ‡ä»¤é”®ï¼ˆå¯ç”¨æ—¶ï¼‰ï¼š

- `provider`ï¼ˆ`openai` | `elevenlabs` | `edge`ï¼‰
- `voice`ï¼ˆOpenAI è¯­éŸ³ï¼‰æˆ– `voiceId`ï¼ˆElevenLabsï¼‰
- `model`ï¼ˆOpenAI TTS æ¨¡åž‹æˆ– ElevenLabs æ¨¡åž‹ IDï¼‰
- `stability`ã€`similarityBoost`ã€`style`ã€`speed`ã€`useSpeakerBoost`
- `applyTextNormalization`ï¼ˆ`auto|on|off`ï¼‰
- `languageCode`ï¼ˆISO 639-1ï¼‰
- `seed`

ç¦ç”¨æ‰€æœ‰æ¨¡åž‹è¦†ç›–ï¼š

```json5
{
  messages: {
    tts: {
      modelOverrides: {
        enabled: false,
      },
    },
  },
}
```

å¯é€‰ç™½åå•ï¼ˆç¦ç”¨ç‰¹å®šè¦†ç›–åŒæ—¶ä¿æŒæ ‡ç­¾å¯ç”¨ï¼‰ï¼š

```json5
{
  messages: {
    tts: {
      modelOverrides: {
        enabled: true,
        allowProvider: false,
        allowSeed: false,
      },
    },
  },
}
```

## å•ç”¨æˆ·åå¥½

æ–œæ å‘½ä»¤å°†æœ¬åœ°è¦†ç›–å†™å…¥ `prefsPath`ï¼ˆé»˜è®¤ï¼š`~/.krabkrab/settings/tts.json`ï¼Œå¯é€šè¿‡ `krabkrab_TTS_PREFS` æˆ– `messages.tts.prefsPath` è¦†ç›–ï¼‰ã€‚

å­˜å‚¨çš„å­—æ®µï¼š

- `enabled`
- `provider`
- `maxLength`ï¼ˆæ‘˜è¦é˜ˆå€¼ï¼›é»˜è®¤ 1500 å­—ç¬¦ï¼‰
- `summarize`ï¼ˆé»˜è®¤ `true`ï¼‰

è¿™äº›ä¸ºè¯¥ä¸»æœºè¦†ç›– `messages.tts.*`ã€‚

## è¾“å‡ºæ ¼å¼ï¼ˆå›ºå®šï¼‰

- **Telegram**ï¼šOpus è¯­éŸ³æ¶ˆæ¯ï¼ˆElevenLabs çš„ `opus_48000_64`ï¼ŒOpenAI çš„ `opus`ï¼‰ã€‚
  - 48kHz / 64kbps æ˜¯è¯­éŸ³æ¶ˆæ¯çš„è‰¯å¥½æƒè¡¡ï¼Œåœ†å½¢æ°”æ³¡æ‰€å¿…éœ€ã€‚
- **å…¶ä»–æ¸ é“**ï¼šMP3ï¼ˆElevenLabs çš„ `mp3_44100_128`ï¼ŒOpenAI çš„ `mp3`ï¼‰ã€‚
  - 44.1kHz / 128kbps æ˜¯è¯­éŸ³æ¸…æ™°åº¦çš„é»˜è®¤å¹³è¡¡ã€‚
- **Edge TTS**ï¼šä½¿ç”¨ `edge.outputFormat`ï¼ˆé»˜è®¤ `audio-24khz-48kbitrate-mono-mp3`ï¼‰ã€‚
  - `node-edge-tts` æŽ¥å— `outputFormat`ï¼Œä½†å¹¶éžæ‰€æœ‰æ ¼å¼éƒ½å¯ä»Ž Edge æœåŠ¡èŽ·å¾—ã€‚citeturn2search0
  - è¾“å‡ºæ ¼å¼å€¼éµå¾ª Microsoft è¯­éŸ³è¾“å‡ºæ ¼å¼ï¼ˆåŒ…æ‹¬ Ogg/WebM Opusï¼‰ã€‚citeturn1search0
  - Telegram `sendVoice` æŽ¥å— OGG/MP3/M4Aï¼›å¦‚æžœä½ éœ€è¦æœ‰ä¿è¯çš„ Opus è¯­éŸ³æ¶ˆæ¯ï¼Œè¯·ä½¿ç”¨ OpenAI/ElevenLabsã€‚citeturn1search1
  - å¦‚æžœé…ç½®çš„ Edge è¾“å‡ºæ ¼å¼å¤±è´¥ï¼ŒKrabKrab ä¼šä½¿ç”¨ MP3 é‡è¯•ã€‚

OpenAI/ElevenLabs æ ¼å¼æ˜¯å›ºå®šçš„ï¼›Telegram æœŸæœ› Opus ä»¥èŽ·å¾—è¯­éŸ³æ¶ˆæ¯ç”¨æˆ·ä½“éªŒã€‚

## è‡ªåŠ¨ TTS è¡Œä¸º

å¯ç”¨åŽï¼ŒKrabKrabï¼š

- å¦‚æžœå›žå¤å·²åŒ…å«åª’ä½“æˆ– `MEDIA:` æŒ‡ä»¤ï¼Œåˆ™è·³è¿‡ TTSã€‚
- è·³è¿‡éžå¸¸çŸ­çš„å›žå¤ï¼ˆ< 10 å­—ç¬¦ï¼‰ã€‚
- å¯ç”¨æ—¶ä½¿ç”¨ `agents.defaults.model.primary`ï¼ˆæˆ– `summaryModel`ï¼‰å¯¹é•¿å›žå¤è¿›è¡Œæ‘˜è¦ã€‚
- å°†ç”Ÿæˆçš„éŸ³é¢‘é™„åŠ åˆ°å›žå¤ä¸­ã€‚

å¦‚æžœå›žå¤è¶…è¿‡ `maxLength` ä¸”æ‘˜è¦å…³é—­ï¼ˆæˆ–æ²¡æœ‰æ‘˜è¦æ¨¡åž‹çš„ API å¯†é’¥ï¼‰ï¼Œåˆ™è·³è¿‡éŸ³é¢‘å¹¶å‘é€æ­£å¸¸çš„æ–‡æœ¬å›žå¤ã€‚

## æµç¨‹å›¾

```
å›žå¤ -> TTS å¯ç”¨ï¼Ÿ
  å¦  -> å‘é€æ–‡æœ¬
  æ˜¯  -> æœ‰åª’ä½“ / MEDIA: / å¤ªçŸ­ï¼Ÿ
          æ˜¯ -> å‘é€æ–‡æœ¬
          å¦ -> é•¿åº¦ > é™åˆ¶ï¼Ÿ
                   å¦  -> TTS -> é™„åŠ éŸ³é¢‘
                   æ˜¯  -> æ‘˜è¦å¯ç”¨ï¼Ÿ
                            å¦  -> å‘é€æ–‡æœ¬
                            æ˜¯  -> æ‘˜è¦ï¼ˆsummaryModel æˆ– agents.defaults.model.primaryï¼‰
                                      -> TTS -> é™„åŠ éŸ³é¢‘
```

## æ–œæ å‘½ä»¤ç”¨æ³•

åªæœ‰ä¸€ä¸ªå‘½ä»¤ï¼š`/tts`ã€‚å‚è§[æ–œæ å‘½ä»¤](/tools/slash-commands)äº†è§£å¯ç”¨è¯¦æƒ…ã€‚

Discord æ³¨æ„ï¼š`/tts` æ˜¯ Discord çš„å†…ç½®å‘½ä»¤ï¼Œæ‰€ä»¥ KrabKrab åœ¨é‚£é‡Œæ³¨å†Œ `/voice` ä½œä¸ºåŽŸç”Ÿå‘½ä»¤ã€‚æ–‡æœ¬ `/tts ...` ä»ç„¶æœ‰æ•ˆã€‚

```
/tts off
/tts always
/tts inbound
/tts tagged
/tts status
/tts provider openai
/tts limit 2000
/tts summary off
/tts audio Hello from KrabKrab
```

æ³¨æ„äº‹é¡¹ï¼š

- å‘½ä»¤éœ€è¦æŽˆæƒå‘é€è€…ï¼ˆç™½åå•/æ‰€æœ‰è€…è§„åˆ™ä»ç„¶é€‚ç”¨ï¼‰ã€‚
- å¿…é¡»å¯ç”¨ `commands.text` æˆ–åŽŸç”Ÿå‘½ä»¤æ³¨å†Œã€‚
- `off|always|inbound|tagged` æ˜¯å•ä¼šè¯å¼€å…³ï¼ˆ`/tts on` æ˜¯ `/tts always` çš„åˆ«åï¼‰ã€‚
- `limit` å’Œ `summary` å­˜å‚¨åœ¨æœ¬åœ°åå¥½ä¸­ï¼Œä¸åœ¨ä¸»é…ç½®ä¸­ã€‚
- `/tts audio` ç”Ÿæˆä¸€æ¬¡æ€§éŸ³é¢‘å›žå¤ï¼ˆä¸ä¼šå¼€å¯ TTSï¼‰ã€‚

## æ™ºèƒ½ä½“å·¥å…·

`tts` å·¥å…·å°†æ–‡æœ¬è½¬æ¢ä¸ºè¯­éŸ³å¹¶è¿”å›ž `MEDIA:` è·¯å¾„ã€‚å½“ç»“æžœä¸Ž Telegram å…¼å®¹æ—¶ï¼Œå·¥å…·åŒ…å« `[[audio_as_voice]]`ï¼Œä»¥ä¾¿ Telegram å‘é€è¯­éŸ³æ°”æ³¡ã€‚

## Gateway ç½‘å…³ RPC

Gateway ç½‘å…³æ–¹æ³•ï¼š

- `tts.status`
- `tts.enable`
- `tts.disable`
- `tts.convert`
- `tts.setProvider`
- `tts.providers`


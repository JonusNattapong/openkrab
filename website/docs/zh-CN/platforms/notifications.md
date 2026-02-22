---
summary: "跨 macOS、Windows 和 Linux 启用原生桌面通知"
read_when:
  - 你想通过 OpenClaw 接收系统桌面通知
  - 你需要调试桌面通知
title: "桌面通知"
---

# 桌面通知

OpenClaw 支持跨 macOS、Windows 和 Linux 的原生系统桌面通知（基于 Rust 的 `notify-rust` 库）。当启用功能或有入站事件（如来自频道系统的消息）时，通知系统便会直接向你的桌面弹出附带声音的原生提示。

## 支持平台

- **macOS**：触发标准的 macOS 桌面通知中心。
- **Windows**：支持触发 Windows Action Center 事件中心通知（在幕后使用 `tauri-winrt-notification`）。
- **Linux**：支持标准的 D-Bus 通知守护程序（如 `notify-osd`、`dunst`、`gnome-shell` 等）。

## 配置

通知功能通常无需手动配置，将在使用控制台或 Agent 想使用工具主动推送通知时触发。

可以通过命令行测试系统是否工作：

```bash
openclaw system notify --title "Test" --body "Hello from OpenClaw!"
```

## 通知声音说明

跨平台触发带有声音的通知时，每个平台支持的声音 ID 各不相同：

- **macOS**: `Ping`、`Basso`、`Blow`、`Bottle`、`Frog`、`Funk`、`Glass`、`Hero`、`Morse`、`Purr`、`Sosumi`、`Submarine`、`Tink`。
- **Windows**: `default`、`IM`、`Mail`、`Reminder`、`SMS`、`Alarm`。
- **Linux**: 取决于你桌面环境下的 freedesktop 声音主题（例如 `message-new-instant`、`bell` 等）。

*(在默认情况下，即使未找到平台下对应的特定音频名称，OpenClaw 通知仍然会弹出并尝试进行平滑回退处理。)*

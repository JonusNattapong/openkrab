import AppKit
import Combine
import Foundation

final class MenuBarController {
    private let state: MacAssistantState

    private let statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)
    private let menu = NSMenu()
    private let statusMenuItem = NSMenuItem(title: "Status: idle", action: nil, keyEquivalent: "")
    private let toggleTalkItem = NSMenuItem(title: "Enable Talk Mode", action: nil, keyEquivalent: "t")
    private let permissionsItem = NSMenuItem(title: "Request Permissions", action: nil, keyEquivalent: "p")

    private var cancellables = Set<AnyCancellable>()

    init(state: MacAssistantState) {
        self.state = state
    }

    func install() {
        if let button = statusItem.button {
            button.title = "KR"
            button.toolTip = "KrabKrab Voice Wake/Talk"
        }

        statusMenuItem.isEnabled = false
        menu.addItem(statusMenuItem)
        menu.addItem(NSMenuItem.separator())

        toggleTalkItem.target = self
        toggleTalkItem.action = #selector(toggleTalkMode)
        menu.addItem(toggleTalkItem)

        permissionsItem.target = self
        permissionsItem.action = #selector(requestPermissions)
        menu.addItem(permissionsItem)

        let settingsItem = NSMenuItem(title: "Open Settings", action: #selector(openSettings), keyEquivalent: ",")
        settingsItem.target = self
        menu.addItem(settingsItem)

        menu.addItem(NSMenuItem.separator())

        let quitItem = NSMenuItem(title: "Quit", action: #selector(quit), keyEquivalent: "q")
        quitItem.target = self
        menu.addItem(quitItem)

        statusItem.menu = menu
        bindState()
    }

    private func bindState() {
        state.$statusText
            .receive(on: DispatchQueue.main)
            .sink { [weak self] value in
                self?.statusMenuItem.title = "Status: \(value)"
            }
            .store(in: &cancellables)

        state.$talkModeEnabled
            .receive(on: DispatchQueue.main)
            .sink { [weak self] enabled in
                guard let self else { return }
                self.toggleTalkItem.title = enabled ? "Disable Talk Mode" : "Enable Talk Mode"
                self.statusItem.button?.title = enabled ? "KR*" : "KR"
            }
            .store(in: &cancellables)
    }

    @objc
    private func toggleTalkMode() {
        state.toggleTalkMode()
    }

    @objc
    private func requestPermissions() {
        Task { await state.requestRequiredPermissions() }
    }

    @objc
    private func openSettings() {
        NSApp.activate(ignoringOtherApps: true)
        if !NSApp.sendAction(Selector(("showSettingsWindow:")), to: nil, from: nil) {
            _ = NSApp.sendAction(Selector(("showPreferencesWindow:")), to: nil, from: nil)
        }
    }

    @objc
    private func quit() {
        NSApp.terminate(nil)
    }
}

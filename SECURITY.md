# Security Policy

Aether is experimental and is not yet safe for routine browsing. It currently lacks renderer sandboxing, site isolation, a complete origin model, a mature certificate UI, and a security review. Do not use it for sensitive accounts.

Password management is defined behind `SecretStore`, so the browser model never chooses plaintext persistence by itself. Production integration must provide macOS Keychain, Linux Secret Service/KWallet, or Windows Credential Manager (or an audited encrypted store); the current test provider is mock-only and must never ship.

Report vulnerabilities privately to the project maintainers before public disclosure. Never include passwords, tokens, or private browsing data in a report.

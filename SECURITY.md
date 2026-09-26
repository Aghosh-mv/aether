# Security Policy

Aether is experimental and is not yet safe for routine browsing. It currently lacks renderer sandboxing, site isolation, a complete origin model, a mature certificate UI, and a security review. Do not use it for sensitive accounts.

Password management is intentionally not implemented as plaintext storage. The eventual credential vault must use OS-secure storage or an audited encryption design; the current browser state work only covers non-secret preferences and cookie matching behavior.

Report vulnerabilities privately to the project maintainers before public disclosure. Never include passwords, tokens, or private browsing data in a report.

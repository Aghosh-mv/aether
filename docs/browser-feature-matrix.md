# Aether normal-browser feature matrix

This matrix is the working definition of a full everyday browser. It is derived from the official Chrome, Firefox, Edge, and Brave feature surfaces and is intentionally separate from the engine roadmap.

Status meanings:

- `FOUNDATION` — a tested model or engine primitive exists, but it is not necessarily wired to the desktop UI or OS.
- `PARTIAL` — some behavior exists, but important workflows or platform integration are missing.
- `PLANNED` — required for the normal-browser milestone and not yet implemented.
- `EVIDENCE REQUIRED` — cannot be called complete until tested against the real workflow.

| Area | Required everyday capabilities | Aether status |
|---|---|---|
| Navigation | URL loading, redirects, back/forward, reload/stop, link activation, multiple windows | HTTPS loading; history model `FOUNDATION`; rest `PLANNED` |
| Tabs | New/close/select, pinned, muted, reopen closed, groups, search, vertical/split options | Tested tab model `FOUNDATION`; UI and groups `PLANNED` |
| Omnibox | URL/search classification, suggestions, history/bookmarks/open tabs, custom engines, commands | Classification and custom search URL `FOUNDATION`; suggestions/UI `PLANNED` |
| Profiles/accounts | Normal profiles, guest, incognito, sign in/out, account creation state, per-profile data | Profile/account state model `FOUNDATION`; real browser account flow `EVIDENCE REQUIRED` |
| Cookies/storage | Cookies, expiry, secure/site matching, cache, local/session storage, IndexedDB, clear-site-data | Cookie matching `FOUNDATION`; persistence and web storage `PLANNED` |
| History | Search, per-site removal, time-range clearing, recently closed, private isolation | Tested history model `FOUNDATION`; UI/session integration `PLANNED` |
| Bookmarks | Add/edit/delete, folders, search, bookmark bar, import/export | Tested bookmark store `FOUNDATION`; UI/import/export `PLANNED` |
| Passwords | OS-secure vault, save/fill, generation, editing, deletion, import/export, breach warnings | Secure-store contract `FOUNDATION`; OS adapters/autofill `PLANNED` |
| Autofill | Addresses, names, emails, phone numbers, safe payment design | `PLANNED` |
| Downloads | Progress, pause/resume, retry, cancel, complete, history, destination, dangerous-file warnings | Tested lifecycle model `FOUNDATION`; file/OS integration `PLANNED` |
| Permissions | Camera, microphone, location, notifications, clipboard, downloads, popups, screen capture | Per-origin decision model `FOUNDATION`; prompts/OS APIs `PLANNED` |
| Media | Audio/video, captions, fullscreen, PiP, speed, streaming, MSE, WebRTC | `PLANNED` |
| Privacy/security | HTTPS-first, certificates, Safe Browsing boundary, tracker blocking, cookie controls, CSP/CORS, sandbox/site isolation | Security policy and permission boundary only; `PLANNED` |
| Extensions | Manifest subset, content scripts, permissions, install/disable/update, signed packages | `PLANNED` |
| PWAs | Install site as app, isolated storage, notifications, offline/service workers, handlers | `PLANNED` |
| DevTools | Elements, styles, console, network, storage, performance, accessibility, security | `PLANNED` |
| Settings | Theme, startup, search engine, privacy, security, permissions, downloads, languages, accessibility, profiles | Settings data model `FOUNDATION`; settings UI `PLANNED` |
| Personalization | Themes, new-tab shortcuts/background, reduced motion, zoom/text scaling, toolbar layout | Theme/homepage/reduced-motion data `FOUNDATION`; UI `PLANNED` |
| Sync | Optional E2E-encrypted bookmarks/history/tabs/settings/extensions/passwords | `PLANNED`; must remain opt-in and local-only capable |
| Accessibility | Keyboard browsing, screen readers, ARIA tree, focus, high contrast, reduced motion, zoom | `PLANNED` |
| Internationalization | UI languages, Unicode URLs, RTL, IME, regional formats, translation | `PLANNED` |
| Files/printing | HTML/TXT/images/SVG/JSON/XML, PDF, save page, print preview, print-to-PDF | `PLANNED` |
| Reliability/updates | Crash isolation, session restore, signed updates, rollback, diagnostics consent | `PLANNED` |

## Research basis

The matrix reflects official feature documentation: Chrome documents profiles, downloads, camera/microphone, tabs, dark mode, languages, sync, privacy, Incognito, passwords, and extensions; Firefox documents Sync for bookmarks, passwords, logins, addresses, add-ons, settings, and open tabs plus HTTPS-only, tracking protection, breach monitoring, and containers; Brave documents Shields, search, sync, extensions, sidebar, translation, privacy controls, and related services.

This matrix is not a claim that Aether currently provides those features. A feature moves to complete only after implementation, workflow tests, and platform evidence exist.

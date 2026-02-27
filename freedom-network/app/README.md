# Freedom Desktop App (Tauri)

Native desktop control surface for Freedom Network VPN/proxy runtime.

## Current Scope

- Desktop-first VPN UX (connect/disconnect, live status, stats)
- Mode selection (`Full System` / `Single App`)
- Location + protocol options
- Kill switch, threat protection, DNS protection
- Auto-connect rules
- Split tunneling app list with apply/stop controls
- Diagnostics checks (dashboard/proxy/QUIC reachability)
- Profile export/import (JSON)
- Optional strict Windows interception mode (firewall-based)

## Runtime Dependencies

- Freedom node binary: `freedom-node(.exe)`
- Dashboard API: `127.0.0.1:9090`
- Local proxy: `127.0.0.1:8080`

## Development

```bash
cd app/src-tauri
cargo tauri dev
```

## Build

```bash
cd app/src-tauri
cargo tauri build
```

Windows all-in-one packaging is handled by:

```bat
scripts\windows-all-in-one-installer.bat
```

## Important Notes

- `System Intercept (Windows)` uses firewall rule changes and may require elevated permissions.
- Split tunneling currently enforces routing for processes launched/managed by the app.
- Full packet-level VPN tunneling (Wintun/TUN) is not yet implemented in this repository.

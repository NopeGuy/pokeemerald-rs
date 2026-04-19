# IPMX Presentation on GBA — Setup Guide

## What you need

| Item | Notes |
|------|-------|
| `pokeemerald.gba` | Built ROM — run `make` in the repo root |
| mGBA | Any version with Lua scripting support (Tools → Scripting) |
| `nmos_bridge.lua` | Lua bridge loaded inside mGBA |
| `nmos_fetch.py` | Python 3.10+ script run on the host machine |
| NMOS registry | Easy NMOS or any IS-04 / IS-05 compatible registry |
| IPMX sender | Label must contain **"presentation"** (case-insensitive) |
| IPMX receiver | Label must contain **"presentation"** (case-insensitive) |

---

## Quick start

### 1. Build the ROM
```bash
make
```
Output: `pokeemerald.gba`

### 2. Launch mGBA
Open the ROM normally (no special flags needed).

### 3. Load the Lua bridge
In mGBA: **Tools → Scripting → Open script** → select `nmos_bridge.lua`.

The script console should print:
```
[nmos_bridge] Loaded. Watching /tmp/nmos_slide.bin
```

### 4. Run the Python bridge
```bash
python3 nmos_fetch.py
```

It will start polling the NMOS registry every 5 seconds and printing the found sender/receiver labels.

---

## Configuration

Open `nmos_fetch.py` and adjust the constants at the top:

```python
NMOS_QUERY      = "http://10.10.30.226:8870/x-nmos/query/v1.3"   # IS-04 query API
NMOS_CONN       = "http://10.10.30.226:8870/x-nmos/connection/v1.1" # IS-05 fallback
SENDER_FILTER   = "presentation"   # sender label must contain this (case-insensitive)
RECEIVER_FILTER = "presentation"   # receiver label must contain this (case-insensitive)
POLL_SEC        = 5                # how often to re-fetch NMOS resources (seconds)
```

- **`NMOS_QUERY`** — point this at your registry's IS-04 query API.
- **`NMOS_CONN`** — fallback IS-05 base; normally the script discovers the real endpoint via device controls.
- **`SENDER_FILTER` / `RECEIVER_FILTER`** — the script picks the first sender/receiver whose label contains this string. Change it if your device labels use a different keyword.
- **`POLL_SEC`** — increase if the registry is slow; decrease if you need faster label updates.

---

## How it works end-to-end

```
NMOS Registry
     │  IS-04 poll (every 5 s)
     ▼
nmos_fetch.py  ──writes──►  /tmp/nmos_slide.bin  ◄──reads── nmos_bridge.lua
                                                                    │
                                                             writes EWRAM
                                                             (0x02020000)
                                                                    │
                                                              GBA ROM reads
                                                           sender/receiver labels
                                                           and shows them on screen
```

### NMOS live demo slide (IS-04)
When the **AMWA NMOS** option is selected in the hub menu, the presentation shows
sender and receiver labels read live from EWRAM.
The Python script must be running and a matching sender + receiver must be visible
in the registry for the labels to appear.

### Animation slide — IS-05 connection (triggered by the GBA)
On the *"Another use case?"* slide, an envelope flies from Nidoran M to Nidoran F.
The moment the envelope lands, the ROM writes a connect command to EWRAM.

```
ROM sets EWRAM_CMD = 1
        │
        ▼ (within ~1 second)
nmos_bridge.lua reads EWRAM_CMD, writes /tmp/nmos_connect, clears EWRAM_CMD
        │
        ▼ (within ~0.5 seconds)
nmos_fetch.py sees /tmp/nmos_connect, deletes it, calls connect_is05():
  1. Fetches sender manifest_href (SDP) to get multicast_ip + destination_port
  2. Discovers receiver IS-05 endpoint via IS-04 device controls
  3. PATCHes receiver /staged with sender_id + transport_params + SDP
```

---

## EWRAM memory map

The Lua script and ROM share this layout at `0x02020000`:

| Offset | Size | Direction | Content |
|--------|------|-----------|---------|
| `+0x00` | 1 B | Python → ROM | Ready flag (`1` = data valid) |
| `+0x01` | 32 B | Python → ROM | Sender label (GBA charmap, `0xFF`-terminated) |
| `+0x21` | 32 B | Python → ROM | Receiver label (GBA charmap, `0xFF`-terminated) |
| `+0x41` | 1 B | ROM → Python | Command (`0` = none, `1` = connect) |
| `+0x42` | 1 B | reserved | Status (unused in current version) |

---

## IPMX sender / receiver requirements

- Both devices must be **registered in the IS-04 registry**.
- Their **labels** must contain the filter keyword (`"presentation"` by default).
- The **receiver** must expose an IS-05 connection API discoverable via its device
  `controls` array (type `urn:x-nmos:control:sr-ctrl/v1.x`).
- The **sender** must expose a `manifest_href` pointing to a valid SDP file so the
  script can retrieve the multicast address and port for the receiver PATCH.
- The sender used in development:
  - Label: `Presentation IPMX Video Sender Luis`
  - Multicast: `239.10.10.11:5006`
  - Transport: `urn:x-nmos:transport:rtp.mcast`

---

## Troubleshooting

| Symptom | Likely cause | Fix |
|---------|-------------|-----|
| ROM shows black screen on NMOS slide | Python script not running or no matching resources | Start `nmos_fetch.py`; check filter keywords match device labels |
| "Waiting for NMOS bridge…" never clears | Lua script not loaded or `/tmp/nmos_slide.bin` not being written | Load `nmos_bridge.lua` via Tools → Scripting in mGBA |
| IS-05 PATCH returns 404 | IS-05 endpoint not in device controls | Check registry for device controls; update `NMOS_CONN` fallback |
| IS-05 PATCH returns 400 | Schema mismatch in transport_params | Check SDP at `manifest_href` is reachable; review printed transport_params |
| Connection activated but no video | `transport_file` SDP empty | Ensure sender `manifest_href` is reachable from the machine running `nmos_fetch.py` |
| `/tmp/nmos_connect` never appears | Lua script not detecting EWRAM_CMD | Make sure the animation slide was reached and the ROM built with latest changes |

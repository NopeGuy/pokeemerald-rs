#!/usr/bin/env python3
"""Poll NMOS API for 'presentation' sender/receiver and write to /tmp/nmos_slide.bin.

Used with nmos_bridge.lua (Lua approach).
Also watches /tmp/nmos_connect (written by the Lua bridge when the GBA signals
a connection request) and triggers the IS-05 connection automatically.
"""

import os
import time
import urllib.request
import urllib.error
import json

NMOS_QUERY      = "http://10.10.30.226:8870/x-nmos/query/v1.3"
NMOS_CONN       = "http://10.10.30.226:8870/x-nmos/connection/v1.1"
SENDER_FILTER   = "presentation"
RECEIVER_FILTER = "presentation"
OUT_FILE        = "/tmp/nmos_slide.bin"
CONNECT_FILE    = "/tmp/nmos_connect"   # trigger written by nmos_bridge.lua
POLL_SEC        = 5


def gba_encode_char(c: str) -> int:
    b = ord(c)
    if ord('a') <= b <= ord('z'):
        return b - ord('a') + 0xd5
    if ord('A') <= b <= ord('Z'):
        return b - ord('A') + 0xbb
    if ord('0') <= b <= ord('9'):
        return b - ord('0') + 0xa1
    return {
        ' ': 0x00, '!': 0xAB, '?': 0xAC, '.': 0xAD, '-': 0xAE,
        '_': 0xAE, ':': 0xF0, '>': 0x86, '<': 0x85, ')': 0x5D,
        '(': 0x5C, ',': 0xB8, '=': 0x35, '+': 0x2E, '&': 0x2D,
        '/': 0xBA, '"': 0xB1, "'": 0xB4, '\n': 0xFE,
    }.get(c, 0xAE)


def encode_slot(label: str, size: int = 32) -> bytes:
    label = label[:size - 1]
    out = bytearray(size)
    for i, ch in enumerate(label):
        out[i] = gba_encode_char(ch)
    out[len(label)] = 0xFF
    return bytes(out)


def fetch_json(url: str):
    try:
        with urllib.request.urlopen(url, timeout=4) as resp:
            return json.loads(resp.read())
    except Exception as e:
        print(f"[nmos_fetch] {url}: {e}")
        return None


def get_label(node: dict) -> str:
    return str(node.get("label") or node.get("description") or node.get("id", "")).strip()


def find_first(resources: list, keyword: str) -> dict | None:
    kw = keyword.lower()
    for r in resources:
        if kw in get_label(r).lower():
            return r
    return None


def find_is05_base(resource: dict) -> str | None:
    """Discover the IS-05 connection API base URL via a resource's device controls."""
    device_id = resource.get("device_id")
    if not device_id:
        return None
    devices = fetch_json(f"{NMOS_QUERY}/devices") or []
    device  = next((d for d in devices if d["id"] == device_id), None)
    if not device:
        print(f"[nmos_fetch] device {device_id} not found in registry")
        return None
    for ctrl in device.get("controls", []):
        if "sr-ctrl" in ctrl.get("type", ""):
            base = ctrl["href"].rstrip("/")
            print(f"[nmos_fetch] IS-05 control: {base}")
            return base
    print(f"[nmos_fetch] no IS-05 control found on device {device_id}")
    print(f"[nmos_fetch] device controls: {device.get('controls', [])}")
    return None


def connect_is05(sender: dict, receiver: dict) -> bool:
    sender_id   = sender["id"]
    receiver_id = receiver["id"]

    # Derive transport params from the sender's SDP manifest (IS-04 manifest_href).
    # The sender's IS-05 active endpoint is not available on this device, but the
    # SDP reliably contains multicast_ip and destination_port.
    transport_params = [{"interface_ip": "auto"}]
    sdp_text = ""
    manifest_href = sender.get("manifest_href")
    if manifest_href:
        try:
            with urllib.request.urlopen(manifest_href, timeout=4) as resp:
                sdp_text = resp.read().decode(errors="replace")
            multicast_ip = None
            destination_port = None
            for line in sdp_text.splitlines():
                if line.startswith("c=") and multicast_ip is None:
                    # c=IN IP4 239.10.10.11/32  or  c=IN IP4 239.10.10.11
                    parts = line.split()[-1].split("/")
                    ip = parts[0]
                    if ip.startswith("2"):   # multicast range 224-239
                        multicast_ip = ip
                elif line.startswith("m=") and destination_port is None:
                    # m=video 5006 RTP/AVP 97
                    destination_port = int(line.split()[1])
            leg: dict = {"interface_ip": "auto"}
            if multicast_ip:
                leg["multicast_ip"] = multicast_ip
            if destination_port:
                leg["destination_port"] = destination_port
            transport_params = [leg]
            print(f"[nmos_fetch] transport_params from SDP: {transport_params}")
        except Exception as e:
            print(f"[nmos_fetch] SDP fetch failed ({manifest_href}): {e}")

    conn_base = find_is05_base(receiver) or NMOS_CONN
    print(f"[nmos_fetch] PATCH {conn_base}/single/receivers/{receiver_id}/staged")
    url = f"{conn_base}/single/receivers/{receiver_id}/staged"

    body = {
        "sender_id": sender_id,
        "master_enable": True,
        "activation": {"mode": "activate_immediate"},
        "transport_params": transport_params,
        "transport_file": {"data": sdp_text, "type": "application/sdp"},
    }
    payload = json.dumps(body).encode()
    req = urllib.request.Request(
        url, data=payload, method="PATCH",
        headers={"Content-Type": "application/json"},
    )
    try:
        with urllib.request.urlopen(req, timeout=6) as resp:
            code = resp.getcode()
            print(f"[nmos_fetch] IS-05 PATCH {url} -> HTTP {code}")
            return code in (200, 202)
    except urllib.error.HTTPError as e:
        print(f"[nmos_fetch] IS-05 PATCH failed HTTP {e.code}: {e.read().decode(errors='replace')}")
        return False
    except Exception as e:
        print(f"[nmos_fetch] IS-05 PATCH error: {e}")
        return False


def main():
    print(f"[nmos_fetch] Polling {NMOS_QUERY} -> {OUT_FILE}")
    print(f"[nmos_fetch] sender filter:   '{SENDER_FILTER}'")
    print(f"[nmos_fetch] receiver filter: '{RECEIVER_FILTER}'")

    sender   = None
    receiver = None
    last_fetch = 0.0

    while True:
        now = time.monotonic()

        if now - last_fetch >= POLL_SEC:
            last_fetch = now
            senders_all   = fetch_json(f"{NMOS_QUERY}/senders")   or []
            receivers_all = fetch_json(f"{NMOS_QUERY}/receivers") or []
            new_sender   = find_first(senders_all,   SENDER_FILTER)
            new_receiver = find_first(receivers_all, RECEIVER_FILTER)

            if new_sender and new_receiver:
                sender, receiver = new_sender, new_receiver
                s_label = get_label(sender)
                r_label = get_label(receiver)
                print(f"[nmos_fetch] sender:   {s_label!r}")
                print(f"[nmos_fetch] receiver: {r_label!r}")
                buf = bytearray(67)
                buf[0] = 1
                buf[1:33]  = encode_slot(s_label)
                buf[33:65] = encode_slot(r_label)
                with open(OUT_FILE, "wb") as f:
                    f.write(buf)
                print(f"[nmos_fetch] Wrote {OUT_FILE}")
            else:
                print(f"[nmos_fetch] No matching resources yet.")

        # Watch for the Lua bridge signalling a connect request from the GBA.
        if os.path.exists(CONNECT_FILE):
            os.remove(CONNECT_FILE)
            if sender and receiver:
                print(f"[nmos_fetch] GBA requested IS-05 connection!")
                success = connect_is05(sender, receiver)
                print(f"[nmos_fetch] Connection {'succeeded' if success else 'FAILED'}.")
            else:
                print(f"[nmos_fetch] Connect requested but no sender/receiver known yet.")

        time.sleep(0.5)


if __name__ == "__main__":
    main()

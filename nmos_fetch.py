#!/usr/bin/env python3
"""Poll NMOS API for 'presentation' sender/receiver and write to /tmp/nmos_slide.bin.

Used with nmos_bridge.lua (Lua approach). For the GDB approach use nmos_gdb_inject.py instead,
which also handles IS-05 connection requests.

Binary format (/tmp/nmos_slide.bin):
  Byte 0:    ready (1 = valid)
  Bytes 1..32:  sender label  (32 bytes, GBA charmap, 0xFF-terminated)
  Bytes 33..64: receiver label (32 bytes)
  Byte 65:   command placeholder (always 0 — written by GBA, read by bridge)
  Byte 66:   status placeholder  (always 0 — written by bridge)
"""

import time
import urllib.request
import json

NMOS_QUERY  = "http://10.10.30.226:8870/x-nmos/query/v1.3"
SENDER_FILTER   = "presentation"      # sender label must contain this
RECEIVER_FILTER = "presentation" # receiver label must contain this
OUT_FILE    = "/tmp/nmos_slide.bin"
POLL_SEC    = 5


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


def main():
    print(f"[nmos_fetch] Polling {NMOS_QUERY} -> {OUT_FILE}")
    print(f"[nmos_fetch] sender filter:   '{SENDER_FILTER}'")
    print(f"[nmos_fetch] receiver filter: '{RECEIVER_FILTER}'")
    while True:
        senders_all   = fetch_json(f"{NMOS_QUERY}/senders")   or []
        receivers_all = fetch_json(f"{NMOS_QUERY}/receivers") or []
        sender   = find_first(senders_all,   SENDER_FILTER)
        receiver = find_first(receivers_all, RECEIVER_FILTER)

        if sender and receiver:
            s_label = get_label(sender)
            r_label = get_label(receiver)
            print(f"[nmos_fetch] sender:   {s_label!r}")
            print(f"[nmos_fetch] receiver: {r_label!r}")
            buf = bytearray(67)
            buf[0] = 1
            buf[1:33]  = encode_slot(s_label)
            buf[33:65] = encode_slot(r_label)
            # bytes 65-66 are command/status, left at 0
            with open(OUT_FILE, "wb") as f:
                f.write(buf)
            print(f"[nmos_fetch] Wrote {OUT_FILE}")
        else:
            print(f"[nmos_fetch] No matching resources yet.")

        time.sleep(POLL_SEC)


if __name__ == "__main__":
    main()

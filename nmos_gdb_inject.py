#!/usr/bin/env python3
"""Fetch NMOS data and inject it into mGBA via its GDB remote stub.
Also handles IS-05 connection requests written by the GBA game.

Usage:
  1. Launch mGBA with GDB stub:  mgba-qt -g 2345 pokeemerald.gba
  2. Run this script:             python3 nmos_gdb_inject.py

EWRAM layout at 0x02020000:
  +0x00: ready flag (1 = data valid)
  +0x01: sender label   (32 bytes, GBA charmap, 0xFF-terminated)
  +0x21: receiver label (32 bytes, GBA charmap, 0xFF-terminated)
  +0x41: command  (0=none, 1=connect)
  +0x42: status   (0=idle, 1=connecting, 2=connected, 3=failed)
"""

import socket
import time
import urllib.request
import urllib.error
import json

NMOS_QUERY = "http://10.10.30.226:8870/x-nmos/query/v1.3"
NMOS_CONN  = "http://10.10.30.226:8870/x-nmos/connection/v1.1"
SENDER_FILTER   = "presentation"      # sender label must contain this
RECEIVER_FILTER = "Matrox IPMX Video" # receiver label must contain this

GDB_HOST   = "localhost"
GDB_PORT   = 2345
POLL_SEC   = 5

EWRAM_BASE   = 0x02020000
EWRAM_READY  = EWRAM_BASE + 0x00
EWRAM_SENDER = EWRAM_BASE + 0x01   # 32 bytes
EWRAM_RECV   = EWRAM_BASE + 0x21   # 32 bytes
EWRAM_CMD    = EWRAM_BASE + 0x41
EWRAM_STATUS = EWRAM_BASE + 0x42

STATUS_IDLE       = 0
STATUS_CONNECTING = 1
STATUS_CONNECTED  = 2
STATUS_FAILED     = 3


# ---------------------------------------------------------------------------
# GBA charmap encoding (mirrors charmap.rs)
# ---------------------------------------------------------------------------

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


# ---------------------------------------------------------------------------
# GDB Remote Serial Protocol client
# ---------------------------------------------------------------------------

class GDBStub:
    def __init__(self, host: str, port: int):
        self.sock = socket.create_connection((host, port), timeout=5)
        self.sock.settimeout(0.5)
        try:
            self.sock.recv(256)   # discard initial stop packet
        except OSError:
            pass
        self.sock.settimeout(5)

    def _checksum(self, data: str) -> int:
        return sum(ord(c) for c in data) & 0xFF

    def _send_packet(self, cmd: str) -> str:
        pkt = f"${cmd}#{self._checksum(cmd):02x}"
        self.sock.sendall(pkt.encode())
        try:
            self.sock.recv(1)   # ack
        except OSError:
            pass
        buf = b""
        self.sock.settimeout(2)
        try:
            while True:
                chunk = self.sock.recv(4096)
                if not chunk:
                    break
                buf += chunk
                if b'#' in buf and len(buf) >= buf.index(b'#') + 3:
                    break
        except OSError:
            pass
        self.sock.sendall(b"+")
        if b'$' in buf and b'#' in buf:
            return buf[buf.index(b'$') + 1: buf.rindex(b'#')].decode(errors='replace')
        return buf.decode(errors='replace')

    def write_bytes(self, addr: int, data: bytes) -> bool:
        reply = self._send_packet(f"M{addr:x},{len(data):x}:{data.hex()}")
        return reply.strip() == "OK"

    def read_byte(self, addr: int) -> int | None:
        reply = self._send_packet(f"m{addr:x},1")
        try:
            return int(reply.strip(), 16)
        except ValueError:
            return None

    def write_byte(self, addr: int, val: int) -> bool:
        return self.write_bytes(addr, bytes([val]))

    def close(self):
        self.sock.close()


# ---------------------------------------------------------------------------
# NMOS helpers
# ---------------------------------------------------------------------------

def fetch_json(url: str):
    try:
        with urllib.request.urlopen(url, timeout=4) as resp:
            return json.loads(resp.read())
    except Exception as e:
        print(f"[nmos] fetch failed {url}: {e}")
        return None


def get_label(node: dict) -> str:
    return str(node.get("label") or node.get("description") or node.get("id", "")).strip()


def find_first(resources: list, keyword: str) -> dict | None:
    """Return first resource whose label contains keyword (case-insensitive)."""
    kw = keyword.lower()
    for r in resources:
        if kw in get_label(r).lower():
            return r
    return None


def connect_is05(sender: dict, receiver: dict) -> bool:
    """Attempt IS-05 connection: patch receiver staged with sender_id."""
    sender_id   = sender["id"]
    receiver_id = receiver["id"]

    # Fetch sender's active transport params to mirror them to receiver.
    active = fetch_json(f"{NMOS_CONN}/single/senders/{sender_id}/active")
    transport_params = None
    if active and "transport_params" in active:
        transport_params = active["transport_params"]

    body: dict = {
        "sender_id": sender_id,
        "master_enable": True,
        "activation": {"mode": "activate_immediate"},
    }
    if transport_params is not None:
        body["transport_params"] = transport_params

    payload = json.dumps(body).encode()
    url = f"{NMOS_CONN}/single/receivers/{receiver_id}/staged"
    req = urllib.request.Request(
        url,
        data=payload,
        method="PATCH",
        headers={"Content-Type": "application/json"},
    )
    try:
        with urllib.request.urlopen(req, timeout=6) as resp:
            code = resp.getcode()
            print(f"[nmos] IS-05 PATCH {url} -> HTTP {code}")
            return code in (200, 202)
    except urllib.error.HTTPError as e:
        body_txt = e.read().decode(errors='replace')
        print(f"[nmos] IS-05 PATCH failed HTTP {e.code}: {body_txt}")
        return False
    except Exception as e:
        print(f"[nmos] IS-05 PATCH error: {e}")
        return False


# ---------------------------------------------------------------------------
# Main loop
# ---------------------------------------------------------------------------

def main():
    print(f"[nmos] Connecting to mGBA GDB stub at {GDB_HOST}:{GDB_PORT} ...")
    try:
        gdb = GDBStub(GDB_HOST, GDB_PORT)
    except OSError as e:
        print(f"[nmos] Cannot connect: {e}")
        print(f"[nmos] Start mGBA with:  mgba-qt -g {GDB_PORT} pokeemerald.gba")
        return

    print(f"[nmos] Connected. Polling NMOS every {POLL_SEC}s.")
    print(f"[nmos] sender filter:   '{SENDER_FILTER}'")
    print(f"[nmos] receiver filter: '{RECEIVER_FILTER}'")

    sender   = None
    receiver = None
    last_fetch = 0.0

    while True:
        now = time.monotonic()

        # Re-fetch NMOS data periodically.
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
                print(f"[nmos] sender:   {s_label!r}  ({sender['id']})")
                print(f"[nmos] receiver: {r_label!r}  ({receiver['id']})")

                payload = bytearray(1 + 32 + 32 + 2)
                payload[0] = 1
                payload[1:33]  = encode_slot(s_label)
                payload[33:65] = encode_slot(r_label)
                payload[65] = 0   # clear command
                payload[66] = STATUS_IDLE
                ok = gdb.write_bytes(EWRAM_BASE, bytes(payload))
                print(f"[nmos] EWRAM write: {'OK' if ok else 'FAILED'}")
            else:
                missing = []
                if not new_sender:   missing.append("sender")
                if not new_receiver: missing.append("receiver")
                print(f"[nmos] No matching {'/'.join(missing)} found yet.")
                gdb.write_byte(EWRAM_READY, 0)

        # Poll command byte from the GBA game.
        if sender and receiver:
            cmd = gdb.read_byte(EWRAM_CMD)
            if cmd == 1:
                print("[nmos] GBA requested IS-05 connection!")
                gdb.write_byte(EWRAM_CMD,    0)
                gdb.write_byte(EWRAM_STATUS, STATUS_CONNECTING)
                success = connect_is05(sender, receiver)
                gdb.write_byte(EWRAM_STATUS, STATUS_CONNECTED if success else STATUS_FAILED)
                print(f"[nmos] Connection {'succeeded' if success else 'FAILED'}.")

        time.sleep(0.5)


if __name__ == "__main__":
    main()

-- nmos_bridge.lua  (Lua alternative to nmos_gdb_inject.py)
-- Load in mGBA via Tools > Scripting, open this file.
-- Run nmos_fetch.py separately so /tmp/nmos_slide.bin is kept up to date.
--
-- EWRAM layout at 0x02020000 (matches Rust + nmos_gdb_inject.py):
--   +0x00: ready flag (1 = data valid)
--   +0x01: sender label   (32 bytes, GBA charmap)
--   +0x21: receiver label (32 bytes, GBA charmap)
--   +0x41: command  (0=none, 1=connect)  -- written by GBA game
--   +0x42: status   (0=idle, 1=connecting, 2=connected, 3=failed)

local BIN_FILE   = "/tmp/nmos_slide.bin"
local EWRAM_BASE = 0x02020000
local PAYLOAD_BYTES = 1 + 32 + 32 + 2   -- 67 bytes

local frame_count = 0

local function read_file(path)
    local f = io.open(path, "rb")
    if not f then return nil end
    local data = f:read("*all")
    f:close()
    return data
end

local function update()
    local ok, err = pcall(function()
        local data = read_file(BIN_FILE)
        if data == nil then
            emu:write8(EWRAM_BASE, 0)
            return
        end
        if #data < PAYLOAD_BYTES then
            console:log("[nmos_bridge] File too short: " .. #data)
            return
        end
        for i = 1, PAYLOAD_BYTES do
            emu:write8(EWRAM_BASE + i - 1, string.byte(data, i))
        end
        console:log("[nmos_bridge] Injected (ready=" .. string.byte(data, 1) .. ")")
    end)
    if not ok then
        console:log("[nmos_bridge] update error: " .. tostring(err))
    end
end

callbacks:add("frame", function()
    frame_count = frame_count + 1
    if frame_count % 60 == 0 then
        update()
    end
end)

console:log("[nmos_bridge] Loaded. Watching " .. BIN_FILE)
console:log("[nmos_bridge] Note: IS-05 connection is handled by nmos_fetch.py, not this bridge.")

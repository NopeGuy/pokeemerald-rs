-- nmos_bridge.lua  (Lua alternative to nmos_gdb_inject.py)
-- Load in mGBA via Tools > Scripting, open this file.
-- Run nmos_fetch.py separately so /tmp/nmos_slide.bin is kept up to date.
--
-- EWRAM layout at 0x02020000 (matches Rust + nmos_gdb_inject.py):
--   +0x00: ready flag (1 = data valid)
--   +0x01: sender label   (32 bytes, GBA charmap)
--   +0x21: receiver label (32 bytes, GBA charmap)
--   +0x41: command  (0=none, 1=connect)  -- written by GBA game, read here
--   +0x42: status   (0=idle, 1=connecting, 2=connected, 3=failed)

local BIN_FILE     = "/tmp/nmos_slide.bin"
local CONNECT_FILE = "/tmp/nmos_connect"   -- written here, read by nmos_fetch.py
local EWRAM_BASE   = 0x02020000
local EWRAM_CMD    = EWRAM_BASE + 0x41
local LABEL_BYTES  = 65   -- ready(1) + sender(32) + receiver(32); stop before CMD/status

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
        -- Inject ready flag + labels only (bytes 0-64).
        -- Deliberately skip bytes 65-66 (CMD/status) so the GBA-written
        -- command byte is not overwritten by the file contents each tick.
        local data = read_file(BIN_FILE)
        if data == nil then
            emu:write8(EWRAM_BASE, 0)
            return
        end
        if #data < LABEL_BYTES then
            console:log("[nmos_bridge] File too short: " .. #data)
            return
        end
        for i = 1, LABEL_BYTES do
            emu:write8(EWRAM_BASE + i - 1, string.byte(data, i))
        end
        console:log("[nmos_bridge] Injected (ready=" .. string.byte(data, 1) .. ")")

        -- Check if the GBA game has requested an IS-05 connection.
        local cmd = emu:read8(EWRAM_CMD)
        if cmd == 1 then
            emu:write8(EWRAM_CMD, 0)   -- acknowledge so it doesn't fire again
            local f = io.open(CONNECT_FILE, "w")
            if f then f:write("1") f:close() end
            console:log("[nmos_bridge] Connect signal -> " .. CONNECT_FILE)
        end
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

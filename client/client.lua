local url, clientName = ...
local terms = { term.native(), peripheral.find 'monitor' }

for _, term in ipairs(terms) do
  if term.setTextScale then term.setTextScale(0.5) end
  term.height = select(2, term.getSize())
end

local function log(packet)
  for _, term in ipairs(terms) do
    term.scroll(1)
    term.setCursorPos(1, term.height)
    term.setTextColor(bit.blshift(1, packet.c))
    term.write(packet.t)
  end
end

local enc (function()
  local m = {
    ['nil'] = function() return '!' end,
    number = function(x) return string.format('#%.17g@', x) end,
    string = function(x) return '@' .. string.gsub(x, '@', '@.') .. '@~' end,
    boolean = function(x) if x then return '+' else return '-' end end,
    table = function(x)
      local r = '='
      for k, v in pairs(x) do r = r .. enc(k) .. enc(v) end
      return r .. '!'
    end
  }
  function enc(x)
    return m[type(x)](x)
  end
end)()

local function dec(h)
  local s, tab
  local function num()
    local p, d = s, ''
    function s(x)
      local i = string.find(x, '@')
      if i then s = p s(tonumber(d .. string.sub(x, 1, i - 1)), string.sub(x, i + 1))
      else d = d .. x end
    end
  end
  local function str()
    local p, d, e = s, '', false
    function s(x)
      for i = 1, #x do
        local c = string.sub(x, i, i)
        if e then
          if c == '.' then d = d .. '@' e = false
          elseif c == '~' then s = p return s(d, string.sub(x, i + 1))
          else error('unknown escape: ' .. c) end
        elseif c == '@' then e = true
        else d = d .. c end
      end
    end
  end
  local function root()
    local p = s
    function s(x)
      if #x > 0 then
        s = p
        local t, r = string.sub(x, 1, 1), string.sub(x, 2)
        if t == '!' then s(nil, r)
        elseif t == '#' then num() s(r)
        elseif t == '@' then str() s(r)
        elseif t == '+' then s(true, r)
        elseif t == '-' then s(false, r)
        elseif t == '=' then tab() s(r)
        else error('invalid tag: ' .. t) end
      end
    end
  end
  function tab()
    local p, d, k = s, {}
    function s(x, r)
      if k == nil then
        if x == nil then s = p s(d, r)
        else k = x root() s(r) end
      else d[k] = x k = nil root() s(r) end
    end
    root()
  end
  function s(x, r) h(x) root() s(r) end
  root()
  return function(x) s(x) end
end

local function callRS(p, f, ...)
  if p then return peripheral.call(p, f, ...)
  else return rs[f](...) end
end

local function exec(p, r)
  if p.o == 'l' then log(p)
  elseif p.o == 'c' then r.r = { peripheral.call(p.p, table.unpack(p.v)) }
  elseif p.o == 'i' then
    if p.b then
      if bit.band(callRS(p.p, 'getBundledInput', p.s), p.b) ~= 0 then r.r = 15 else r.r = 0 end
    else r.r = callRS(p.p, 'getAnalogInput', p.s) end
  elseif p.o == 'o' then
    if p.b then
      local v = callRS(p.p, 'getBundledOutput', p.s)
      if p.v ~= 0 then v = bit.bor(v, p.b) else v = bit.band(v, bit.bnot(p.b)) end
      callRS(p.p, 'setBundledOutput', p.s, v)
    else callRS(p.p, 'setAnalogOutput', p.s, p.v) end
  elseif p.o == 't' then r.r = { turtle[p.f](table.unpack(p.v)) }
  else error('invalid op: ' .. tostring(p.o)) end
  return 0
end

while true do
  local tid = os.startTimer(3)
  local url, socket = url .. '#' .. tid
  log { t = 'Connecting to ' .. clientName .. '@' .. url, c = 4 }
  http.websocketAsync(url)
  while true do
    local e = { os.pullEvent() }
    if e[1] == 'timer' then
      if e[2] == tid then log { t = 'Timed out', c = 14 } break end
    elseif e[1] == 'websocket_failure' then
      if e[2] == url then
        log { t = e[3], c = 14 }
        while e[1] ~= 'timer' or e[2] ~= tid do e = { os.pullEvent() } end
        break
      end
    elseif e[1] == 'websocket_success' then
      if e[2] == url then socket = e[3] break end
    end
  end
  if socket then
    log { t = 'Connected', c = 13 }
    local out, tasks = enc(clientName), {}
    local handler = dec(function(p)
      for _, p in ipairs(p) do
        local task = coroutine.create(exec)
        local r = { i = p.i }
        local e, d = coroutine.resume(task, p, r)
        if not e then
          r.r = nil
          r.e = d
          out = out .. enc(r)
        elseif type(d) == 'number' then out = out .. enc(r)
        else tasks[#tasks + 1] = { task = task, filter = d, r = r } end
      end
    end)
    while true do
      local e = true
      while #out > 0 do
        local n = math.min(#out, 65536)
        local d = out:sub(1, n)
        e, d = pcall(socket.send, d, true)
        if not e then log { t = d, c = 14 } break end
        out = out:sub(n + 1)
      end
      if not e then break end
      e = { os.pullEvent() }
      if e[1] == 'timer' then
        if e[2] == tid then tid = nil end
      elseif e[1] == 'websocket_closed' then
        if e[2] == url then log { t = 'Connection closed', c = 14 } break end
      elseif e[1] == 'websocket_message' then
        if e[2] == url then handler(e[3]) end
      end
      local newTasks = {}
      for _, v in ipairs(tasks) do
        if not v.filter or v.filter == e[1] then
          local e, d = coroutine.resume(v.task, table.unpack(e))
          if not e then
            v.r.r = nil
            v.r.e = d
            out = out .. enc(v.r)
          elseif type(d) == 'number' then out = out .. enc(v.r)
          else newTasks[#newTasks + 1] = { task = v.task, filter = d, r = v.r } end
        else newTasks[#newTasks + 1] = v end
      end
      tasks = newTasks
    end
    if tid then repeat local e = { os.pullEvent() } until e[1] ~= 'timer' or e[2] ~= tid end
  end
end

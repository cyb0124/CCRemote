local url, clientName = ...
local terms = {term.native(), peripheral.find 'monitor'}

for _, term in ipairs(terms) do
  if term.setTextScale then term.setTextScale(0.5) end
  term.height = select(2, term.getSize())
end

local function log(packet)
  for _, term in ipairs(terms) do
    term.scroll(1)
    term.setCursorPos(1, term.height)
    term.setTextColor(packet.color)
    term.write(packet.text)
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

while true do
  local id, socket = os.startTimer(3)
  local url = url .. '#' .. id
  log { text = 'Connecting to ' .. clientName .. '@' .. url, color = colors.yellow }
  http.websocketAsync(url)
  while true do
    local e, i, d = os.pullEvent()
    if e == 'timer' then
      if i == id then log { text = 'Timed out', color = colors.red } break end
    elseif e == 'websocket_failure' then
      if i == url then
        log { text = d, color = colors.red }
        while e ~= 'timer' or i ~= id do e, i = os.pullEvent() end
        break
      end
    elseif e == 'websocket_success' then
      if i == url then socket = d break end
    end
  end
  if socket then
    os.cancelTimer(id)
    log { text = "Connected", color = colors.green }
    local h, q = dec(function(p)
      for _, p in ipairs(p) do
        local r if p.op == 'log' then log(p)
        elseif p.op == 'call' then r = {peripheral.call(p.p, table.unpack(p.v))}
        else error('invalid op: ' .. tostring(p.op)) end
        q = q .. enc(r)
      end
    end), enc(clientName)
    while true do
      if #q > 0 then
        r, d = pcall(socket.send, q, true)
        if not r then log { text = d, color = colors.red } break end
        q = ''
      end
      r, d = pcall(socket.receive)
      if not r then log { text = d, color = colors.red } break end
      h(d)
    end
  end
end

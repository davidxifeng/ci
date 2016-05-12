--- module vm
-- Thu 19:11 May 12

local function lc_load(fn)
  local f = io.open(fn, 'rb')
  if not f then return nil end
  local hd = assert(f:read(12))
  local fmt = '<i4i4i4'
  local main_addr, t_sz, d_sz = fmt:unpack(hd)
  local text = {}
  local r = { main = main_addr, text = text }
  for i = 1, t_sz // 4 do
    local code = ('<i4'):unpack(f:read(4))
    table.insert(text, code)
  end
  r.data = f:read(d_sz)
  f:close()
  return r
end

local function lc_run()
end

local function lc_dump(p)
  for i = 1, #p.text do
    print(p.text[i])
  end
  print(p.main, p.data)
end

local function main()
  local fn = arg[1]
  if fn then
    lc_dump(lc_load(fn))
  end
end

return main()

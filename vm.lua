#!/usr/bin/env lua

local LEA = 0
local IMM = 1
local JMP = 2
local JSR = 3
local BZ = 4
local BNZ = 5
local ENT = 6
local ADJ = 7
local LGB = 8
local LEV = 9

local LI = 10
local LC = 11
local SI = 12
local SC = 13
local PSH = 14

local OR = 15
local XOR = 16
local AND = 17
local EQ = 18
local NE = 19
local LT = 20
local GT = 21
local LE = 22
local GE = 23

local SHL = 24
local SHR = 25
local ADD = 26
local SUB = 27
local MUL = 28
local DIV = 29
local MOD = 30

local OPEN = 31
local READ = 32
local CLOS = 33
local PRTF = 34
local MALC = 35
local MSET = 36
local MCMP = 37
local EXIT = 38

--- module vm
-- Thu 19:11 May 12

-- {
--  main
--  text {}
--  data ''
-- }
local function lc_load(fn)
    local f = io.open(fn, 'rb')
    if not f then return nil end

    local hd = assert(f:read(12))

    local main_addr, t_sz, d_sz = ('<i4i4i4'):unpack(hd)

    local text = {}
    local r = {main = main_addr, text = text}

    for i = 1, t_sz // 4 do
        local code = ('<i4'):unpack(f:read(4))
        table.insert(text, code)
    end
    r.data = f:read(d_sz)
    f:close()
    return r
end


-- 封装栈的操作
local function new_stack()
    return {
        sp = 0,
        push = function(self, value)
            self.sp = self.sp + 1
            table.insert(self, value)
        end,
        pop = function(self)
            self.sp = self.sp - 1
            return table.remove(self)
        end,
        get_sp = function(self) return self.sp end,
        set_sp = function(self, value) self.sp = value end
    }
end

-- 封装 PC寄存器的操作？ 为何叫text?
local function new_text(p)
    return {
        data = p.text,
        pc = p.main,
        get_pc_inc = function(self)
            local i = self.pc
            self.pc = i + 1
            return self.data[i]
        end,
        current_pc = function(self) return self.pc end,
        current_pc_value = function(self) return self.data[self.pc] end,
        add_pc_by = function(self, value) self.pc = self.pc + value end,
        set_pc = function(self, value) self.pc = value end
    }
end

local function lc_run(p)
    assert(p.main and p.text and p.data)
    local data = p.data

    local sp = new_stack() -- 与C版不同，栈的增长方向向上
    sp:push(EXIT)
    sp:push(PSH)
    local t = sp:get_sp()

    -- TODO
    local argc, argv = 0, 0

    sp:push(argc)
    sp:push(argv)
    sp:push(t)

    local cycle = 0
    local bp = 'NULL'
    local pc = new_text(p)
    local a = 0
    while true do
        local i = pc:get_pc_inc()
        cycle = cycle + 1
        if true then print(pc:current_pc(), i, cycle) end

        if cycle > 100 then break end

        if i == 0 then -- LEA
            a = bp + pc:get_pc_inc()
        elseif i == 1 then -- IMM
            a = pc:get_pc_inc()
        elseif i == 2 then -- JMP
            pc:add_pc_by(pc:current_pc_value())
        elseif i == 3 then -- JSR
            sp:push(pc:current_pc() + 1)
            pc:set_pc(pc:current_pc_value())
        elseif i == 4 then -- BZ
            if a ~= 0 then
                pc:set_pc(pc:current_pc() + 1)
            else
                pc:set_pc(pc:current_pc() + pc:current_pc_value())
            end
        elseif i == 5 then -- BNZ
            if a ~= 0 then
                pc:set_pc(pc:current_pc() + pc:current_pc_value())
            else
                pc:set_pc(pc:current_pc() + 1)
            end
        elseif i == 6 then -- ENT
            sp:push(bp)
            bp = sp:get_sp()
            sp:set_sp(sp:get_sp() - pc:get_pc_inc())
        elseif i == 7 then -- ADJ
            sp:set_sp(sp:get_sp() + pc:get_pc_inc())
        elseif i == 8 then -- LGB
            a = pc:get_pc_inc()
        elseif i == 9 then -- LEV

        elseif i == 10 then -- LI
            -- Lua中怎么实现寻址这样的操作呢
            -- Sun 16:09 May 15
        elseif i == 11 then -- LC
        elseif i == 12 then -- SI
        elseif i == 13 then -- SC
        elseif i == 14 then -- PSH

        elseif i == 15 then -- OR
        elseif i == 16 then -- XOR
        elseif i == 17 then -- AND
        elseif i == 18 then -- EQ
        elseif i == 19 then -- NE
        elseif i == 20 then -- LT
        elseif i == 21 then -- GT
        elseif i == 22 then -- LE
        elseif i == 23 then -- GE

        elseif i == 24 then -- SHL
        elseif i == 25 then -- SHR
        elseif i == 26 then -- ADD
        elseif i == 27 then -- SUB
        elseif i == 28 then -- MUL
        elseif i == 29 then -- DIV
        elseif i == 30 then -- MOD

        elseif i == 31 then -- OPEN
        elseif i == 32 then -- READ
        elseif i == 33 then -- CLOS
        elseif i == 34 then -- PRTF
        elseif i == 35 then -- MALC
        elseif i == 36 then -- MSET
        elseif i == 37 then -- MCMP
        elseif i == 38 then -- EXIT
        end
    end
end

local function lc_dump(p)
    for i = 1, #p.text do print(p.text[i]) end
    print(p.main, p.data)
end

local function main()
    local fn = arg[1]
    if fn then
        local p = lc_load(fn)
        -- lc_dump(p)
        lc_run(p)
    end
end

return main()

local function pop_n(tbl: {any}, n: integer): {any}
	local len = #tbl
	assert(len >= n)
	for i = len - n + 1, len do
		tbl[i] = nil
	end
	return tbl
end

local function run()
	local data : {string} = {
		'hello',
		'world',
	}
	local codes: { {string, integer}} = {
		{'push', 1},
		{'prints', 1},
		{'pop', 1},

		{'push', 1},
		{'push', 2},
		{'add'},
		{'print', 1},

		{'push', 3},
		{'add'},
		{'print', 1},
		{'pop', 1},

		{'push', 1},
		{'push', 2},
		{'prints', 2},
		{'pop', 2},

		{'halt'},
	}

	local stack : {integer}= {}
	local pc = 0
	while true do
		pc = pc + 1
		local t = codes[pc]
		local opcode, operand = t[1], t[2]
		if opcode == 'push' then
			stack[#stack+1] = operand
		elseif opcode == 'pop' then
			pop_n(stack, operand)
		elseif opcode == 'add' then
			stack[#stack - 1] = stack[#stack] + stack[#stack - 1]
			stack[#stack] = nil
		elseif opcode == 'sub' then
			stack[#stack - 1] = stack[#stack] - stack[#stack - 1]
			stack[#stack] = nil
		elseif opcode == 'mul' then
			stack[#stack - 1] = stack[#stack] * stack[#stack - 1]
			stack[#stack] = nil
		elseif opcode == 'div' then
			stack[#stack - 1] = stack[#stack] // stack[#stack - 1]
			stack[#stack] = nil
		elseif opcode == 'print' then
			print(table.unpack(stack, #stack - operand + 1))
		elseif opcode == 'prints' then
			local pv = {}
			for i = #stack - operand + 1, #stack do
				pv[#pv+1] = data[stack[i]]
			end
			print(table.unpack(pv))
		elseif opcode == 'halt' then
			break
		end
	end
	assert(#stack == 0, 'stack should be balanced')
end

run()
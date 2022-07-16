local function pop_n(tbl: {any}, n: integer): {any}
	local len = #tbl
	assert(len >= n)
	for i = len - n + 1, len do
		tbl[i] = nil
	end
	return tbl
end

local type Instruction = enum
	'push'
	'pop'

	'halt'

	'print'
	'prints'
	'input'

	'add'
	'sub'
	'mul'
	'div'
end

local data : {string} = {
	'hello',
	'world',
	'please input a number: ',
	'sum of your input is: ',
}

local codes: { {Instruction, integer}} = {
	{'push', 1},
	{'push', 2},
	{'prints', 2},
	{'pop', 2},

	{'push', 3},
	{'prints', 1},
	{'pop', 1},

	{'input', },

	{'push', 3},
	{'prints', 1},
	{'pop', 1},

	{'input', },

	{'push', 4},
	{'prints', 1},
	{'pop', 1},

	{'add'},
	{'print', 1},
	{'pop', 1},

	{'halt'},
}

local function run()


	local stack : {integer} = {}
	local cycle, pc = 0, 0
	while true do
		pc, cycle = pc + 1, cycle + 1

		local instruction = codes[pc]
		local opcode, operand = instruction[1], instruction[2]
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
		elseif opcode == 'input' then
			-- 没有找到忽略错误的选项,但有as就可以愉快地play with teal了
			local iv = io.read('n') as integer
			if type(iv) == 'number' then
				stack[#stack + 1] = iv
			else
				stack[#stack + 1] = 0
			end
		elseif opcode == 'print' then
			print(table.unpack(stack, #stack - operand + 1))
		elseif opcode == 'prints' then
			local pv = {}
			for i = #stack - operand + 1, #stack do
				pv[#pv+1] = data[stack[i]]
			end
			print(table.unpack(pv))
		elseif opcode == 'halt' then
			print(("halt. cycle = %d"):format(cycle));
			break
		end
	end
	assert(#stack == 0, 'stack should be balanced')
end

run()
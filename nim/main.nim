# 原来不需要手动导入
# from system/io import readFile
# This is a comment

# echo "What's your name? "
# var name: string = readLine(stdin)
# echo "Hi, ", name, "!"

# const x = "this is a const str, "
# const
  # y = 2
  # z = 3 + y

proc main(file="../rust/data/t0.c", args: seq[string]): int=
  let fc = readFile file
  echo "file content is: ", fc

import cligen; dispatch main

# watch folder自动执行:
# cargo watch --workdir . -s "nim r main.nim"

# 没用内置的sum type, 先放弃了
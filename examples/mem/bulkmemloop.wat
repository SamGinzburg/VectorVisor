(module
  (memory (export "memory") 48)
  (func $_start (result)
  	i32.const 1048576
    i32.const 0
	i32.const 1024
	i32.const 1024
    i32.mul
    memory.copy
  )
  (export "_start" (func $_start))
)

(module
  (memory (export "memory") 1)

  (func $_start (result i32)
    (local i32)
    i32.const 1
    block (result i32)
    	i32.const 100
    	i32.const 0
    	br_if 0
	drop
	i32.const 200
    end
    i32.add
  )
  (export "_start" (func $_start))
)

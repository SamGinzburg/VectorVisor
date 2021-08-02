(module
  (memory (export "memory") 1)

  (func $_start (result i32)
    (local i32)
    i32.const 1
    block (result i32)
    	i32.const 999
    end
    i32.add
  )
  (export "_start" (func $_start))
)

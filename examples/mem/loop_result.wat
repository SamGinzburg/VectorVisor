(module
  (memory (export "memory") 1)

  (func $_start (result i32)
    (local i32)
    i32.const 100
    local.set 0
    block (result i32) ;; 1
    loop (result i32) ;; 0
      local.get 0
      local.get 0
      i32.const 20 ;; stop value, func should return this val
      i32.eq
      br_if 1
      local.get 0
      i32.const 1
      i32.sub
      local.set 0
      br 0
      i32.const 0
    end
    end
  )
  (export "_start" (func $_start))
)

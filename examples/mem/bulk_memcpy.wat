(module
  (memory (export "memory") 1)
  (data $d0 (i32.const 0) "\10\20\30\40")
  (func $_start (result i32)
    (i32.const 200) ;; d
    (i32.const 0) ;; s
    (i32.const 128) ;; n
    (memory.copy)
    (i32.const 2)
    (i32.load align=1)
  )
  (export "_start" (func $_start))
)

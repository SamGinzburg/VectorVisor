(module
  (memory (export "memory") 1)
  ;;(data $d0 (i32.const 0) "\10\20\30\40")
  (func $_start (result i32)
    (i32.const 11)
    (i32.const 42)
    (i32.const 128)
    (memory.fill)
    (i32.const 19)
    (i32.load align=1)
  )
  (export "_start" (func $_start))
)

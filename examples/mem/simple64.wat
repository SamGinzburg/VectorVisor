(module
  (memory (export "memory") 1)
  (data $d0 (i32.const 5) "\10\20\30\40")
  (func $_start (result i64)
    (i32.const 7)
    (i64.const 0xAABBCCDDEEFF1122)
    (i64.store align=1)
    (i32.const 7)
    (i64.load align=1)
  )
  (export "_start" (func $_start))
)

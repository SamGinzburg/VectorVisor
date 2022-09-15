(module
  (memory (export "memory") 1)

  (func $_start (result v128)
    (local $l2 v128)
    (i32.const 4)
    (v128.load64_zero)
  )
  (export "_start" (func $_start))
  (data $d0 (i32.const 8) "\10\00\00\00\0d\00\00\00")
)

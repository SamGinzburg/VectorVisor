(module
  (memory (export "memory") 1)

  (func $_start (result v128)
    (local $l2 v128)
    (i32.const 0)
    (v128.const i32x4 0x00000001 0x00000002 0x00000003 0x00000004)
    (v128.store)
    (i32.const 0)
    (v128.load)
  )
  (export "_start" (func $_start))
  (data $d0 (i32.const 8) "\10\00\00\00\0d\00\00\00")
)

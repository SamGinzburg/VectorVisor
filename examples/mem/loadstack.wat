(module
  (memory (export "memory") 1)
  (func $_start (result i32)
    (local $l1 i32)
    (i32.const 5) ;; dummy item on stack for test
    (i32.const 5)
    (i32.const 100)
    (i32.store)
    (i32.const 5)
    (i32.load)
    (local.set $l1)
    (local.set $l1)
    (local.get $l1)
  )
  (export "_start" (func $_start))
)
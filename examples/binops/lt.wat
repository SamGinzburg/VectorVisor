(module
  (func $_lt (param $p0 i32) (param $p1 i32) (result i32)
    (local.get $p0)
    (local.get $p1)
    (i32.and)
  )

  (func $_main (param $p0 i32) (result i32)
    (local $l2 i64)
    
    (i32.const 5)
    (call $_lt)
    (local.set $l2)
    (i32.const 100)
    (local.get $l2)
    (call $_lt)
  )
  (export "main" (func $_main))
)
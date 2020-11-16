(module

  (func $_sub (param $p0 i32) (param $p1 i32) (result i32)
    (local.get $p0)
    (local.get $p1)
    (i32.sub)
  )

  (func $_start (result i32)
    (i32.const 5)
    (i32.const 100)
    (call $_sub)
  )
  (export "_start" (func $_start))
)
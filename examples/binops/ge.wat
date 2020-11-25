(module
  (func $_ge (param $p0 i32) (param $p1 i32) (result i32)
    (local.get $p0)
    (local.get $p1)
    (i32.ge_s)
  )

  (func $_start (result i32)
    (i32.const 101)
    (i32.const 100)
    (call $_ge)
  )
  (export "_start" (func $_start))
)
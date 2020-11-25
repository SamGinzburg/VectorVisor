(module
  (func $_or (param $p0 i32) (param $p1 i32) (result i32)
    (local.get $p0)
    (local.get $p1)
    (i32.or)
  )

  (func $_start (result i32)
    (i32.const 1)
    (i32.const 1)
    (call $_or)
  )
  (export "_start" (func $_start))
)
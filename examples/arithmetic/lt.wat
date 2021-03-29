(module
  (type $t1 (func (param i32 i32) (result i32)))

  (func $_lt (param $p0 i32) (param $p1 i32) (result i32)
    (local.get $p0)
    (local.get $p1)
    (i32.div_s)
  )

  (func $_start (result i32)
    (local $l2 i32)
    (i32.const 10)
    (i32.const 5)
    (call $_lt)
  )
  (export "_start" (func $_start))
)